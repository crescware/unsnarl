//! Pins the rules:
//!   - Pure-recursive shapes (Identifier / Member / Call / New /
//!     Await) round-trip verbatim.
//!   - Assign / Update / Raw shapes convert their operand /
//!     position offsets to `Span(line/column/offset)` via
//!     `span_from_offset`.
//!   - `Elided` operands keep their span attached so the operand is
//!     still locatable in the source.
//!
//! `SerializedHeadExpression` does not derive `PartialEq`, so the
//! tests compare the serde-serialised JSON for shape + ordering.

use unsnarl_ir::primitive::{SourceIndex, SourceOffset};
use unsnarl_ir::reference::expression_statement_head::{HeadExpression, HeadOperand};
use unsnarl_oxc_parity::{AssignOperator, UpdateOperator};

use super::serialize_head_expression;

fn json(s: &impl serde::Serialize) -> serde_json::Value {
    serde_json::to_value(s).expect("serialize")
}

#[test]
fn passes_an_identifier_head_through_unchanged() {
    let head = HeadExpression::identifier("x".to_string());
    assert_eq!(
        json(&serialize_head_expression(&head, &SourceIndex::build(""))),
        serde_json::json!({"kind": "identifier", "name": "x"})
    );
}

#[test]
fn recurses_through_a_member_head_and_keeps_the_property_name() {
    let head = HeadExpression::member(
        HeadExpression::identifier("fns".to_string()),
        "push".to_string(),
    );
    assert_eq!(
        json(&serialize_head_expression(&head, &SourceIndex::build(""))),
        serde_json::json!({
            "kind": "member",
            "object": {"kind": "identifier", "name": "fns"},
            "property": "push"
        })
    );
}

#[test]
fn recurses_through_a_call_head() {
    let head = HeadExpression::Call {
        callee: Box::new(HeadExpression::identifier("foo".to_string())),
    };
    assert_eq!(
        json(&serialize_head_expression(&head, &SourceIndex::build(""))),
        serde_json::json!({
            "kind": "call",
            "callee": {"kind": "identifier", "name": "foo"}
        })
    );
}

#[test]
fn recurses_through_a_new_head() {
    let head = HeadExpression::New {
        callee: Box::new(HeadExpression::identifier("C".to_string())),
    };
    assert_eq!(
        json(&serialize_head_expression(&head, &SourceIndex::build(""))),
        serde_json::json!({
            "kind": "new",
            "callee": {"kind": "identifier", "name": "C"}
        })
    );
}

#[test]
fn recurses_through_an_await_head() {
    let head = HeadExpression::Await {
        argument: Box::new(HeadExpression::Call {
            callee: Box::new(HeadExpression::identifier("go".to_string())),
        }),
    };
    assert_eq!(
        json(&serialize_head_expression(&head, &SourceIndex::build(""))),
        serde_json::json!({
            "kind": "await",
            "argument": {
                "kind": "call",
                "callee": {"kind": "identifier", "name": "go"}
            }
        })
    );
}

#[test]
fn recurses_through_an_assign_head_and_converts_each_operands_offsets_to_spans() {
    let raw = "C.z = v";
    let head = HeadExpression::Assign {
        operator: AssignOperator::Assign,
        left: Box::new(HeadOperand {
            head: HeadExpression::member(
                HeadExpression::identifier("C".to_string()),
                "z".to_string(),
            ),
            start_offset: SourceOffset(0),
            end_offset: SourceOffset(3),
        }),
        right: Box::new(HeadOperand {
            head: HeadExpression::identifier("v".to_string()),
            start_offset: SourceOffset(6),
            end_offset: SourceOffset(7),
        }),
    };
    assert_eq!(
        json(&serialize_head_expression(&head, &SourceIndex::build(raw))),
        serde_json::json!({
            "kind": "assign",
            "operator": "=",
            "left": {
                "head": {
                    "kind": "member",
                    "object": {"kind": "identifier", "name": "C"},
                    "property": "z"
                },
                "startSpan": {"offset": 0, "line": 1, "column": 0},
                "endSpan": {"offset": 3, "line": 1, "column": 3}
            },
            "right": {
                "head": {"kind": "identifier", "name": "v"},
                "startSpan": {"offset": 6, "line": 1, "column": 6},
                "endSpan": {"offset": 7, "line": 1, "column": 7}
            }
        })
    );
}

#[test]
fn preserves_the_span_on_an_elided_assign_operand_through_serialization() {
    // The elided side has no structural position of its own, so the
    // operand's span IS the only locator for that side. The serializer
    // must keep it intact and well-formed.
    let raw = "C.z = 1";
    let head = HeadExpression::Assign {
        operator: AssignOperator::Assign,
        left: Box::new(HeadOperand {
            head: HeadExpression::member(
                HeadExpression::identifier("C".to_string()),
                "z".to_string(),
            ),
            start_offset: SourceOffset(0),
            end_offset: SourceOffset(3),
        }),
        right: Box::new(HeadOperand {
            head: HeadExpression::Elided,
            start_offset: SourceOffset(6),
            end_offset: SourceOffset(7),
        }),
    };
    assert_eq!(
        json(&serialize_head_expression(&head, &SourceIndex::build(raw))),
        serde_json::json!({
            "kind": "assign",
            "operator": "=",
            "left": {
                "head": {
                    "kind": "member",
                    "object": {"kind": "identifier", "name": "C"},
                    "property": "z"
                },
                "startSpan": {"offset": 0, "line": 1, "column": 0},
                "endSpan": {"offset": 3, "line": 1, "column": 3}
            },
            "right": {
                "head": {"kind": "elided"},
                "startSpan": {"offset": 6, "line": 1, "column": 6},
                "endSpan": {"offset": 7, "line": 1, "column": 7}
            }
        })
    );
}

#[test]
fn recurses_through_an_update_head_and_keeps_operator_prefix_argument_span() {
    let raw = "++C.z;";
    let head = HeadExpression::Update {
        operator: UpdateOperator::Increment,
        prefix: true,
        argument: Box::new(HeadOperand {
            head: HeadExpression::member(
                HeadExpression::identifier("C".to_string()),
                "z".to_string(),
            ),
            start_offset: SourceOffset(2),
            end_offset: SourceOffset(5),
        }),
    };
    assert_eq!(
        json(&serialize_head_expression(&head, &SourceIndex::build(raw))),
        serde_json::json!({
            "kind": "update",
            "operator": "++",
            "prefix": true,
            "argument": {
                "head": {
                    "kind": "member",
                    "object": {"kind": "identifier", "name": "C"},
                    "property": "z"
                },
                "startSpan": {"offset": 2, "line": 1, "column": 2},
                "endSpan": {"offset": 5, "line": 1, "column": 5}
            }
        })
    );
}

#[test]
fn converts_a_raw_heads_offsets_to_spans_against_the_original_source() {
    let raw = "line1\nline2 += 1;";
    let head = HeadExpression::Raw {
        start_offset: SourceOffset(6),
        end_offset: SourceOffset(16),
    };
    assert_eq!(
        json(&serialize_head_expression(&head, &SourceIndex::build(raw))),
        serde_json::json!({
            "kind": "raw",
            "startSpan": {"offset": 6, "line": 2, "column": 0},
            "endSpan": {"offset": 16, "line": 2, "column": 10}
        })
    );
}
