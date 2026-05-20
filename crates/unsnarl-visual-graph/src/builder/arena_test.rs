//! Tests for [`super::arena`].
//!
//! Cover the round-trip from arena handle tree to `Vec<VisualElement>`
//! since the rest of the builder relies on that invariant. Field
//! order of the rendered subgraphs is checked separately by the
//! sibling tests at the crate root (`visual_subgraph_test.rs`).

use super::{BuildArena, Container, ElementHandle};
use crate::direction::Direction;
use crate::visual_element::VisualElement;
use crate::visual_element_type::{NodeTypeTag, SubgraphTypeTag};
use crate::visual_node::{
    BindingExtras, BindingNodeKind, BindingVisualNode, SyntheticExtras, SyntheticNodeKind,
    SyntheticVisualNode, VisualNode,
};
use crate::visual_subgraph::{
    ControlExtras, ControlSubgraphKind, ControlVisualSubgraph, OwnedExtras, OwnedSubgraphKind,
    OwnedVisualSubgraph, VisualSubgraph,
};

fn binding_node(id: &str, name: &str) -> VisualNode {
    VisualNode::Binding(BindingVisualNode {
        r#type: NodeTypeTag::Node,
        id: id.to_string(),
        name: name.to_string(),
        line: 1,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        kind: BindingNodeKind::ConstBinding,
        extras: BindingExtras::Variable {
            init_is_function: false,
        },
    })
}

fn synthetic_node(id: &str, name: &str, kind: SyntheticNodeKind) -> VisualNode {
    VisualNode::Synthetic(SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id: id.to_string(),
        kind,
        name: name.to_string(),
        line: 1,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::None {},
    })
}

fn owned_subgraph(id: &str) -> VisualSubgraph {
    VisualSubgraph::Owned(OwnedVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: id.to_string(),
        kind: OwnedSubgraphKind::Function,
        line: 1,
        end_line: Some(3),
        direction: Direction::RL,
        extras: OwnedExtras::Function {
            owner_node_id: None,
            owner_name: "fn".to_string(),
        },
        elements: Vec::new(),
    })
}

fn control_subgraph(id: &str) -> VisualSubgraph {
    VisualSubgraph::Control(ControlVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: id.to_string(),
        line: 1,
        end_line: Some(2),
        direction: Direction::RL,
        elements: Vec::new(),
        kind: ControlSubgraphKind::Block,
        extras: ControlExtras::None {},
    })
}

#[test]
fn finalize_returns_root_children_in_push_order() {
    let mut arena = BuildArena::new();
    let n_a = arena.push_node(binding_node("a", "a"));
    let n_b = arena.push_node(binding_node("b", "b"));
    arena.append_child(Container::Root, ElementHandle::Node(n_a));
    arena.append_child(Container::Root, ElementHandle::Node(n_b));
    let rendered = arena.finalize_root();
    assert_eq!(rendered.len(), 2);
    assert_eq!(rendered[0].id(), "a");
    assert_eq!(rendered[1].id(), "b");
}

#[test]
fn finalize_rehydrates_subgraph_elements() {
    let mut arena = BuildArena::new();
    let sg = arena.push_subgraph(owned_subgraph("sg"));
    let leaf = arena.push_node(binding_node("a", "a"));
    arena.append_child(Container::Subgraph(sg), ElementHandle::Node(leaf));
    arena.append_child(Container::Root, ElementHandle::Subgraph(sg));
    let rendered = arena.finalize_root();
    let VisualElement::Subgraph(sg) = &rendered[0] else {
        panic!("root child should be a subgraph");
    };
    assert_eq!(sg.elements().len(), 1);
    assert_eq!(sg.elements()[0].id(), "a");
}

#[test]
fn prepend_child_places_handle_at_index_zero() {
    let mut arena = BuildArena::new();
    let sg = arena.push_subgraph(control_subgraph("sg"));
    let body = arena.push_node(binding_node("body", "body"));
    let anchor = arena.push_node(synthetic_node(
        "test-anchor",
        "if-test",
        SyntheticNodeKind::SyntheticIfStatementTest,
    ));
    arena.append_child(Container::Subgraph(sg), ElementHandle::Node(body));
    arena.prepend_child(Container::Subgraph(sg), ElementHandle::Node(anchor));
    arena.append_child(Container::Root, ElementHandle::Subgraph(sg));
    let rendered = arena.finalize_root();
    let VisualElement::Subgraph(sg) = &rendered[0] else {
        panic!("root child should be a subgraph");
    };
    let ids: Vec<&str> = sg.elements().iter().map(VisualElement::id).collect();
    assert_eq!(ids, vec!["test-anchor", "body"]);
}

#[test]
fn nested_subgraphs_preserve_handle_tree() {
    let mut arena = BuildArena::new();
    let outer = arena.push_subgraph(owned_subgraph("outer"));
    let inner = arena.push_subgraph(control_subgraph("inner"));
    let leaf = arena.push_node(binding_node("leaf", "leaf"));
    arena.append_child(Container::Subgraph(inner), ElementHandle::Node(leaf));
    arena.append_child(Container::Subgraph(outer), ElementHandle::Subgraph(inner));
    arena.append_child(Container::Root, ElementHandle::Subgraph(outer));
    let rendered = arena.finalize_root();
    let VisualElement::Subgraph(outer) = &rendered[0] else {
        panic!("expected outer subgraph at root");
    };
    let VisualElement::Subgraph(inner) = &outer.elements()[0] else {
        panic!("expected inner subgraph inside outer");
    };
    assert_eq!(inner.elements()[0].id(), "leaf");
}

#[test]
fn node_mut_writes_through_to_finalized_output() {
    let mut arena = BuildArena::new();
    let n = arena.push_node(binding_node("x", "x"));
    arena.append_child(Container::Root, ElementHandle::Node(n));
    arena.node_mut(n).set_unused(true);
    let rendered = arena.finalize_root();
    let VisualElement::Node(node) = &rendered[0] else {
        panic!("root child should be a node");
    };
    match node {
        VisualNode::Binding(b) => assert!(b.unused),
        VisualNode::Synthetic(_) => panic!("expected binding node"),
    }
}
