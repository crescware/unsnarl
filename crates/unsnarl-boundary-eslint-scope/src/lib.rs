//! eslint-scope compatible scope-builder + parser wrapper.
//!
//! Boundary crate: must not depend on `unsnarl-annotations` or `unsnarl-analyzer`.
//! This invariant is physically enforced by the absence of those entries in
//! `Cargo.toml`.
//!
//! Step 8 populated the parser wrapper (`parser` module). Step 8.5 added the
//! scope-builder entry skeleton (`analyze` / `analysis_result` / `visitor`
//! modules) so the parser API surface can be reviewed against an actual
//! consumer; the body of `analyze` is fleshed out across Step 9.

pub mod analysis_result;
pub mod analyze;
pub mod parser;
pub(crate) mod state;
pub mod visitor;
