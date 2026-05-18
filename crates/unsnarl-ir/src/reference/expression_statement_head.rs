//! Mini-AST captured for an ExpressionStatement's "head".
//!
//! Recursive: `Member` / `Call` / `New` / `Await` all reference
//! `HeadExpression`. `Assign` / `Update` operands carry their own
//! per-side offset so downstream consumers retain operand-level
//! positions even when the operand is `Elided` and therefore has no
//! intrinsic span.

pub struct HeadOperand {
    pub head: HeadExpression,
    pub start_offset: u32,
    pub end_offset: u32,
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
        operator: String,
        left: Box<HeadOperand>,
        right: Box<HeadOperand>,
    },
    Update {
        operator: String,
        prefix: bool,
        argument: Box<HeadOperand>,
    },
    Elided,
    Raw {
        start_offset: u32,
        end_offset: u32,
    },
}
