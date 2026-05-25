use super::*;

use crate::prune::prune_fixtures::{const_binding_node, function_subgraph};

#[test]
fn returns_top_level_node_ids() {
    assert_eq!(
        collect_node_ids(&[
            VisualElement::Node(const_binding_node("a", "a", 1)),
            VisualElement::Node(const_binding_node("b", "b", 1)),
        ]),
        vec!["a".to_string(), "b".to_string()]
    );
}

#[test]
fn recurses_into_subgraphs_but_does_not_include_subgraph_ids() {
    assert_eq!(
        collect_node_ids(&[VisualElement::Subgraph(function_subgraph(
            "s",
            1,
            vec![
                VisualElement::Node(const_binding_node("x", "x", 1)),
                VisualElement::Subgraph(function_subgraph(
                    "inner",
                    1,
                    vec![VisualElement::Node(const_binding_node("y", "y", 1))],
                )),
            ],
        ))]),
        vec!["x".to_string(), "y".to_string()]
    );
}

#[test]
fn empty_input_yields_empty_output() {
    assert!(collect_node_ids(&[]).is_empty());
}
