//! Per-AST-shape classify helpers for identifier references.
//!
//! Mirrors `ts/src/boundary/eslint-scope/classify/`. The TS layer
//! switches on `node.type` strings; the Rust port matches on
//! `AstKind<'a>` variants directly, so all helpers consume the
//! structural form that `classify-identifier` and friends originally
//! read off the unnormalised `NodeLike`.

pub(crate) mod classify_identifier;
pub(crate) mod classify_ordinary_reference;
pub(crate) mod classify_result;
pub(crate) mod find_binding_root_context;
pub(crate) mod is_computed;
pub(crate) mod is_direct_binding;
pub(crate) mod is_pattern_step;
pub(crate) mod is_skip_context;
pub(crate) mod reference;
