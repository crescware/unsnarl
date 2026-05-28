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

pub fn find_call_callee_in_head<'a>(
    head: &'a SerializedHeadExpression,
    call_start: Utf16CodeUnitOffset,
    call_end: Utf16CodeUnitOffset,
) -> Option<&'a SerializedHeadExpression> {
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
        // `Assign` / `Update` / `Identifier` / `Elided` / `Raw` carry
        // no nested call/new shape to descend into.
        _ => None,
    }
}

#[cfg(test)]
#[path = "find_call_callee_in_head_test.rs"]
mod find_call_callee_in_head_test;
