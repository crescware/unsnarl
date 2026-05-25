use super::*;

use crate::prune::prune_fixtures::{const_binding_node, function_subgraph};

#[test]
fn includes_both_node_and_subgraph_ids() {
    let ids = collect_ids(&[
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
        VisualElement::Node(const_binding_node("top", "top", 1)),
    ]);
    let mut sorted: Vec<String> = ids.into_iter().collect();
    sorted.sort();
    assert_eq!(sorted, vec!["inner", "outer", "top", "x", "y"]);
}

#[test]
fn empty_input_yields_empty_set() {
    assert_eq!(collect_ids(&[]).len(), 0);
}
