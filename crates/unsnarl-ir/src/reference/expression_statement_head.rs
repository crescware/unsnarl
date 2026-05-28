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
    /// `start_offset` / `end_offset` carry the **enclosing
    /// CallExpression**'s span (UTF-8 byte). In a chained call
    /// expression (`a.b().c(cb)`) every nested `CallExpression`
    /// shares its `span.start` with the chain root, so consumers
    /// that need to identify which call in the chain a position
    /// targets must compare both ends. Populated by
    /// `analyzer::build_head_expression` at construction time and
    /// translated to UTF-16 [`crate::primitive::Span`]s by the flat
    /// serializer.
    Call {
        callee: Box<HeadExpression>,
        start_offset: Utf8ByteOffset,
        end_offset: Utf8ByteOffset,
    },
    /// `start_offset` / `end_offset` carry the **enclosing
    /// NewExpression**'s span (UTF-8 byte). Same identification
    /// rationale as [`Self::Call`].
    New {
        callee: Box<HeadExpression>,
        start_offset: Utf8ByteOffset,
        end_offset: Utf8ByteOffset,
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
