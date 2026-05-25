use super::*;

use crate::prune::prune_fixtures::{const_binding_node, function_subgraph};
use crate::visual_element::VisualElement;

#[test]
fn top_level_elements_have_no_parent_entry() {
    let map = build_parent_map(&[
        VisualElement::Node(const_binding_node("a", "a", 1)),
        VisualElement::Node(const_binding_node("b", "b", 1)),
    ]);
    assert_eq!(map.len(), 0);
}

#[test]
fn each_child_of_a_subgraph_maps_to_that_subgraph_id() {
    let map = build_parent_map(&[VisualElement::Subgraph(function_subgraph(
        "s",
        1,
        vec![
            VisualElement::Node(const_binding_node("x", "x", 1)),
            VisualElement::Node(const_binding_node("y", "y", 1)),
        ],
    ))]);
    assert_eq!(map.get("x"), Some(&"s".to_string()));
    assert_eq!(map.get("y"), Some(&"s".to_string()));
    assert!(!map.contains_key("s"));
}

#[test]
fn nested_subgraphs_chain_correctly() {
    let map = build_parent_map(&[VisualElement::Subgraph(function_subgraph(
        "outer",
        1,
        vec![VisualElement::Subgraph(function_subgraph(
            "inner",
            1,
            vec![VisualElement::Node(const_binding_node("deep", "deep", 1))],
        ))],
    ))]);
    assert_eq!(map.get("inner"), Some(&"outer".to_string()));
    assert_eq!(map.get("deep"), Some(&"inner".to_string()));
}
