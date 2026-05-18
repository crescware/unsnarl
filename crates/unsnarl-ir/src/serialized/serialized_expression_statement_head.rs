//! Serialized counterpart of `HeadExpression`.
//!
//! Identical shape to the internal head expression except the `Raw`
//! leaf carries `Span` (line/column/offset) instead of bare offsets,
//! and `Assign` / `Update` operands likewise carry spans rather than
//! raw offsets.

use serde::Serialize;

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

impl SerializedHeadExpression {
    pub fn identifier(name: impl Into<String>) -> Self {
        let name = name.into();
        assert!(
            !name.is_empty(),
            "SerializedHeadExpression::Identifier.name must be non-empty"
        );
        Self::Identifier { name }
    }

    pub fn member(object: SerializedHeadExpression, property: impl Into<String>) -> Self {
        let property = property.into();
        assert!(
            !property.is_empty(),
            "SerializedHeadExpression::Member.property must be non-empty"
        );
        Self::Member {
            object: Box::new(object),
            property,
        }
    }
}
