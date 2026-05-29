//! Serialized counterpart of `HeadExpression`.
//!
//! Identical shape to the internal head expression except the `Raw`
//! leaf carries `Span` (line/column/offset) instead of bare offsets,
//! and `Assign` / `Update` operands likewise carry spans rather than
//! raw offsets.

use serde::Serialize;
use unsnarl_oxc_parity::{AssignOperator, UpdateOperator};

use crate::non_empty::assert_non_empty;
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
        name: String,
    },
    Member {
        object: Box<SerializedHeadExpression>,
        property: String,
    },
    /// `start_span` / `end_span` carry the **enclosing
    /// CallExpression**'s span. They are emitted so consumers
    /// (notably the visual-graph builder's callback-arg labeller)
    /// can match a call node inside a chained expression
    /// (`a.b().c(cb)`) where every nested `CallExpression` shares
    /// the same `start.offset` with the chain root.
    #[serde(rename_all = "camelCase")]
    Call {
        callee: Box<SerializedHeadExpression>,
        start_span: Span,
        end_span: Span,
    },
    /// `start_span` / `end_span` carry the **enclosing
    /// NewExpression**'s span. Same identification rationale as
    /// [`Self::Call`].
    #[serde(rename_all = "camelCase")]
    New {
        callee: Box<SerializedHeadExpression>,
        start_span: Span,
        end_span: Span,
    },
    Await {
        argument: Box<SerializedHeadExpression>,
    },
    Assign {
        operator: AssignOperator,
        left: Box<SerializedHeadOperand>,
        right: Box<SerializedHeadOperand>,
    },
    Update {
        operator: UpdateOperator,
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

impl SerializedHeadExpression {
    pub fn identifier(name: String) -> Self {
        assert_non_empty(&name, "SerializedHeadExpression::Identifier.name");
        Self::Identifier { name }
    }

    pub fn member(object: SerializedHeadExpression, property: String) -> Self {
        assert_non_empty(&property, "SerializedHeadExpression::Member.property");
        Self::Member {
            object: Box::new(object),
            property,
        }
    }
}
