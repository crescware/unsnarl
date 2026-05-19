//! eslint-scope compatible scope-builder + parser wrapper.
//!
//! Boundary crate: must not depend on `unsnarl-annotations` or `unsnarl-analyzer`.
//! This invariant is physically enforced by the absence of those entries in
//! `Cargo.toml`.
//!
//! Step 8 has populated the parser wrapper (`parser` module); the
//! scope-builder will be added in Step 9.

pub mod parser;
