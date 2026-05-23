//! Sibling tests for [`render_head_expression`], covering every
//! head shape: identifier, member, call, new, await, assign, update
//! (prefix + postfix), elided, raw.

use unsnarl_ir::primitive::{SourceColumn, SourceIndex, SourceLine, Span, Utf16CodeUnitOffset};
use unsnarl_ir::serialized::{SerializedHeadExpression, SerializedHeadOperand};
use unsnarl_oxc_parity::{AssignOperator, UpdateOperator};

use super::render_head_expression;

fn span_at(line: u32, column: u32, offset: u32) -> Span {
    Span {
        line: SourceLine(line),
        column: SourceColumn(column),
        offset: Utf16CodeUnitOffset(offset),
    }
}

#[test]
fn identifier_renders_as_name() {
    let head = SerializedHeadExpression::identifier("foo".to_string());
    assert_eq!(
        render_head_expression(&head, &SourceIndex::build("")),
        "foo"
    );
}

#[test]
fn member_renders_with_dot() {
    let head = SerializedHeadExpression::member(
        SerializedHeadExpression::identifier("console".to_string()),
        "log".to_string(),
    );
    assert_eq!(
        render_head_expression(&head, &SourceIndex::build("")),
        "console.log"
    );
}

#[test]
fn call_appends_parens() {
    let head = SerializedHeadExpression::Call {
        callee: Box::new(SerializedHeadExpression::identifier("run".to_string())),
    };
    assert_eq!(
        render_head_expression(&head, &SourceIndex::build("")),
        "run()"
    );
}

#[test]
fn new_renders_with_new_keyword() {
    let head = SerializedHeadExpression::New {
        callee: Box::new(SerializedHeadExpression::identifier("Set".to_string())),
    };
    assert_eq!(
        render_head_expression(&head, &SourceIndex::build("")),
        "new Set()"
    );
}

#[test]
fn await_renders_with_await_keyword() {
    let head = SerializedHeadExpression::Await {
        argument: Box::new(SerializedHeadExpression::identifier("p".to_string())),
    };
    assert_eq!(
        render_head_expression(&head, &SourceIndex::build("")),
        "await p"
    );
}

#[test]
fn assign_inserts_operator_with_spaces() {
    let head = SerializedHeadExpression::Assign {
        operator: AssignOperator::AddAssign,
        left: Box::new(SerializedHeadOperand {
            head: SerializedHeadExpression::identifier("x".to_string()),
            start_span: span_at(1, 0, 0),
            end_span: span_at(1, 1, 1),
        }),
        right: Box::new(SerializedHeadOperand {
            head: SerializedHeadExpression::Elided,
            start_span: span_at(1, 5, 5),
            end_span: span_at(1, 6, 6),
        }),
    };
    assert_eq!(
        render_head_expression(&head, &SourceIndex::build("")),
        "x += ..."
    );
}

#[test]
fn prefix_update_prepends_operator() {
    let head = SerializedHeadExpression::Update {
        operator: UpdateOperator::Increment,
        prefix: true,
        argument: Box::new(SerializedHeadOperand {
            head: SerializedHeadExpression::identifier("i".to_string()),
            start_span: span_at(1, 0, 0),
            end_span: span_at(1, 1, 1),
        }),
    };
    assert_eq!(
        render_head_expression(&head, &SourceIndex::build("")),
        "++i"
    );
}

#[test]
fn postfix_update_appends_operator() {
    let head = SerializedHeadExpression::Update {
        operator: UpdateOperator::Decrement,
        prefix: false,
        argument: Box::new(SerializedHeadOperand {
            head: SerializedHeadExpression::identifier("i".to_string()),
            start_span: span_at(1, 0, 0),
            end_span: span_at(1, 1, 1),
        }),
    };
    assert_eq!(
        render_head_expression(&head, &SourceIndex::build("")),
        "i--"
    );
}

#[test]
fn elided_collapses_to_ellipsis() {
    let head = SerializedHeadExpression::Elided;
    assert_eq!(
        render_head_expression(&head, &SourceIndex::build("")),
        "..."
    );
}

#[test]
fn raw_slices_the_source_between_offsets() {
    let raw = "abc(def)";
    let head = SerializedHeadExpression::Raw {
        start_span: span_at(1, 0, 0),
        end_span: span_at(1, 8, 8),
    };
    assert_eq!(
        render_head_expression(&head, &SourceIndex::build(raw)),
        "abc(def)"
    );
}

#[test]
fn raw_slice_respects_utf16_offsets_for_non_ascii_source() {
    // Em-dash (U+2014) sits inside the BMP (one UTF-16 unit, three
    // UTF-8 bytes). The first call slices offsets 0..1 — the source
    // identifier `a` — and the second slices 1..4, the em-dash plus
    // the trailing identifier.
    let raw = "a\u{2014}b)";
    let head_a = SerializedHeadExpression::Raw {
        start_span: span_at(1, 0, 0),
        end_span: span_at(1, 1, 1),
    };
    assert_eq!(
        render_head_expression(&head_a, &SourceIndex::build(raw)),
        "a"
    );
    let head_b = SerializedHeadExpression::Raw {
        start_span: span_at(1, 1, 1),
        end_span: span_at(1, 3, 3),
    };
    assert_eq!(
        render_head_expression(&head_b, &SourceIndex::build(raw)),
        "\u{2014}b"
    );
}
