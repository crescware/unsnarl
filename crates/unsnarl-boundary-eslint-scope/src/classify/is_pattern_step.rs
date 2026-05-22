//! Is this ancestor part of a destructuring pattern chain?
//!
//! Membership: `ObjectPattern` / `ArrayPattern` / `RestElement` /
//! `AssignmentPattern`, plus the special case where `Property` lives
//! inside an `ObjectPattern` (i.e. a destructuring property, not an
//! object-literal property).
//!
//! oxc-specific notes:
//!
//! - oxc splits `RestElement` into `BindingRestElement` (binding
//!   patterns) / `FormalParameterRest` (function rest parameters);
//!   only `BindingRestElement` appears inside `BindingPattern`
//!   chains.
//! - oxc's destructuring property is `BindingProperty` (vs the
//!   object-literal `ObjectProperty`), so the `Property in
//!   ObjectPattern` rule simplifies to `BindingProperty`.

use oxc_ast::AstKind;

use crate::walk::PathEntry;

pub(crate) fn is_pattern_step(node: &AstKind<'_>, path: &[PathEntry<'_>], i: usize) -> bool {
    match node {
        AstKind::ObjectPattern(_)
        | AstKind::ArrayPattern(_)
        | AstKind::BindingRestElement(_)
        | AstKind::AssignmentPattern(_) => true,
        // oxc-specific: ESTree models `Function.params` as a flat
        // `Pattern[]`, but oxc routes each rest parameter through
        // `FormalParameters.rest -> FormalParameterRest ->
        // BindingRestElement`. Treat the two wrapper nodes as
        // transparent pattern steps so `find_binding_root_context`
        // can keep climbing past them to the `Function` terminator.
        AstKind::FormalParameters(_) | AstKind::FormalParameterRest(_) => true,
        // oxc-specific: destructuring on the LHS of an assignment
        // expression (`[a, b] = ...` / `({ a } = ...)`) uses a
        // dedicated `AssignmentTarget*` family of nodes instead of
        // reusing the `BindingPattern` family. In ESTree the same
        // nodes appear as `ArrayPattern` / `ObjectPattern` /
        // `AssignmentPattern` / `RestElement`, all of which are
        // pattern steps; treat the assignment-target variants as
        // pattern steps too.
        AstKind::ArrayAssignmentTarget(_)
        | AstKind::ObjectAssignmentTarget(_)
        | AstKind::AssignmentTargetWithDefault(_)
        | AstKind::AssignmentTargetRest(_) => true,
        AstKind::BindingProperty(_) => i
            .checked_sub(1)
            .and_then(|prev| path.get(prev))
            .map(|p| matches!(p.node, AstKind::ObjectPattern(_)))
            .unwrap_or(false),
        AstKind::AssignmentTargetPropertyIdentifier(_)
        | AstKind::AssignmentTargetPropertyProperty(_) => i
            .checked_sub(1)
            .and_then(|prev| path.get(prev))
            .map(|p| matches!(p.node, AstKind::ObjectAssignmentTarget(_)))
            .unwrap_or(false),
        _ => false,
    }
}

#[cfg(test)]
#[path = "is_pattern_step_test.rs"]
mod is_pattern_step_test;
