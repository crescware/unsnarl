//! Serialized counterpart of [`crate::scope::CallbackHost`]: the same
//! annotation with byte offsets resolved to [`Span`]s at serialize time.
//! See [`crate::scope::CallbackHost`] for what each field means.

use serde::Serialize;

use crate::primitive::Span;
use crate::serialized::serialized_expression_statement_head::SerializedHeadExpression;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SerializedCallbackHostKind {
    VariableDeclarator,
    Return,
    Assignment,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedCallbackHost {
    pub kind: SerializedCallbackHostKind,
    pub start_span: Span,
    pub end_span: Span,
    pub head: SerializedHeadExpression,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_span: Option<Span>,
}
