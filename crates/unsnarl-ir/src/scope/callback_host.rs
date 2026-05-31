//! Host annotation for a callback argument.
//!
//! The host is the nearest binding / return / assignment whose *value*
//! is the callback's enclosing call (directly or through nested
//! argument calls). It drives the visual-graph CallProxy that contains
//! the callback: the proxy spans the host's bound expression and, for a
//! `VariableDeclarator` host, is bundled with the bound variable by
//! containment.
//!
//! Statement-hosted callbacks (`arr.forEach(cb);`) carry **no** host --
//! the visual layer already groups those via the `ExpressionStatement`
//! spans it owns. Keeping the host to declarator / return / assignment
//! is what lets the statement path stay untouched while the same
//! containment is extended to the remaining contexts.
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
    /// Span of the bound expression (declarator init / return argument
    /// / assignment RHS) -- the CallProxy's extent.
    pub start_offset: Utf8ByteOffset,
    pub end_offset: Utf8ByteOffset,
    /// Head of the bound expression, rendered as the proxy's label.
    pub head: HeadExpression,
    /// For an [`CallbackHostKind::Assignment`] whose left-hand side is a
    /// plain identifier (`y = arr.map(cb)`), the offset of that target
    /// identifier. The visual layer maps it to the reassignment's
    /// write-op node and bundles the CallProxy with that node -- the
    /// call ↔ variable relationship shown by containment, mirroring the
    /// declarator case. `None` for declarator / return hosts and for
    /// non-identifier assignment targets (member / destructuring), which
    /// carry no single write-op node.
    pub target_offset: Option<Utf8ByteOffset>,
}
