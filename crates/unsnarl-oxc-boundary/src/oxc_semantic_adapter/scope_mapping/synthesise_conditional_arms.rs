//! Synthesise per-arm `Block` scopes for every `ConditionalExpression`.
//!
//! `oxc_semantic` allocates no scope for a ternary `cond ? a : b` — the
//! arms share the expression's enclosing scope. To render each arm as
//! its own branch subgraph (mirroring `if` / `else` Block scopes and the
//! per-`SwitchCase` synthesis in [`super::build_scopes`]), this pass
//! appends one `Block` scope per arm, anchored to the arm expression's
//! span, and re-parents:
//!
//! * each arm to the innermost arm that encloses it, so nested ternaries
//!   (`a ? b : c ? d : e`) nest — falling back to the expression's
//!   enclosing scope; and
//! * any pre-existing scope that falls inside an arm (e.g. a callback
//!   arrow in `cond ? xs.map(f) : xs`) to that arm, so it renders within
//!   the branch.
//!
//! Only the *structure* is synthesised here — the arm subgraph frames and
//! the nesting of arm-local scopes (callbacks, nested ternaries). The arm
//! *values* are not relocated: a ternary is an expression whose single
//! value flows to its consumer (binding / return / call) the same way any
//! expression's value does, so the value reads keep their natural target.
//! Only the test reads are redirected — to the diamond, via the
//! `predicate_container` annotation, not here.

use oxc_ast::AstKind;
use oxc_index::IndexVec;
use oxc_semantic::Semantic;
use oxc_span::Span;
use oxc_syntax::scope::ScopeId as OxcScopeId;

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope::ScopeData;
use unsnarl_ir::scope_type::ScopeType;

use crate::materialise::ast_node_of_expression;

/// One ternary's arms, captured before `scopes` is mutated so the
/// immutable `semantic` borrow does not overlap the push loop.
struct ArmSpec {
    enclosing: ScopeId,
    cond_start: u32,
    consequent: AstNode,
    alternate: AstNode,
}

/// Append two `Block` scopes (consequent, alternate) per
/// `ConditionalExpression`, nesting arms and pulling arm-local scopes in.
/// See the module docs for the re-parenting performed.
pub(super) fn synthesise_conditional_arms(
    semantic: &Semantic<'_>,
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    translation: &IndexVec<OxcScopeId, Option<ScopeId>>,
) {
    let mut specs: Vec<ArmSpec> = Vec::new();
    for node in semantic.nodes().iter() {
        let AstKind::ConditionalExpression(cond) = node.kind() else {
            continue;
        };
        let Some(enclosing) = translation[node.scope_id()] else {
            continue;
        };
        specs.push(ArmSpec {
            enclosing,
            cond_start: cond.span.start,
            consequent: ast_node_of_expression(&cond.consequent),
            alternate: ast_node_of_expression(&cond.alternate),
        });
    }
    // Source order keeps the synthetic arm IR ids — and thus the
    // sibling / child-scope order in the rendered graph — deterministic.
    specs.sort_by_key(|s| s.cond_start);

    let first_arm = ScopeId::from_usize(scopes.len());
    let mut arms: Vec<(Span, ScopeId)> = Vec::with_capacity(specs.len() * 2);
    for spec in &specs {
        let is_strict = scopes[spec.enclosing].is_strict;
        let variable_scope = scopes[spec.enclosing].variable_scope;
        for arm in [&spec.consequent, &spec.alternate] {
            let arm_ir = ScopeId::from_usize(scopes.len());
            scopes.push(ScopeData::new(
                ScopeType::Block,
                is_strict,
                Some(spec.enclosing),
                Vec::new(),
                variable_scope,
                arm.clone(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                false,
            ));
            arms.push((arm.span, arm_ir));
        }
    }

    // Nest each arm inside the innermost arm that encloses it.
    for &(span, arm_ir) in &arms {
        if let Some(parent) = innermost_enclosing_arm(&arms, span, Some(arm_ir)) {
            scopes[arm_ir].upper = Some(parent);
        }
    }

    // Pull any pre-existing scope sitting inside an arm into that arm
    // (e.g. a callback arrow body in `cond ? xs.map(f) : xs`).
    for raw in 1..first_arm.index() {
        let s = ScopeId::from_usize(raw);
        let span = scopes[s].block.span;
        let Some(upper) = scopes[s].upper else {
            continue;
        };
        let upper_span = scopes[upper].block.span;
        if let Some(arm) = innermost_arm_between(&arms, span, upper_span) {
            scopes[s].upper = Some(arm);
        }
    }
}

/// Innermost arm whose span strictly contains `span`, skipping `exclude`.
fn innermost_enclosing_arm(
    arms: &[(Span, ScopeId)],
    span: Span,
    exclude: Option<ScopeId>,
) -> Option<ScopeId> {
    let mut best: Option<(u32, ScopeId)> = None;
    for &(aspan, aid) in arms {
        if exclude == Some(aid) {
            continue;
        }
        if !strictly_contains(aspan, span) {
            continue;
        }
        let width = aspan.end - aspan.start;
        if best.is_none_or(|(w, _)| width < w) {
            best = Some((width, aid));
        }
    }
    best.map(|(_, id)| id)
}

/// Innermost arm whose span strictly contains `span` while itself being
/// strictly contained by `upper_span` — i.e. an arm that sits between a
/// scope and its current parent.
fn innermost_arm_between(
    arms: &[(Span, ScopeId)],
    span: Span,
    upper_span: Span,
) -> Option<ScopeId> {
    let mut best: Option<(u32, ScopeId)> = None;
    for &(aspan, aid) in arms {
        if !strictly_contains(aspan, span) || !strictly_contains(upper_span, aspan) {
            continue;
        }
        let width = aspan.end - aspan.start;
        if best.is_none_or(|(w, _)| width < w) {
            best = Some((width, aid));
        }
    }
    best.map(|(_, id)| id)
}

/// `outer` contains `inner` and the two spans are not identical.
fn strictly_contains(outer: Span, inner: Span) -> bool {
    outer.start <= inner.start
        && inner.end <= outer.end
        && (outer.start < inner.start || inner.end < outer.end)
}

#[cfg(test)]
#[path = "synthesise_conditional_arms_test.rs"]
mod synthesise_conditional_arms_test;
