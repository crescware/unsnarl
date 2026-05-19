//! Decide what kind of slot a plain `Identifier` / `JSXIdentifier`
//! occupies.
//!
//! Mirrors `classifyIdentifier` in
//! `classify/classify-identifier.ts`. The order of the checks
//! matches TS exactly:
//!
//! 1. No parent → plain read reference.
//! 2. Skip context (estree property name, label, ...) → `Skip`.
//! 3. Direct binding slot (`VariableDeclarator.id`, etc.)
//!    → `Binding`, with the `var x = 1` special case promoted to a
//!    write reference with `init = true`.
//! 4. Pattern step (destructuring) → walk up to the root context
//!    and classify accordingly (`var` / `param` / `catch` → binding,
//!    `assign` → write reference).
//! 5. Otherwise → `classify_ordinary_reference`.

use oxc_ast::AstKind;

use unsnarl_ir::reference::reference_flags::ReferenceFlags;

use crate::classify::classify_ordinary_reference::classify_ordinary_reference;
use crate::classify::classify_result::ClassifyResult;
use crate::classify::find_binding_root_context::{find_binding_root_context, BindingRootContext};
use crate::classify::is_direct_binding::is_direct_binding;
use crate::classify::is_pattern_step::is_pattern_step;
use crate::classify::is_skip_context::is_skip_context;
use crate::classify::reference::reference;
use crate::walk::PathEntry;

pub(crate) fn classify_identifier(
    parent: Option<&AstKind<'_>>,
    key: Option<&'static str>,
    path: &[PathEntry<'_>],
) -> ClassifyResult {
    let Some(parent) = parent else {
        return reference(ReferenceFlags::READ, false);
    };
    if is_skip_context(parent, key) {
        return ClassifyResult::Skip;
    }
    if is_direct_binding(parent, key) {
        if let AstKind::VariableDeclarator(vd) = parent {
            if key == Some("id") && vd.init.is_some() {
                return reference(ReferenceFlags::WRITE, true);
            }
        }
        return ClassifyResult::Binding;
    }
    if let Some(last_idx) = path.len().checked_sub(1) {
        if is_pattern_step(parent, path, last_idx) {
            match find_binding_root_context(parent, key, path) {
                Some(BindingRootContext::Var)
                | Some(BindingRootContext::Param)
                | Some(BindingRootContext::Catch) => return ClassifyResult::Binding,
                Some(BindingRootContext::Assign) => {
                    return reference(ReferenceFlags::WRITE, false);
                }
                None => {}
            }
        }
    }
    classify_ordinary_reference(parent, key)
}

#[cfg(test)]
#[path = "classify_identifier_test.rs"]
mod classify_identifier_test;
