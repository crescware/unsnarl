use super::*;

use crate::prune::test_helpers::{const_binding_node, function_subgraph};

#[test]
fn yields_nothing_for_plain_nodes() {
    let elements = [VisualElement::Node(const_binding_node("a", "a", 1))];
    let out = collect_subgraphs(&elements);
    assert!(out.is_empty());
}

#[test]
fn yields_each_subgraph_in_pre_order() {
    let elements = [
        VisualElement::Subgraph(function_subgraph(
            "outer",
            1,
            vec![
                VisualElement::Node(const_binding_node("x", "x", 1)),
                VisualElement::Subgraph(function_subgraph(
                    "inner",
                    1,
                    vec![VisualElement::Node(const_binding_node("y", "y", 1))],
                )),
            ],
        )),
        VisualElement::Subgraph(function_subgraph("sibling", 1, vec![])),
    ];
    let out = collect_subgraphs(&elements);
    assert_eq!(
        out.iter().map(|v| v.id()).collect::<Vec<_>>(),
        vec!["outer", "inner", "sibling"]
    );
}

#[test]
fn empty_input_yields_empty_output() {
    let empty: [VisualElement; 0] = [];
    assert!(collect_subgraphs(&empty).is_empty());
}
