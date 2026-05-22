//! Per-AST-shape classify helpers for identifier references.
//!
//! The helpers match on `AstKind<'a>` variants directly to operate
//! on the structural AST form rather than on stringly-typed `type`
//! tags.

pub(crate) mod classify_identifier;
pub(crate) mod classify_ordinary_reference;
pub(crate) mod classify_result;
pub(crate) mod find_binding_root_context;
pub(crate) mod is_computed;
pub(crate) mod is_direct_binding;
pub(crate) mod is_pattern_step;
pub(crate) mod is_skip_context;
pub(crate) mod reference;
