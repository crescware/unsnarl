//! Serialize an in-memory `HeadExpression` (offset-based) into the
//! on-disk `SerializedHeadExpression` (span-based).
//!
//! Mirrors `serializeHeadExpression` in
//! `ts/src/serializer/flat/serialize-expression-statement-head.ts`.

use unsnarl_ir::primitive::span_from_offset;
use unsnarl_ir::reference::expression_statement_head::{HeadExpression, HeadOperand};
use unsnarl_ir::serialized::{SerializedHeadExpression, SerializedHeadOperand};

pub fn serialize_head_expression(head: &HeadExpression, raw: &str) -> SerializedHeadExpression {
    match head {
        HeadExpression::Identifier { name } => SerializedHeadExpression::identifier(name.clone()),
        HeadExpression::Member { object, property } => SerializedHeadExpression::member(
            serialize_head_expression(object, raw),
            property.clone(),
        ),
        HeadExpression::Call { callee } => SerializedHeadExpression::Call {
            callee: Box::new(serialize_head_expression(callee, raw)),
        },
        HeadExpression::New { callee } => SerializedHeadExpression::New {
            callee: Box::new(serialize_head_expression(callee, raw)),
        },
        HeadExpression::Await { argument } => SerializedHeadExpression::Await {
            argument: Box::new(serialize_head_expression(argument, raw)),
        },
        HeadExpression::Assign {
            operator,
            left,
            right,
        } => SerializedHeadExpression::Assign {
            operator: *operator,
            left: Box::new(serialize_head_operand(left, raw)),
            right: Box::new(serialize_head_operand(right, raw)),
        },
        HeadExpression::Update {
            operator,
            prefix,
            argument,
        } => SerializedHeadExpression::Update {
            operator: *operator,
            prefix: *prefix,
            argument: Box::new(serialize_head_operand(argument, raw)),
        },
        HeadExpression::Elided => SerializedHeadExpression::Elided,
        HeadExpression::Raw {
            start_offset,
            end_offset,
        } => SerializedHeadExpression::Raw {
            start_span: span_from_offset(raw, start_offset.0 as usize),
            end_span: span_from_offset(raw, end_offset.0 as usize),
        },
    }
}

fn serialize_head_operand(operand: &HeadOperand, raw: &str) -> SerializedHeadOperand {
    SerializedHeadOperand {
        head: serialize_head_expression(&operand.head, raw),
        start_span: span_from_offset(raw, operand.start_offset.0 as usize),
        end_span: span_from_offset(raw, operand.end_offset.0 as usize),
    }
}
