//! Resolve a binding pattern (or assignment-target pattern) against
//! the live scope chain.
//!
//! Mirrors `ts/src/analyzer/owner/all-binding-variables.ts`. The TS
//! port takes an unnormalised `AstNode` and dispatches dynamically;
//! the Rust port splits along the static type:
//!
//! * [`all_binding_variables`] for `BindingPattern` (the
//!   `VariableDeclarator.id` shape).
//! * [`assignment_target_variables`] for `AssignmentTarget` (the
//!   `AssignmentExpression.left` shape, which oxc_ast spells with a
//!   different enum even though the TS code treats it as the same
//!   pattern grammar).
//!
//! Both helpers collect identifier names from the pattern, resolve
//! each name through [`resolve_in_scope_chain`], and dedupe the
//! returned `VariableId`s in encounter order.

use oxc_ast::ast::{
    AssignmentTarget, AssignmentTargetMaybeDefault, AssignmentTargetProperty, BindingPattern,
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
    let mut names: Vec<String> = Vec::new();
    collect_assignment_target_names(target, &mut names);
    let mut out: Vec<VariableId> = Vec::new();
    for name in names {
        if let Some(id) = resolve_in_scope_chain(arena, scope, &name) {
            if !out.contains(&id) {
                out.push(id);
            }
        }
    }
    out
}

fn collect_assignment_target_names(target: &AssignmentTarget<'_>, out: &mut Vec<String>) {
    use AssignmentTarget as AT;
    match target {
        AT::AssignmentTargetIdentifier(id) => {
            out.push(id.name.as_str().to_string());
        }
        AT::ArrayAssignmentTarget(arr) => {
            for el in arr.elements.iter().flatten() {
                collect_maybe_default(el, out);
            }
            if let Some(rest) = arr.rest.as_deref() {
                collect_assignment_target_names(&rest.target, out);
            }
        }
        AT::ObjectAssignmentTarget(obj) => {
            for prop in &obj.properties {
                collect_property(prop, out);
            }
            if let Some(rest) = obj.rest.as_deref() {
                collect_assignment_target_names(&rest.target, out);
            }
        }
        // Member expressions and TS wrappers in target position do not
        // introduce new bindings; mirror the TS behavior of falling
        // through with no contributions.
        AT::ComputedMemberExpression(_)
        | AT::StaticMemberExpression(_)
        | AT::PrivateFieldExpression(_)
        | AT::TSAsExpression(_)
        | AT::TSSatisfiesExpression(_)
        | AT::TSNonNullExpression(_)
        | AT::TSTypeAssertion(_) => {}
    }
}

fn collect_maybe_default(node: &AssignmentTargetMaybeDefault<'_>, out: &mut Vec<String>) {
    use AssignmentTargetMaybeDefault as M;
    match node {
        M::AssignmentTargetWithDefault(wd) => {
            collect_assignment_target_names(&wd.binding, out);
        }
        M::AssignmentTargetIdentifier(id) => {
            out.push(id.name.as_str().to_string());
        }
        M::ArrayAssignmentTarget(arr) => {
            for el in arr.elements.iter().flatten() {
                collect_maybe_default(el, out);
            }
            if let Some(rest) = arr.rest.as_deref() {
                collect_assignment_target_names(&rest.target, out);
            }
        }
        M::ObjectAssignmentTarget(obj) => {
            for prop in &obj.properties {
                collect_property(prop, out);
            }
            if let Some(rest) = obj.rest.as_deref() {
                collect_assignment_target_names(&rest.target, out);
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

fn collect_property(prop: &AssignmentTargetProperty<'_>, out: &mut Vec<String>) {
    match prop {
        AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(id) => {
            // The shorthand `{ foo }` and `{ foo = init }` forms: the
            // binding *is* the property key. The `init` slot here is
            // the default value, not a binding.
            out.push(id.binding.name.as_str().to_string());
        }
        AssignmentTargetProperty::AssignmentTargetPropertyProperty(p) => {
            collect_maybe_default(&p.binding, out);
        }
    }
}

#[cfg(test)]
#[path = "all_binding_variables_test.rs"]
mod all_binding_variables_test;
