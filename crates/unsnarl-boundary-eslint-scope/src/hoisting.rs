//! Hoist-pass helpers.
//!
//! Groups `hoist_declarations`, `visit`, and the four
//! `handle_*_declaration` per-shape handlers (`Class`, `Function`,
//! `Import`, `Variable`).
//!
//! `oxc_ast`'s strongly-typed AST enums subsume the work that an
//! unnormalised `NodeLike` (string-typed `type` field) would
//! otherwise require, so there is no separate
//! `is_identifier_node` / `node_like` module.

pub(crate) mod handle_class_declaration;
pub(crate) mod handle_function_declaration;
pub(crate) mod handle_import_declaration;
pub(crate) mod handle_variable_declaration;
pub(crate) mod hoist_declarations;
pub(crate) mod visit;
