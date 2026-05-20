use super::*;

use crate::prune::test_helpers::{const_binding_node, function_subgraph};
use crate::visual_element_type::NodeTypeTag;
use crate::visual_node::{SyntheticExtras, SyntheticNodeKind, SyntheticVisualNode};

fn non_candidate_node(id: &str) -> VisualNode {
    // SyntheticBeyondDepth is not in ROOT_CANDIDATE_KINDS, so it
    // must not be yielded.
    VisualNode::Synthetic(SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id: id.to_string(),
        kind: SyntheticNodeKind::SyntheticBeyondDepth,
        name: id.to_string(),
        line: 1,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::None {},
    })
}

#[test]
fn yields_only_root_candidate_kinds() {
    let elements = [
        VisualElement::Node(const_binding_node("a", "a", 1)),
        VisualElement::Node(non_candidate_node("b")),
    ];
    let out = collect_root_candidates(&elements);
    assert_eq!(out.iter().map(|v| v.id()).collect::<Vec<_>>(), vec!["a"]);
}

#[test]
fn recurses_into_subgraphs() {
    let elements = [
        VisualElement::Subgraph(function_subgraph(
            "s",
            1,
            vec![
                VisualElement::Node(const_binding_node("inner", "inner", 1)),
                VisualElement::Subgraph(function_subgraph(
                    "s2",
                    1,
                    vec![VisualElement::Node(const_binding_node("deep", "deep", 1))],
                )),
            ],
        )),
        VisualElement::Node(const_binding_node("top", "top", 1)),
    ];
    let out = collect_root_candidates(&elements);
    assert_eq!(
        out.iter().map(|v| v.id()).collect::<Vec<_>>(),
        vec!["inner", "deep", "top"]
    );
}

#[test]
fn empty_input_yields_empty_output() {
    let empty: [VisualElement; 0] = [];
    assert!(collect_root_candidates(&empty).is_empty());
}
