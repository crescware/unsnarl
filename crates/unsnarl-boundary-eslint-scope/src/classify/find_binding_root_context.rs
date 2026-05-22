//! Walk up the ancestor chain through pattern wrappers until we hit
//! the binding-defining context.
//!
//! Returns the matching [`BindingRootContext`] (`Var` / `Param` /
//! `Catch` / `Assign`) or `None`. The walk: while the current
//! ancestor is a pattern step, climb one level; when a non-pattern
//! node is reached, the `(type, key)` pair determines the root
//! context.

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
        // so this shouldn't fire. Bail out safely.
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
                // oxc-specific: ESTree models `Function.params` as a
                // `Pattern[]` directly, but oxc wraps each entry as
                // `Function.params -> FormalParameters -> FormalParameter
                // -> pattern`. To produce the ESTree-equivalent
                // classify outcome, treat `FormalParameter` itself as
                // a binding terminator for its `pattern` slot.
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
