//! Tests for [`super::arena`].
//!
//! Cover the round-trip from arena handle tree to `Vec<VisualElement>`
//! since the rest of the builder relies on that invariant. Field
//! order of the rendered subgraphs is checked separately by the
//! sibling tests at the crate root (`visual_subgraph_test.rs`).

use std::collections::HashSet;

use super::{BuildArena, Container, ElementHandle};
use crate::direction::Direction;
use crate::visual_element::VisualElement;
use crate::visual_node::{BindingVisualNode, SyntheticVisualNode, VisualNode};
use crate::visual_subgraph::{ControlVisualSubgraph, OwnedVisualSubgraph, VisualSubgraph};

fn binding_node(id: &str, name: &str) -> VisualNode {
    BindingVisualNode::const_binding(id, name, 1).into()
}

fn if_test_node(id: &str) -> VisualNode {
    SyntheticVisualNode::if_statement_test(id, 1).into()
}

fn owned_subgraph(id: &str) -> VisualSubgraph {
    let sg = OwnedVisualSubgraph {
        end_line: Some(3),
        ..OwnedVisualSubgraph::function(id, 1, None, "fn", Vec::new(), Direction::RL)
    };
    sg.into()
}

fn control_subgraph(id: &str) -> VisualSubgraph {
    let sg = ControlVisualSubgraph {
        end_line: Some(2),
        ..ControlVisualSubgraph::block(id, 1, Vec::new(), Direction::RL)
    };
    sg.into()
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
    let anchor = arena.push_node(if_test_node("test-anchor"));
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
fn detach_root_nodes_reparents_a_binding_into_a_subgraph_without_double_finalize() {
    let mut arena = BuildArena::new();
    // Two bindings emitted at the root, as `build_scope` would.
    let keep = arena.push_node(binding_node("keep", "keep"));
    let moved = arena.push_node(binding_node("moved", "moved"));
    arena.append_child(Container::Root, ElementHandle::Node(keep));
    arena.append_child(Container::Root, ElementHandle::Node(moved));
    // Re-parent `moved` under a freshly-created module subgraph.
    let mut detach: HashSet<_> = HashSet::new();
    detach.insert(moved);
    arena.detach_root_nodes(&detach);
    let sg = arena.push_subgraph(owned_subgraph("sg"));
    arena.append_child(Container::Subgraph(sg), ElementHandle::Node(moved));
    arena.append_child(Container::Root, ElementHandle::Subgraph(sg));
    // `moved` must be finalized exactly once (inside the subgraph),
    // never twice -- a double visit would panic in `finalize_handle`.
    let rendered = arena.finalize_root();
    assert_eq!(rendered.len(), 2);
    assert_eq!(rendered[0].id(), "keep");
    let VisualElement::Subgraph(sg) = &rendered[1] else {
        panic!("second root child should be the subgraph");
    };
    assert_eq!(sg.elements().len(), 1);
    assert_eq!(sg.elements()[0].id(), "moved");
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
