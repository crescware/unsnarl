use super::*;

use crate::prune::test_helpers::{const_binding_node, function_subgraph};

fn keep(ids: &[&str]) -> HashSet<String> {
    ids.iter().map(|s| s.to_string()).collect()
}

#[test]
fn keeps_only_nodes_whose_id_is_in_the_keep_set() {
    let out = rebuild_elements(
        &[
            VisualElement::Node(const_binding_node("a", "a", 1)),
            VisualElement::Node(const_binding_node("b", "b", 1)),
            VisualElement::Node(const_binding_node("c", "c", 1)),
        ],
        &keep(&["a", "c"]),
    );
    assert_eq!(
        out.iter().map(|v| v.id().to_string()).collect::<Vec<_>>(),
        vec!["a", "c"]
    );
}

#[test]
fn subgraph_survives_only_when_at_least_one_descendant_survives() {
    let out = rebuild_elements(
        &[
            VisualElement::Subgraph(function_subgraph(
                "s",
                1,
                vec![
                    VisualElement::Node(const_binding_node("x", "x", 1)),
                    VisualElement::Node(const_binding_node("y", "y", 1)),
                ],
            )),
            VisualElement::Subgraph(function_subgraph(
                "t",
                1,
                vec![VisualElement::Node(const_binding_node("z", "z", 1))],
            )),
        ],
        &keep(&["x"]),
    );
    assert_eq!(
        out.iter().map(|v| v.id().to_string()).collect::<Vec<_>>(),
        vec!["s"]
    );
    let VisualElement::Subgraph(s) = &out[0] else {
        panic!("expected subgraph");
    };
    assert_eq!(
        s.elements()
            .iter()
            .map(|v| v.id().to_string())
            .collect::<Vec<_>>(),
        vec!["x"]
    );
}

#[test]
fn subgraph_with_zero_surviving_descendants_is_dropped() {
    let out = rebuild_elements(
        &[VisualElement::Subgraph(function_subgraph(
            "empty",
            1,
            vec![VisualElement::Node(const_binding_node("x", "x", 1))],
        ))],
        &HashSet::new(),
    );
    assert!(out.is_empty());
}

#[test]
fn nested_subgraphs_are_reconstructed_independently() {
    let out = rebuild_elements(
        &[VisualElement::Subgraph(function_subgraph(
            "outer",
            1,
            vec![
                VisualElement::Subgraph(function_subgraph(
                    "inner",
                    1,
                    vec![VisualElement::Node(const_binding_node("deep", "deep", 1))],
                )),
                VisualElement::Node(const_binding_node("mid", "mid", 1)),
            ],
        ))],
        &keep(&["deep"]),
    );
    assert_eq!(out.len(), 1);
    let VisualElement::Subgraph(outer) = &out[0] else {
        panic!("expected subgraph");
    };
    assert_eq!(outer.elements().len(), 1);
    let VisualElement::Subgraph(inner) = &outer.elements()[0] else {
        panic!("expected subgraph");
    };
    assert_eq!(inner.id(), "inner");
    assert_eq!(
        inner
            .elements()
            .iter()
            .map(|v| v.id().to_string())
            .collect::<Vec<_>>(),
        vec!["deep"]
    );
}
