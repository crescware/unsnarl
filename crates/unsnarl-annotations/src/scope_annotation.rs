//! Side-table row for `ScopeData`.
//!
//! Field order matches the source interface (`blockContext`,
//! `fallsThrough`, `exitsFunction`, `nestingDepths`). The pipeline
//! does not serialize this struct directly; `SerializedScope` reads
//! each field individually at serialize time. The `Serialize` derive
//! is in place because every constituent type already serializes,
//! so the in-memory and pipeline shapes coincide for this row.

use serde::Serialize;

use unsnarl_ir::nesting_kind::NestingDepths;
use unsnarl_ir::scope::{AbruptStatement, BlockContext, CallbackArgument};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeAnnotation {
    pub block_context: Option<BlockContext>,
    pub callback_argument: Option<CallbackArgument>,
    pub falls_through: bool,
    pub exits_function: bool,
    pub nesting_depths: NestingDepths,
    pub abrupt_statements: Vec<AbruptStatement>,
}

#[cfg(test)]
#[path = "scope_annotation_test.rs"]
mod scope_annotation_test;
