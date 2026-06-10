//! ExpressionStatement-wrapping info for a `Reference`.

use crate::primitive::Utf8ByteOffset;
use crate::reference::expression_statement_head::HeadExpression;

pub struct ExpressionStatementContainer {
    pub start_offset: Utf8ByteOffset,
    pub end_offset: Utf8ByteOffset,
    pub head: HeadExpression,
    /// Start of the statement's `ConditionalExpression` once the
    /// wrapping parentheses are stripped, set **only** for a
    /// parenthesized bare ternary statement (`(cond ? a : b);`). There
    /// the statement span starts at `(`, so `start_offset` no longer
    /// coincides with the `ConditionalExpression`'s start; this carries
    /// that inner start so the visual layer can still recognise the
    /// statement as a bare ternary (and discard its arm values). `None`
    /// for an unparenthesized expression and for a parenthesized
    /// non-conditional, where `start_offset` already points at the
    /// right place and the override is unnecessary.
    pub expression_start_offset: Option<Utf8ByteOffset>,
}
