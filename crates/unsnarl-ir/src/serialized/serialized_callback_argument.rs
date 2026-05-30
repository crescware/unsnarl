//! Serialized counterpart of [`crate::scope::CallbackArgument`].
//!
//! `callee` is the span-based head subtree of the enclosing call's
//! callee (e.g. `run`, `console.log`, `Promise.resolve().then`),
//! rendered by the visual-graph labeller into
//! `<callee>(args[<arg_index>])`. The annotation is a pure structural
//! fact -- it carries no rendering correlation (the CallProxy wrapper
//! grouping is resolved in the visual-graph layer).

use serde::Serialize;

use crate::serialized::serialized_callback_host::SerializedCallbackHost;
use crate::serialized::serialized_expression_statement_head::SerializedHeadExpression;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedCallbackArgument {
    pub callee: SerializedHeadExpression,
    pub arg_index: u32,
    /// The binding / return / assignment whose value is this call.
    /// Present for non-statement contexts; absent for statement-hosted
    /// or host-less callbacks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<SerializedCallbackHost>,
}
