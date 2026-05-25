//! Pins the preorder walk: top-level nodes come through verbatim,
//! subgraphs flatten their `elements` into the same stream, and
//! empty subgraphs contribute nothing.

use unsnarl_visual_graph::direction::Direction;
use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_node::{BindingVisualNode, VisualNode};
use unsnarl_visual_graph::visual_subgraph::OwnedVisualSubgraph;

use super::collect_nodes;

fn node(id: &str, line: u32) -> VisualElement {
    VisualElement::from(VisualNode::from(BindingVisualNode::const_binding(
        id, id, line,
    )))
}

fn sg(id: &str, elements: Vec<VisualElement>) -> VisualElement {
    VisualElement::from(unsnarl_visual_graph::visual_subgraph::VisualSubgraph::from(
        OwnedVisualSubgraph::function(
            id,
            1,
            Some("n_owner".to_string()),
            "owner",
            elements,
            Direction::TB,
        ),
    ))
}

fn ids(out: &[&VisualNode]) -> Vec<String> {
    out.iter().map(|n| n.id().to_string()).collect()
}

#[test]
fn returns_top_level_nodes_verbatim() {
    let input = vec![node("a", 1), node("b", 2)];
    assert_eq!(
        ids(&collect_nodes(&input)),
        vec!["a".to_string(), "b".to_string()]
    );
}

#[test]
fn flattens_one_level_of_subgraph_nesting() {
    let input = vec![
        node("a", 1),
        sg("s", vec![node("b", 1), node("c", 1)]),
        node("d", 1),
    ];
    assert_eq!(
        ids(&collect_nodes(&input)),
        vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string()
        ]
    );
}

#[test]
fn flattens_recursively_across_multiple_levels() {
    let input = vec![
        sg("s1", vec![sg("s2", vec![node("deep", 1)]), node("mid", 1)]),
        node("top", 1),
    ];
    assert_eq!(
        ids(&collect_nodes(&input)),
        vec!["deep".to_string(), "mid".to_string(), "top".to_string()]
    );
}

#[test]
fn empty_input_to_empty_output() {
    let input: Vec<VisualElement> = Vec::new();
    assert!(collect_nodes(&input).is_empty());
}

#[test]
fn subgraph_with_no_node_descendants_contributes_nothing() {
    let input = vec![sg("s", vec![sg("inner", vec![])])];
    assert!(collect_nodes(&input).is_empty());
}
