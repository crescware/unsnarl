//! Sibling tests for [`ensure_expression_statement_node`].

use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::serialized::SerializedExpressionStatementContainer;
use unsnarl_ir::serialized::SerializedHeadExpression;

use super::ensure_expression_statement_node;
use crate::builder::arena::{BuildArena, Container, ElementHandle};
use crate::builder::builder_fixtures::{base_serialized_reference, reference_id, span_offset_line};
use crate::builder::state::BuildState;
use crate::visual_element::VisualElement;
use crate::visual_node::{SyntheticNodeKind, VisualNode};
use unsnarl_ir::serialized::SerializedReference;

fn ref_with_head(
    ref_id_str: &str,
    start_offset: u32,
    end_offset: u32,
    start_line: u32,
    end_line: u32,
    head: SerializedHeadExpression,
) -> SerializedReference {
    let mut r = base_serialized_reference();
    r.id = reference_id(ref_id_str);
    r.identifier = unsnarl_ir::serialized::SerializedReferenceIdentifier::new(
        "x".to_string(),
        span_offset_line(0, 1),
    );
    r.expression_statement_container = Some(SerializedExpressionStatementContainer {
        start_span: span_offset_line(start_offset, start_line),
        end_span: span_offset_line(end_offset, end_line),
        head,
        expression_start_span: None,
    });
    r
}

fn console_log_head() -> SerializedHeadExpression {
    SerializedHeadExpression::Call {
        callee: Box::new(SerializedHeadExpression::Member {
            object: Box::new(SerializedHeadExpression::Identifier {
                name: "console".to_string(),
            }),
            property: "log".to_string(),
        }),
        start_span: span_offset_line(0, 1),
        end_span: span_offset_line(14, 1),
    }
}

fn finalize_one_node(arena: BuildArena) -> Option<VisualNode> {
    let elements = arena.finalize_root();
    if elements.is_empty() {
        return None;
    }
    match elements
        .into_iter()
        .next()
        .expect("test fixture produces at least one element")
    {
        VisualElement::Node(n) => Some(n),
        _ => None,
    }
}

#[test]
fn returns_none_when_ref_has_no_expression_statement_container() {
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let r = base_serialized_reference();
    let id = ensure_expression_statement_node(
        &mut arena,
        &mut state,
        &r,
        &SourceIndex::build(""),
        Container::Root,
    );
    assert_eq!(id, None);
    assert!(arena.root_children.is_empty());
}

#[test]
fn renders_call_member_identifier_to_receiver_dot_property() {
    let r = ref_with_head("r1", 0, 15, 7, 7, console_log_head());
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let id = ensure_expression_statement_node(
        &mut arena,
        &mut state,
        &r,
        &SourceIndex::build("console.log(a);"),
        Container::Root,
    );
    assert_eq!(id.as_deref(), Some("expr_stmt_0"));
    let node = finalize_one_node(arena).expect("one node");
    let VisualNode::Synthetic(s) = node else {
        panic!("expected synthetic");
    };
    assert_eq!(s.id, "expr_stmt_0");
    assert!(matches!(
        s.kind,
        SyntheticNodeKind::SyntheticExpressionStatement
    ));
    assert_eq!(s.name, "console.log()");
    assert_eq!(s.line, 7);
}

#[test]
fn renders_bare_identifier_head_without_parens() {
    let r = ref_with_head(
        "r1",
        0,
        2,
        2,
        2,
        SerializedHeadExpression::Identifier {
            name: "a".to_string(),
        },
    );
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    ensure_expression_statement_node(
        &mut arena,
        &mut state,
        &r,
        &SourceIndex::build("a;"),
        Container::Root,
    );
    let VisualNode::Synthetic(s) = finalize_one_node(arena).expect("node") else {
        panic!("expected synthetic");
    };
    assert_eq!(s.name, "a");
}

#[test]
fn renders_awaited_chain() {
    let head = SerializedHeadExpression::Await {
        argument: Box::new(SerializedHeadExpression::Call {
            callee: Box::new(SerializedHeadExpression::Member {
                object: Box::new(SerializedHeadExpression::Call {
                    callee: Box::new(SerializedHeadExpression::Member {
                        object: Box::new(SerializedHeadExpression::Identifier {
                            name: "Promise".to_string(),
                        }),
                        property: "resolve".to_string(),
                    }),
                    start_span: span_offset_line(6, 1),
                    end_span: span_offset_line(23, 1),
                }),
                property: "then".to_string(),
            }),
            start_span: span_offset_line(6, 1),
            end_span: span_offset_line(31, 1),
        }),
    };
    let r = ref_with_head("r1", 0, 50, 1, 5, head);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    ensure_expression_statement_node(
        &mut arena,
        &mut state,
        &r,
        &SourceIndex::build(""),
        Container::Root,
    );
    let VisualNode::Synthetic(s) = finalize_one_node(arena).expect("node") else {
        panic!("expected synthetic");
    };
    assert_eq!(s.name, "await Promise.resolve().then()");
}

#[test]
fn slices_original_source_for_raw_head() {
    let head = SerializedHeadExpression::Raw {
        start_span: span_offset_line(0, 3),
        end_span: span_offset_line(5, 3),
    };
    let r = ref_with_head("r1", 0, 6, 3, 3, head);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    ensure_expression_statement_node(
        &mut arena,
        &mut state,
        &r,
        &SourceIndex::build("x = 1;"),
        Container::Root,
    );
    let VisualNode::Synthetic(s) = finalize_one_node(arena).expect("node") else {
        panic!("expected synthetic");
    };
    assert_eq!(s.name, "x = 1");
}

#[test]
fn sets_end_line_when_statement_spans_multiple_lines() {
    let r = ref_with_head("r1", 0, 20, 1, 3, console_log_head());
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    ensure_expression_statement_node(
        &mut arena,
        &mut state,
        &r,
        &SourceIndex::build("console.log(\n  a,\n);"),
        Container::Root,
    );
    let VisualNode::Synthetic(s) = finalize_one_node(arena).expect("node") else {
        panic!("expected synthetic");
    };
    assert_eq!(s.line, 1);
    assert_eq!(s.end_line, Some(3));
}

#[test]
fn returns_cached_id_and_does_not_re_append_for_refs_in_same_statement() {
    let ref_a = ref_with_head("r1", 0, 15, 7, 7, console_log_head());
    let ref_b = ref_with_head("r2", 0, 15, 7, 7, console_log_head());
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let index = SourceIndex::build("console.log(a);");
    let id_a =
        ensure_expression_statement_node(&mut arena, &mut state, &ref_a, &index, Container::Root);
    let id_b =
        ensure_expression_statement_node(&mut arena, &mut state, &ref_b, &index, Container::Root);
    assert_eq!(id_a, id_b);
    // Single node appended to root.
    assert_eq!(arena.root_children.len(), 1);
    assert!(matches!(arena.root_children[0], ElementHandle::Node(_)));
}
