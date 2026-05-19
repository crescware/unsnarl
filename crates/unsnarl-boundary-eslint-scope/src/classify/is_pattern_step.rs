//! Is this ancestor part of a destructuring pattern chain?
//!
//! Mirrors `isPatternStep` in `classify/is-pattern-step.ts`. The TS
//! port checks `node.type` against `ObjectPattern` / `ArrayPattern` /
//! `RestElement` / `AssignmentPattern`, plus the special case where
//! `Property` lives inside an `ObjectPattern` (i.e. a destructuring
//! property, not an object-literal property).
//!
//! The Rust port matches the same set, with two notes:
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
        AstKind::BindingProperty(_) => i
            .checked_sub(1)
            .and_then(|prev| path.get(prev))
            .map(|p| matches!(p.node, AstKind::ObjectPattern(_)))
            .unwrap_or(false),
        _ => false,
    }
}
