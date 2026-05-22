//! Declare-side helpers shared across `enter_*` and `hoisting/*`.
//!
//! `declare_variable` lives in [`crate::state`] because it directly
//! mutates `ScopeBuilderState`; the remaining helpers
//! (`collect_binding_identifiers`, `declare_implicit_arguments`)
//! stay here.

pub mod collect_binding_identifiers;
pub(crate) mod declare_implicit_arguments;

pub use collect_binding_identifiers::collect_binding_identifiers;
