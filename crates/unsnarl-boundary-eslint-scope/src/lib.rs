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
pub(crate) mod declare;
pub(crate) mod declare_for_left;
pub(crate) mod declare_function_params;
pub(crate) mod diagnostic_collector;
pub(crate) mod enter_block;
pub(crate) mod enter_catch;
pub(crate) mod enter_class;
pub(crate) mod enter_for;
pub(crate) mod enter_function;
pub(crate) mod enter_switch;
pub(crate) mod enter_switch_case;
pub(crate) mod hoist_into;
pub(crate) mod hoisting;
pub mod parser;
pub(crate) mod scope_build_visitor;
pub(crate) mod skip_block_scope;
pub(crate) mod span_util;
pub(crate) mod state;
pub mod visitor;
