//! Scope-builder + parser wrapper around the oxc crates
//! (`oxc-parser`, `oxc_semantic`, `oxc_ast`).
//!
//! Boundary crate: must not depend on `unsnarl-annotations` or
//! `unsnarl-analyzer`. This invariant is physically enforced by the
//! absence of those entries in `Cargo.toml`.

pub mod analysis_result;
pub mod analyze;
#[cfg(test)]
pub(crate) mod boundary_fixtures;
pub mod declare;
pub mod materialise;
pub mod oxc_semantic_adapter;
pub mod parser;
pub mod resolve;
pub mod visitor;

#[cfg(test)]
#[path = "ast_type_coverage_test.rs"]
mod ast_type_coverage_test;
