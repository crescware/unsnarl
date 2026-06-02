//! Scope-side annotation marking a function scope as the `arg_index`-th
//! argument of an enclosing `CallExpression` (or `NewExpression`).
//!
//! Re-abstracted for #220: the annotation carries only the **structural
//! fact** that AST analysis can establish -- the call's `callee` head
//! subtree and the zero-based `arg_index`. It says nothing about how
//! the function is rendered.
//!
//! Whether a callback is hosted by a statement-level CallProxy wrapper
//! (and under which `expr_stmt_<offset>` it groups) is a visual-graph
//! rendering concern, resolved there from the `ExpressionStatement`
//! spans the visual-graph builder already owns -- not encoded here.
//! Keeping that correlation out of the IR is what lets each layer carry
//! the minimum it needs: the IR the structure, the visual graph the
//! layout.
//!
//! Like [`crate::reference::ExpressionStatementContainer`], the
//! in-memory shape (this type) carries the UTF-8 [`HeadExpression`]
//! while the on-disk
//! [`crate::serialized::SerializedCallbackArgument`] carries the
//! span-based `SerializedHeadExpression`; the conversion happens at
//! serialize time.

use crate::reference::expression_statement_head::HeadExpression;
use crate::scope::callback_host::CallbackHost;

pub struct CallbackArgument {
    pub callee: HeadExpression,
    pub arg_index: u32,
    /// The binding / return / assignment whose value is this call, when
    /// there is one. `None` for statement-hosted callbacks (recoverable
    /// from the `ExpressionStatement` spans) and callbacks with no
    /// recognised host.
    pub host: Option<CallbackHost>,
}

impl CallbackArgument {
    pub fn new(callee: HeadExpression, arg_index: u32, host: Option<CallbackHost>) -> Self {
        Self {
            callee,
            arg_index,
            host,
        }
    }
}
