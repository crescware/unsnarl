//! ExpressionStatement-wrapping info for a `Reference`.

use crate::primitive::Utf8ByteOffset;
use crate::reference::expression_statement_head::HeadExpression;

pub struct ExpressionStatementContainer {
    pub start_offset: Utf8ByteOffset,
    pub end_offset: Utf8ByteOffset,
    pub head: HeadExpression,
}
