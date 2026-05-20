//! ExpressionStatement-wrapping info for a `Reference`.

use crate::primitive::SourceOffset;
use crate::reference::expression_statement_head::HeadExpression;

pub struct ExpressionStatementContainer {
    pub start_offset: SourceOffset,
    pub end_offset: SourceOffset,
    pub head: HeadExpression,
}
