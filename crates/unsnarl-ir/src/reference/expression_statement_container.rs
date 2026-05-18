//! ExpressionStatement-wrapping info for a `Reference`. Ports
//! `ts/src/ir/reference/expression-statement-container.ts`.

use crate::reference::expression_statement_head::HeadExpression;

pub struct ExpressionStatementContainer {
    pub start_offset: u32,
    pub end_offset: u32,
    pub head: HeadExpression,
}
