//! Scope-side annotation marking a function scope as the `arg_index`-th
//! argument of an enclosing `CallExpression` (or `NewExpression`).
//!
//! Re-abstracted for #220: the annotation is now self-contained. It
//! carries the call's `callee` head subtree and the zero-based
//! `arg_index`, so the visual-graph labeller can render
//! `<callee>(args[<arg_index>])` for **any** call-argument function,
//! independent of whether the enclosing call sits at
//! `ExpressionStatement` level. Previously the callee text was
//! recovered indirectly from the reference-side
//! `ExpressionStatementContainer.head`, which only existed for
//! `ExpressionStatement`-level calls and left variable-bound /
//! returned / nested callbacks rendering as `(anonymous)`.
//!
//! `statement_offset` is `Some` only when the enclosing call is an
//! `ExpressionStatement`-level call. The CallProxy wrapper (which
//! reuses the `expr_stmt_<offset>` leaf to group callback bodies) is
//! an `ExpressionStatement`-specific mechanism and keys off this
//! offset; non-statement callbacks carry `None` and receive only the
//! label, never a wrapper.
//!
//! Like [`crate::reference::ExpressionStatementContainer`], the
//! in-memory shape (this type) carries the UTF-8 [`HeadExpression`]
//! while the on-disk
//! [`crate::serialized::SerializedCallbackArgument`] carries the
//! span-based `SerializedHeadExpression`; the conversion happens at
//! serialize time.

use crate::primitive::Utf16CodeUnitOffset;
use crate::reference::expression_statement_head::HeadExpression;

pub struct CallbackArgument {
    pub callee: HeadExpression,
    pub arg_index: u32,
    pub statement_offset: Option<Utf16CodeUnitOffset>,
}

impl CallbackArgument {
    pub fn new(
        callee: HeadExpression,
        arg_index: u32,
        statement_offset: Option<Utf16CodeUnitOffset>,
    ) -> Self {
        Self {
            callee,
            arg_index,
            statement_offset,
        }
    }
}
