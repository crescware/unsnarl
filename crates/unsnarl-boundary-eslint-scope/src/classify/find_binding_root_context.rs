//! Walk up the ancestor chain through pattern wrappers until we hit
//! the binding-defining context.
//!
//! Mirrors `findBindingRootContext` in
//! `classify/find-binding-root-context.ts`. The TS port returns one
//! of `"var" | "param" | "catch" | "assign" | null` and the Rust
//! port mirrors that as a dedicated enum
//! ([`BindingRootContext`]). The walk shape is identical: while the
//! current ancestor is a pattern step, climb one level; when we hit
//! a non-pattern node, the relevant `(type, key)` pair determines
//! the root context.

use oxc_ast::AstKind;

use crate::classify::is_pattern_step::is_pattern_step;
use crate::walk::PathEntry;

pub(crate) enum BindingRootContext {
    Var,
    Param,
    Catch,
    Assign,
}

pub(crate) fn find_binding_root_context(
    parent: &AstKind<'_>,
    key: Option<&'static str>,
    path: &[PathEntry<'_>],
) -> Option<BindingRootContext> {
    let mut cur_parent: Option<AstKind<'_>> = Some(*parent);
    let mut cur_key = key;
    let mut i = path.len();
    if i == 0 {
        // Defensive: classify is called with `path[len-1] == parent`,
        // so this shouldn't fire. Bail out the same way TS does.
        return None;
    }
    i -= 1;
    while let Some(p) = cur_parent {
        if !is_pattern_step(&p, path, i) {
            return match p {
                AstKind::VariableDeclarator(_) if cur_key == Some("id") => {
                    Some(BindingRootContext::Var)
                }
                AstKind::CatchClause(_) if cur_key == Some("param") => {
                    Some(BindingRootContext::Catch)
                }
                AstKind::Function(_) | AstKind::ArrowFunctionExpression(_)
                    if cur_key == Some("params") =>
                {
                    Some(BindingRootContext::Param)
                }
                // oxc-specific: TS folds Function.params -> Pattern[]
                // into a single field, but oxc wraps it as
                // Function.params -> FormalParameters -> FormalParameter
                // -> pattern. To keep classify aligned with TS, we
                // treat `FormalParameter` itself as a binding terminator
                // for its `pattern` slot.
                AstKind::FormalParameter(_) if cur_key == Some("pattern") => {
                    Some(BindingRootContext::Param)
                }
                AstKind::AssignmentExpression(_) if cur_key == Some("left") => {
                    Some(BindingRootContext::Assign)
                }
                _ => None,
            };
        }
        if i == 0 {
            return None;
        }
        i -= 1;
        let next = path.get(i)?;
        cur_parent = Some(next.node);
        cur_key = path.get(i + 1).and_then(|e| e.key);
    }
    None
}

#[cfg(test)]
#[path = "find_binding_root_context_test.rs"]
mod find_binding_root_context_test;
