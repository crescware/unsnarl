//! `Annotations::of_variable(v).is_unused` source of truth.
//!
//! A variable is considered unused when no read reference originates
//! from outside one of the variable's own defining bodies. Writes
//! (the init Write plus any later re-assignments) and self-internal
//! reads (the recursive call inside `function foo() { foo(); }` or
//! `const a = () => a;`) do not count as usage.
//!
//! "Body nodes" are collected from each def. The materialised
//! [`AstNode`] does not carry the `init` slot, so that lookup is
//! factored out into a caller-supplied `body_span_lookup` closure:
//! given a [`DefinitionId`], it returns the span of the body that
//! should establish a body scope (or `None` when the def has none).
//! The pipeline populates this closure from the underlying AST at
//! boundary build time; tests construct it inline.

use oxc_span::Span;

use unsnarl_ir::ids::{DefinitionId, ScopeId, VariableId};
use unsnarl_ir::reference::reference_flags::ReferenceFlags;
use unsnarl_ir::DefinitionType;
use unsnarl_ir::IrArena;
use unsnarl_oxc_parity::AstType;

pub fn is_unused<F>(variable: VariableId, arena: &IrArena, body_span_lookup: F) -> bool
where
    F: Fn(DefinitionId) -> Option<Span>,
{
    let body_scopes = collect_body_scopes(variable, arena, &body_span_lookup);
    let var = &arena.variables[variable];
    for &ref_id in &var.references {
        let r = &arena.references[ref_id];
        if (r.flags & ReferenceFlags::READ).0 == 0 {
            continue;
        }
        if !is_from_inside(arena, r.from, &body_scopes) {
            return false;
        }
    }
    true
}

fn collect_body_scopes<F>(
    variable: VariableId,
    arena: &IrArena,
    body_span_lookup: &F,
) -> Vec<ScopeId>
where
    F: Fn(DefinitionId) -> Option<Span>,
{
    let var = &arena.variables[variable];
    let mut body_spans: Vec<Span> = Vec::new();
    for &def_id in &var.defs {
        let def = &arena.definitions[def_id];
        let body_span = body_span_from_def(def, body_span_lookup, def_id);
        if let Some(span) = body_span {
            if !body_spans
                .iter()
                .any(|existing| spans_match(existing, &span))
            {
                body_spans.push(span);
            }
        }
    }
    if body_spans.is_empty() {
        return Vec::new();
    }
    let mut result: Vec<ScopeId> = Vec::new();
    // The inner ClassName for `class C { ... }` is declared inside the
    // class scope itself, so the defining body is `variable.scope`
    // rather than a child of it. Restrict to ClassName defs whose
    // body matches the variable's own scope block.
    if is_inner_class_name(arena, variable) {
        result.push(var.scope);
    }
    for &child in &arena.scopes[var.scope].child_scopes {
        let child_span = arena.scopes[child].block.span;
        if body_spans.iter().any(|s| spans_match(s, &child_span)) && !result.contains(&child) {
            result.push(child);
        }
    }
    result
}

fn body_span_from_def<F>(
    def: &unsnarl_ir::scope::DefinitionData,
    body_span_lookup: &F,
    def_id: DefinitionId,
) -> Option<Span>
where
    F: Fn(DefinitionId) -> Option<Span>,
{
    if is_functionlike(&def.node.r#type) {
        return Some(def.node.span);
    }
    if matches!(def.node.r#type, AstType::VariableDeclarator) {
        return body_span_lookup(def_id);
    }
    None
}

fn is_functionlike(ty: &AstType) -> bool {
    matches!(
        ty,
        AstType::FunctionDeclaration
            | AstType::FunctionExpression
            | AstType::ArrowFunctionExpression
            | AstType::ClassDeclaration
            | AstType::ClassExpression
    )
}

fn is_inner_class_name(arena: &IrArena, variable: VariableId) -> bool {
    let var = &arena.variables[variable];
    let scope_block_span = arena.scopes[var.scope].block.span;
    var.defs.iter().any(|&id| {
        let def = &arena.definitions[id];
        matches!(def.r#type, DefinitionType::ClassName)
            && spans_match(&def.node.span, &scope_block_span)
    })
}

fn is_from_inside(arena: &IrArena, from: ScopeId, body_scopes: &[ScopeId]) -> bool {
    if body_scopes.is_empty() {
        return false;
    }
    let mut cur = Some(from);
    while let Some(s) = cur {
        if body_scopes.contains(&s) {
            return true;
        }
        cur = arena.scopes[s].upper;
    }
    false
}

fn spans_match(a: &Span, b: &Span) -> bool {
    a.start == b.start && a.end == b.end
}

#[cfg(test)]
#[path = "is_unused_test.rs"]
mod is_unused_test;
