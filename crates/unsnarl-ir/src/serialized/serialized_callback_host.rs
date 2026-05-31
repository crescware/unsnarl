//! Serialized counterpart of [`crate::scope::CallbackHost`].
//!
//! `start_span` / `end_span` cover the host's bound expression (the
//! declarator init / return argument / assignment RHS); the visual
//! graph uses them as the containing CallProxy's extent. `head` renders
//! the proxy label. `target_span` is present only for an `Assignment`
//! host with a plain-identifier left-hand side; the visual graph maps
//! it to the reassignment's write-op node so the proxy can be bundled
//! with that node by containment.

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
