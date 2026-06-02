//! Host annotation for a callback argument.
//!
//! The host is the nearest binding / return / assignment whose *value*
//! is the callback's enclosing call (directly or through nested
//! argument calls).
//!
//! Statement-hosted callbacks (`arr.forEach(cb);`) deliberately carry
//! **no** host: that context is recoverable downstream from the
//! `ExpressionStatement` spans alone, so the IR records the host only
//! for the three binding forms and leaves layout to its consumer.
//!
//! Like [`super::CallbackArgument`], the in-memory shape carries the
//! UTF-8 [`HeadExpression`] while the serialized counterpart carries the
//! span-based head; the conversion happens at serialize time.

use crate::primitive::Utf8ByteOffset;
use crate::reference::expression_statement_head::HeadExpression;

pub enum CallbackHostKind {
    VariableDeclarator,
    Return,
    Assignment,
}

pub struct CallbackHost {
    pub kind: CallbackHostKind,
    /// Span of the bound expression: the declarator init, the return
    /// argument, or the assignment RHS.
    pub start_offset: Utf8ByteOffset,
    pub end_offset: Utf8ByteOffset,
    /// Head of the bound expression.
    pub head: HeadExpression,
    /// For an [`CallbackHostKind::Assignment`] whose left-hand side is a
    /// plain identifier (`y = arr.map(cb)`), the offset of that target
    /// identifier. `None` for declarator / return hosts and for
    /// non-identifier assignment targets (member / destructuring), which
    /// have no single target identifier to record.
    pub target_offset: Option<Utf8ByteOffset>,
}
