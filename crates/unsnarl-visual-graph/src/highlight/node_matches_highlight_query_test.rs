use super::*;

use unsnarl_ir::SourceLine;

use crate::visual_element_type::NodeTypeTag;
use crate::visual_node::{
    BindingExtras, BindingNodeKind, BindingVisualNode, SyntheticExtras, SyntheticNodeKind,
    SyntheticVisualNode, VisualNode,
};

fn variable(name: &str, line: u32, end_line: Option<u32>) -> VisualNode {
    VisualNode::Binding(BindingVisualNode {
        r#type: NodeTypeTag::Node,
        id: "n".to_string(),
        name: name.to_string(),
        line,
        end_line,
        is_jsx_element: false,
        unused: false,
        kind: BindingNodeKind::ConstBinding,
        extras: BindingExtras::Variable {
            init_is_function: false,
        },
    })
}

fn return_use(name: &str, line: u32) -> VisualNode {
    VisualNode::Synthetic(SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id: "n".to_string(),
        kind: SyntheticNodeKind::ReturnArgumentReference,
        name: name.to_string(),
        line,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::None {},
    })
}

fn write_op(name: &str, line: u32) -> VisualNode {
    VisualNode::Synthetic(SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id: "n".to_string(),
        kind: SyntheticNodeKind::WriteReference,
        name: name.to_string(),
        line,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::WriteOp {
            declaration_kind: None,
        },
    })
}

#[test]
fn line_query_matches_when_query_line_falls_in_node_range() {
    let q = ParsedRootQuery::Line {
        line: SourceLine(5),
        raw: "5".to_string(),
    };
    assert!(node_matches_highlight_query(&variable("x", 5, None), &q));
    assert!(node_matches_highlight_query(&variable("x", 3, Some(7)), &q));
    let q6 = ParsedRootQuery::Line {
        line: SourceLine(6),
        raw: "6".to_string(),
    };
    assert!(!node_matches_highlight_query(&variable("x", 5, None), &q6));
}

#[test]
fn line_name_requires_both_line_membership_and_name() {
    let q = ParsedRootQuery::LineName {
        line: SourceLine(5),
        name: "x".to_string(),
        raw: "5:x".to_string(),
    };
    assert!(node_matches_highlight_query(&variable("x", 5, None), &q));
    assert!(!node_matches_highlight_query(&variable("y", 5, None), &q));
    assert!(!node_matches_highlight_query(&variable("x", 6, None), &q));
}

#[test]
fn range_query_treats_node_spans_inclusively() {
    let q = ParsedRootQuery::Range {
        start: SourceLine(3),
        end: SourceLine(7),
        raw: "3-7".to_string(),
    };
    assert!(node_matches_highlight_query(&variable("x", 5, None), &q));
    assert!(node_matches_highlight_query(
        &variable("x", 7, Some(10)),
        &q
    ));
    assert!(!node_matches_highlight_query(&variable("x", 8, None), &q));
}

#[test]
fn range_name_query_requires_range_overlap_and_name() {
    let q = ParsedRootQuery::RangeName {
        start: SourceLine(3),
        end: SourceLine(7),
        name: "x".to_string(),
        raw: "3-7:x".to_string(),
    };
    assert!(node_matches_highlight_query(&variable("x", 5, None), &q));
    assert!(!node_matches_highlight_query(&variable("y", 5, None), &q));
    assert!(!node_matches_highlight_query(&variable("x", 8, None), &q));
}

#[test]
fn name_query_matches_write_op_and_return_use_unlike_prune() {
    let q = ParsedRootQuery::Name {
        name: "counter".to_string(),
        raw: "counter".to_string(),
    };
    assert!(node_matches_highlight_query(
        &variable("counter", 1, None),
        &q
    ));
    assert!(node_matches_highlight_query(&write_op("counter", 2), &q));
    assert!(node_matches_highlight_query(&return_use("counter", 3), &q));
    assert!(!node_matches_highlight_query(
        &variable("other", 1, None),
        &q
    ));
}

#[test]
fn line_or_name_is_unreachable_post_resolution_and_returns_false() {
    let q = ParsedRootQuery::LineOrName {
        line: SourceLine(5),
        name: "L5".to_string(),
        raw: "L5".to_string(),
    };
    assert!(!node_matches_highlight_query(&variable("L5", 5, None), &q));
}
