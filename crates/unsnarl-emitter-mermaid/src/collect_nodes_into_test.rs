//! Mirrors `ts/src/emitter/mermaid/collect-nodes-into.test.ts`.

use std::collections::HashMap;

use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_node::{BindingVisualNode, VisualNode};
use unsnarl_visual_graph::visual_subgraph::VisualSubgraph;

use super::collect_nodes_into;
use crate::testing::{base_const_binding, base_function_subgraph};

fn node(id: &str) -> VisualElement {
    VisualElement::Node(VisualNode::Binding(BindingVisualNode {
        id: id.to_string(),
        ..base_const_binding()
    }))
}

fn node_named(id: &str, name: &str) -> VisualElement {
    VisualElement::Node(VisualNode::Binding(BindingVisualNode {
        id: id.to_string(),
        name: name.to_string(),
        ..base_const_binding()
    }))
}

fn subgraph(id: &str, elements: Vec<VisualElement>) -> VisualElement {
    let mut sg = base_function_subgraph();
    sg.id = id.to_string();
    sg.elements = elements;
    VisualElement::Subgraph(VisualSubgraph::Owned(sg))
}

#[test]
fn collects_flat_top_level_nodes_keyed_by_id() {
    let elements = vec![node("a"), node("b")];
    let mut out: HashMap<String, &VisualNode> = HashMap::new();
    collect_nodes_into(&elements, &mut out);
    let mut keys: Vec<String> = out.keys().cloned().collect();
    keys.sort();
    assert_eq!(keys, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn recursively_descends_into_subgraph_elements() {
    let elements = vec![
        node("a"),
        subgraph("s1", vec![node("b"), subgraph("s2", vec![node("c")])]),
    ];
    let mut out: HashMap<String, &VisualNode> = HashMap::new();
    collect_nodes_into(&elements, &mut out);
    let mut keys: Vec<String> = out.keys().cloned().collect();
    keys.sort();
    assert_eq!(
        keys,
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
}

#[test]
fn does_not_add_subgraph_ids() {
    let elements = vec![subgraph("sg", Vec::new())];
    let mut out: HashMap<String, &VisualNode> = HashMap::new();
    collect_nodes_into(&elements, &mut out);
    assert!(out.is_empty());
}

#[test]
fn preserves_the_latest_write_when_ids_collide() {
    let elements = vec![node_named("a", "first"), node_named("a", "second")];
    let mut out: HashMap<String, &VisualNode> = HashMap::new();
    collect_nodes_into(&elements, &mut out);
    assert_eq!(out.get("a").map(|n| n.name()), Some("second"));
}

#[test]
fn empty_input_list_leaves_out_empty() {
    let mut out: HashMap<String, &VisualNode> = HashMap::new();
    collect_nodes_into(&[], &mut out);
    assert!(out.is_empty());
}
