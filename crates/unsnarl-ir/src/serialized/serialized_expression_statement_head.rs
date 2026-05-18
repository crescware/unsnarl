//! Serialized counterpart of `HeadExpression`. Ports
//! `ts/src/ir/serialized/serialized-expression-statement-head.ts`.
//!
//! Identical shape to the internal head expression except the `raw`
//! leaf carries `Span` (line/column/offset) instead of bare offsets,
//! and assign / update operands likewise carry spans.

use serde::Serialize;

use crate::filled_string::FilledString;
use crate::primitive::Span;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedHeadOperand {
    pub head: SerializedHeadExpression,
    pub start_span: Span,
    pub end_span: Span,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum SerializedHeadExpression {
    Identifier {
        name: FilledString,
    },
    Member {
        object: Box<SerializedHeadExpression>,
        property: FilledString,
    },
    Call {
        callee: Box<SerializedHeadExpression>,
    },
    New {
        callee: Box<SerializedHeadExpression>,
    },
    Await {
        argument: Box<SerializedHeadExpression>,
    },
    Assign {
        operator: String,
        left: Box<SerializedHeadOperand>,
        right: Box<SerializedHeadOperand>,
    },
    Update {
        operator: String,
        prefix: bool,
        argument: Box<SerializedHeadOperand>,
    },
    Elided,
    #[serde(rename_all = "camelCase")]
    Raw {
        start_span: Span,
        end_span: Span,
    },
}
