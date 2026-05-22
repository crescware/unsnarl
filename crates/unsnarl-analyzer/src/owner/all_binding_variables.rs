//! Resolve a binding pattern (or assignment-target pattern) against
//! the live scope chain.
//!
//! The two shapes are split along the static type:
//!
//! * [`all_binding_variables`] for `BindingPattern` (the
//!   `VariableDeclarator.id` shape).
//! * [`assignment_target_variables`] for `AssignmentTarget` (the
//!   `AssignmentExpression.left` shape, which oxc_ast spells with a
//!   different enum even though the underlying pattern grammar is
//!   the same).
//!
//! Both helpers collect identifier names from the pattern, resolve
//! each name through [`resolve_in_scope_chain`], and dedupe the
//! returned `VariableId`s in encounter order.

use oxc_ast::ast::{
    AssignmentTarget, AssignmentTargetMaybeDefault, AssignmentTargetProperty, BindingPattern,
    IdentifierReference,
};

use unsnarl_boundary_eslint_scope::declare::collect_binding_identifiers;
use unsnarl_boundary_eslint_scope::resolve::resolve_in_scope_chain;
use unsnarl_ir::ids::{ScopeId, VariableId};
use unsnarl_ir::IrArena;

pub fn all_binding_variables(
    pattern: &BindingPattern<'_>,
    scope: ScopeId,
    arena: &IrArena,
) -> Vec<VariableId> {
    let idents = collect_binding_identifiers(pattern);
    let mut out: Vec<VariableId> = Vec::new();
    for ident in idents {
        if let Some(id) = resolve_in_scope_chain(arena, scope, ident.name()) {
            if !out.contains(&id) {
                out.push(id);
            }
        }
    }
    out
}

pub fn assignment_target_variables(
    target: &AssignmentTarget<'_>,
    scope: ScopeId,
    arena: &IrArena,
) -> Vec<VariableId> {
    let mut out: Vec<VariableId> = Vec::new();
    walk_assignment_target_identifiers(target, &mut |id| {
        if let Some(var_id) = resolve_in_scope_chain(arena, scope, id.name.as_str()) {
            if !out.contains(&var_id) {
                out.push(var_id);
            }
        }
    });
    out
}

/// Walk every identifier reachable from an `AssignmentTarget`,
/// invoking `f` once per occurrence in source order.
///
/// The traversal mirrors the TS `findReferenceOwners /
/// allBindingVariables` shape: identifiers nested under
/// destructuring patterns, defaulted slots, rest targets and
/// shorthand property bindings are all visited. Member expressions
/// and TS-only wrappers in target position contribute no bindings
/// and are skipped.
///
/// `oxc_ast` spells the identifier slot two different ways: the
/// `AssignmentTargetIdentifier` variants wrap a
/// `Box<IdentifierReference>`, while shorthand property bindings
/// hold the same `IdentifierReference` directly. The callback
/// receives `&IdentifierReference` in every arm so the shape stays
/// uniform.
///
/// Used both by [`assignment_target_variables`] (which resolves
/// each name against the live scope chain) and by the analysis-pass
/// `BuildAnalysisVisitor` (which uses each identifier's span to
/// look up the existing reference's resolved binding instead).
pub fn walk_assignment_target_identifiers(
    target: &AssignmentTarget<'_>,
    f: &mut dyn FnMut(&IdentifierReference<'_>),
) {
    use AssignmentTarget as AT;
    match target {
        AT::AssignmentTargetIdentifier(id) => {
            f(id.as_ref());
        }
        AT::ArrayAssignmentTarget(arr) => {
            for el in arr.elements.iter().flatten() {
                walk_maybe_default(el, f);
            }
            if let Some(rest) = arr.rest.as_deref() {
                walk_assignment_target_identifiers(&rest.target, f);
            }
        }
        AT::ObjectAssignmentTarget(obj) => {
            for prop in &obj.properties {
                walk_property(prop, f);
            }
            if let Some(rest) = obj.rest.as_deref() {
                walk_assignment_target_identifiers(&rest.target, f);
            }
        }
        AT::ComputedMemberExpression(_)
        | AT::StaticMemberExpression(_)
        | AT::PrivateFieldExpression(_)
        | AT::TSAsExpression(_)
        | AT::TSSatisfiesExpression(_)
        | AT::TSNonNullExpression(_)
        | AT::TSTypeAssertion(_) => {}
    }
}

fn walk_maybe_default(
    node: &AssignmentTargetMaybeDefault<'_>,
    f: &mut dyn FnMut(&IdentifierReference<'_>),
) {
    use AssignmentTargetMaybeDefault as M;
    match node {
        M::AssignmentTargetWithDefault(wd) => {
            walk_assignment_target_identifiers(&wd.binding, f);
        }
        M::AssignmentTargetIdentifier(id) => {
            f(id.as_ref());
        }
        M::ArrayAssignmentTarget(arr) => {
            for el in arr.elements.iter().flatten() {
                walk_maybe_default(el, f);
            }
            if let Some(rest) = arr.rest.as_deref() {
                walk_assignment_target_identifiers(&rest.target, f);
            }
        }
        M::ObjectAssignmentTarget(obj) => {
            for prop in &obj.properties {
                walk_property(prop, f);
            }
            if let Some(rest) = obj.rest.as_deref() {
                walk_assignment_target_identifiers(&rest.target, f);
            }
        }
        M::ComputedMemberExpression(_)
        | M::StaticMemberExpression(_)
        | M::PrivateFieldExpression(_)
        | M::TSAsExpression(_)
        | M::TSSatisfiesExpression(_)
        | M::TSNonNullExpression(_)
        | M::TSTypeAssertion(_) => {}
    }
}

fn walk_property(prop: &AssignmentTargetProperty<'_>, f: &mut dyn FnMut(&IdentifierReference<'_>)) {
    match prop {
        AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(id) => {
            // The shorthand `{ foo }` and `{ foo = init }` forms: the
            // binding *is* the property key. The `init` slot here is
            // the default value, not a binding.
            f(&id.binding);
        }
        AssignmentTargetProperty::AssignmentTargetPropertyProperty(p) => {
            walk_maybe_default(&p.binding, f);
        }
    }
}

#[cfg(test)]
#[path = "all_binding_variables_test.rs"]
mod all_binding_variables_test;
