//! Serialize an in-memory `HeadExpression` (offset-based) into the
//! on-disk `SerializedHeadExpression` (span-based).

use std::sync::atomic::{AtomicU64, Ordering};

use unsnarl_instrumentation::{count_if_verbose, record_elapsed_ns, timing_start, verbose};
use unsnarl_ir::primitive::{SourceIndex, Utf8ByteOffset};
use unsnarl_ir::reference::expression_statement_head::{HeadExpression, HeadOperand};
use unsnarl_ir::serialized::{SerializedHeadExpression, SerializedHeadOperand};

// Module-level counters drained by `take_head_stats`. Counts every
// HeadExpression node visited (recursive total per top-level call),
// every span lookup made inside the head walk, and the accumulated
// time spent inside those lookups. The drained triple is emitted as
// fields of the `serialize_reference sub-phase totals` event, so
// there is no `emit_stats` here; the caller in `serialize_reference`
// folds it into its own combined event.
static N_NODES: AtomicU64 = AtomicU64::new(0);
static N_SPAN_CALLS: AtomicU64 = AtomicU64::new(0);
static T_SPAN_NS: AtomicU64 = AtomicU64::new(0);

/// Drain the accumulators and return `(nodes, span_calls, span_ns)`.
/// No-op (returns zeros) when verbose is off. Called from
/// [`crate::serializer::flat::serialize_reference::emit_stats`].
pub fn emit_head_stats() -> (u64, u64, u64) {
    if !verbose() {
        return (0, 0, 0);
    }
    (
        N_NODES.swap(0, Ordering::Relaxed),
        N_SPAN_CALLS.swap(0, Ordering::Relaxed),
        T_SPAN_NS.swap(0, Ordering::Relaxed),
    )
}

fn timed_span_at(index: &SourceIndex<'_>, offset: Utf8ByteOffset) -> unsnarl_ir::primitive::Span {
    count_if_verbose(&N_SPAN_CALLS, 1);
    let t = timing_start();
    let out = index.span_at(offset);
    record_elapsed_ns(&T_SPAN_NS, t);
    out
}

pub fn serialize_head_expression(
    head: &HeadExpression,
    index: &SourceIndex<'_>,
) -> SerializedHeadExpression {
    count_if_verbose(&N_NODES, 1);
    match head {
        HeadExpression::Identifier { name } => SerializedHeadExpression::identifier(name.clone()),
        HeadExpression::Member { object, property } => SerializedHeadExpression::member(
            serialize_head_expression(object, index),
            property.clone(),
        ),
        HeadExpression::Call { callee } => SerializedHeadExpression::Call {
            callee: Box::new(serialize_head_expression(callee, index)),
        },
        HeadExpression::New { callee } => SerializedHeadExpression::New {
            callee: Box::new(serialize_head_expression(callee, index)),
        },
        HeadExpression::Await { argument } => SerializedHeadExpression::Await {
            argument: Box::new(serialize_head_expression(argument, index)),
        },
        HeadExpression::Assign {
            operator,
            left,
            right,
        } => SerializedHeadExpression::Assign {
            operator: *operator,
            left: Box::new(serialize_head_operand(left, index)),
            right: Box::new(serialize_head_operand(right, index)),
        },
        HeadExpression::Update {
            operator,
            prefix,
            argument,
        } => SerializedHeadExpression::Update {
            operator: *operator,
            prefix: *prefix,
            argument: Box::new(serialize_head_operand(argument, index)),
        },
        HeadExpression::Elided => SerializedHeadExpression::Elided,
        HeadExpression::Raw {
            start_offset,
            end_offset,
        } => SerializedHeadExpression::Raw {
            start_span: timed_span_at(index, *start_offset),
            end_span: timed_span_at(index, *end_offset),
        },
    }
}

fn serialize_head_operand(operand: &HeadOperand, index: &SourceIndex<'_>) -> SerializedHeadOperand {
    SerializedHeadOperand {
        head: serialize_head_expression(&operand.head, index),
        start_span: timed_span_at(index, operand.start_offset),
        end_span: timed_span_at(index, operand.end_offset),
    }
}

#[cfg(test)]
#[path = "serialize_expression_statement_head_test.rs"]
mod serialize_expression_statement_head_test;
