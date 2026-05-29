//! Serialized counterpart of [`crate::scope::CallbackArgument`].
//!
//! `callee` is the span-based head subtree of the enclosing call's
//! callee (e.g. `run`, `console.log`, `Promise.resolve().then`),
//! rendered by the visual-graph labeller into
//! `<callee>(args[<arg_index>])`. `statement_offset` is present only
//! for `ExpressionStatement`-level calls -- it keys the CallProxy
//! wrapper -- and is `null` for variable-bound / returned / nested
//! callbacks.

use serde::Serialize;

use crate::primitive::Utf16CodeUnitOffset;
use crate::serialized::serialized_expression_statement_head::SerializedHeadExpression;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedCallbackArgument {
    pub callee: SerializedHeadExpression,
    pub arg_index: u32,
    pub statement_offset: Option<Utf16CodeUnitOffset>,
}
