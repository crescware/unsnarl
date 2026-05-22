//! Sibling tests for [`find_node_by_id`].

use super::find_node_by_id;
use crate::direction::Direction;
use crate::visual_element::VisualElement;
use crate::visual_element_type::{NodeTypeTag, SubgraphTypeTag};
use crate::visual_node::{BindingExtras, BindingNodeKind, BindingVisualNode, VisualNode};
use crate::visual_subgraph::{
    ControlExtras, ControlSubgraphKind, ControlVisualSubgraph, OwnedExtras, OwnedSubgraphKind,
    OwnedVisualSubgraph, VisualSubgraph,
};

fn leaf(id: &str) -> VisualElement {
    VisualElement::Node(VisualNode::Binding(BindingVisualNode {
        r#type: NodeTypeTag::Node,
        id: id.to_string(),
        name: id.to_string(),
        line: 1,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        kind: BindingNodeKind::ConstBinding,
        extras: BindingExtras::Variable {
            init_is_function: false,
        },
    }))
}

fn function_subgraph(id: &str, elements: Vec<VisualElement>) -> VisualElement {
    VisualElement::Subgraph(VisualSubgraph::Owned(OwnedVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: id.to_string(),
        kind: OwnedSubgraphKind::Function,
        line: 1,
        end_line: None,
        direction: Direction::RL,
        extras: OwnedExtras::Function {
            owner_node_id: Some("n_owner".to_string()),
            owner_name: "owner".to_string(),
        },
        elements,
    }))
}

fn if_subgraph(id: &str, elements: Vec<VisualElement>) -> VisualElement {
    VisualElement::Subgraph(VisualSubgraph::Control(ControlVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: id.to_string(),
        line: 1,
        end_line: None,
        direction: Direction::RL,
        elements,
        kind: ControlSubgraphKind::If,
        extras: ControlExtras::None {},
    }))
}

fn build_elements() -> Vec<VisualElement> {
    let inner = if_subgraph("s2", vec![leaf("c")]);
    let function = function_subgraph("s1", vec![leaf("b"), inner]);
    vec![leaf("a"), function, leaf("d")]
}

#[test]
fn finds_top_level_node() {
    let mut elements = build_elements();
    let found = find_node_by_id(&mut elements, "a").expect("node found");
    assert_eq!(found.id(), "a");
}

#[test]
fn finds_node_one_level_down() {
    let mut elements = build_elements();
    let found = find_node_by_id(&mut elements, "b").expect("node found");
    assert_eq!(found.id(), "b");
}

#[test]
fn finds_node_two_levels_down() {
    let mut elements = build_elements();
    let found = find_node_by_id(&mut elements, "c").expect("node found");
    assert_eq!(found.id(), "c");
}

#[test]
fn finds_top_level_after_nested_subgraph() {
    let mut elements = build_elements();
    let found = find_node_by_id(&mut elements, "d").expect("node found");
    assert_eq!(found.id(), "d");
}

#[test]
fn returns_none_when_id_is_absent() {
    let mut elements = build_elements();
    assert!(find_node_by_id(&mut elements, "missing").is_none());
}

#[test]
fn returns_none_on_empty_element_list() {
    let mut elements: Vec<VisualElement> = Vec::new();
    assert!(find_node_by_id(&mut elements, "a").is_none());
}

#[test]
fn ignores_subgraph_ids() {
    // `find_node_by_id` only matches nodes, never subgraphs.
    let mut elements = build_elements();
    assert!(find_node_by_id(&mut elements, "s1").is_none());
    assert!(find_node_by_id(&mut elements, "s2").is_none());
}
