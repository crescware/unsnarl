//! Owner-resolution helpers.
//!
//! Mirrors `ts/src/analyzer/owner/`. Three pieces:
//!
//! * [`all_binding_variables`] — flattens a `BindingPattern` (used at
//!   `VariableDeclarator.id` sites) into the list of `VariableId`s it
//!   binds within `scope`'s chain.
//! * [`assignment_target_variables`] — the same operation for
//!   `AssignmentExpression.left` shapes, which oxc_ast spells as the
//!   separate `AssignmentTarget` enum rather than as `BindingPattern`.
//! * [`locate_reference_owner_slot`] — walks the ancestor chain
//!   leaf -> root and reports the nearest interesting slot (a
//!   `VariableDeclarator` / `AssignmentExpression`, or a function /
//!   class boundary that cuts off the search). The call site is
//!   responsible for resolving the slot back to an AST handle and
//!   invoking one of the two helpers above; the analyzer crate stays
//!   free of `(path index → AST reference)` bookkeeping for the same
//!   reason the expression-statement-container split does.

pub mod all_binding_variables;
pub mod find_reference_owners;

pub use all_binding_variables::{
    all_binding_variables, assignment_target_variables, walk_assignment_target_identifiers,
};
pub use find_reference_owners::{locate_reference_owner_slot, OwnerLookup};
