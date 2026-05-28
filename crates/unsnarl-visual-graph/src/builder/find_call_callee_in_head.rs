//! Walk a [`SerializedHeadExpression`] tree and locate the
//! `Call` / `New` node whose own span matches a `(start, end)` pair,
//! returning its `callee` subtree.
//!
//! Used by the callback-arg labeller: a [`CallbackArgument`] carries
//! the enclosing call's `(start, end)` offsets so the labeller can
//! identify *which* call in a chained statement
//! (`a.b().c(cb).d(cb)`) a callback belongs to -- every nested
//! `CallExpression` shares its `span.start` with the chain root, so
//! the end offset is what disambiguates.
//!
//! [`CallbackArgument`]: unsnarl_ir::scope::CallbackArgument
//! [`SerializedHeadExpression`]: unsnarl_ir::serialized::SerializedHeadExpression

use unsnarl_ir::primitive::Utf16CodeUnitOffset;
use unsnarl_ir::serialized::SerializedHeadExpression;

pub fn find_call_callee_in_head(
    head: &SerializedHeadExpression,
    call_start: Utf16CodeUnitOffset,
    call_end: Utf16CodeUnitOffset,
) -> Option<&SerializedHeadExpression> {
    match head {
        SerializedHeadExpression::Call {
            callee,
            start_span,
            end_span,
        }
        | SerializedHeadExpression::New {
            callee,
            start_span,
            end_span,
        } => {
            if start_span.offset == call_start && end_span.offset == call_end {
                return Some(callee.as_ref());
            }
            // Continue searching: the matching call may be a nested
            // call inside this call's callee (chained shape) or
            // hidden under a member-chain object further down.
            find_call_callee_in_head(callee, call_start, call_end)
        }
        SerializedHeadExpression::Member { object, .. } => {
            find_call_callee_in_head(object, call_start, call_end)
        }
        SerializedHeadExpression::Await { argument } => {
            find_call_callee_in_head(argument, call_start, call_end)
        }
        // An assignment ExpressionStatement such as `x = a(cb)`
        // wraps the call inside `Assign.right.head` (and, for
        // shapes like `x[a()] = b(cb)`, possibly `Assign.left.head`
        // too). Walk both operands so the callback labeller can
        // still recover the enclosing call's callee subtree.
        SerializedHeadExpression::Assign { left, right, .. } => {
            find_call_callee_in_head(&left.head, call_start, call_end)
                .or_else(|| find_call_callee_in_head(&right.head, call_start, call_end))
        }
        // `++a(...)` is not syntactically valid, but the operand
        // descent costs nothing and keeps the walker symmetric with
        // `Assign` -- any future shape that puts a call under an
        // Update operand stays reachable.
        SerializedHeadExpression::Update { argument, .. } => {
            find_call_callee_in_head(&argument.head, call_start, call_end)
        }
        // `Identifier` / `Elided` / `Raw` carry no nested call/new
        // shape to descend into.
        SerializedHeadExpression::Identifier { .. }
        | SerializedHeadExpression::Elided
        | SerializedHeadExpression::Raw { .. } => None,
    }
}

#[cfg(test)]
#[path = "find_call_callee_in_head_test.rs"]
mod find_call_callee_in_head_test;
