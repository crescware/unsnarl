//! Mini-AST captured for an ExpressionStatement's "head".
//!
//! Recursive: `Member` / `Call` / `New` / `Await` all reference
//! `HeadExpression`. `Assign` / `Update` operands carry their own
//! per-side offset so downstream consumers retain operand-level
//! positions even when the operand is `Elided` and therefore has no
//! intrinsic span.

use unsnarl_oxc_parity::{AssignOperator, UpdateOperator};

use crate::primitive::Utf8ByteOffset;

pub struct HeadOperand {
    pub head: HeadExpression,
    pub start_offset: Utf8ByteOffset,
    pub end_offset: Utf8ByteOffset,
}

pub enum HeadExpression {
    Identifier {
        name: String,
    },
    Member {
        object: Box<HeadExpression>,
        property: String,
    },
    Call {
        callee: Box<HeadExpression>,
    },
    New {
        callee: Box<HeadExpression>,
    },
    Await {
        argument: Box<HeadExpression>,
    },
    Assign {
        operator: AssignOperator,
        left: Box<HeadOperand>,
        right: Box<HeadOperand>,
    },
    Update {
        operator: UpdateOperator,
        prefix: bool,
        argument: Box<HeadOperand>,
    },
    Elided,
    Raw {
        start_offset: Utf8ByteOffset,
        end_offset: Utf8ByteOffset,
    },
}

impl HeadExpression {
    pub fn identifier(name: String) -> Self {
        assert!(
            !name.is_empty(),
            "HeadExpression::Identifier.name must be non-empty"
        );
        Self::Identifier { name }
    }

    pub fn member(object: HeadExpression, property: String) -> Self {
        assert!(
            !property.is_empty(),
            "HeadExpression::Member.property must be non-empty"
        );
        Self::Member {
            object: Box::new(object),
            property,
        }
    }
}
