//! `MermaidStrategy` has two concrete variants (Dagre / Elk) and no
//! open extension point, so the placeholder-emission tests pin
//! against `Elk` (which always emits a placeholder) and `Dagre`
//! (which always returns `None`).

use unsnarl_visual_graph::direction::Direction;
use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_node::{BindingVisualNode, VisualNode};
use unsnarl_visual_graph::visual_subgraph::{
    ControlSubgraphKind, ControlVisualSubgraph, VisualSubgraph,
};

use super::emit_plain_subgraph;
use crate::strategy::MermaidStrategy;
use crate::testing::{base_const_binding, base_plain_subgraph, base_render_state};

fn node(id: &str) -> VisualElement {
    VisualNode::from(BindingVisualNode {
        id: id.to_string(),
        ..base_const_binding()
    })
    .into()
}

fn if_subgraph(id: &str, elements: Vec<VisualElement>) -> VisualSubgraph {
    ControlVisualSubgraph {
        id: id.to_string(),
        elements,
        ..base_plain_subgraph(ControlSubgraphKind::If)
    }
    .into()
}

fn else_subgraph(id: &str) -> VisualSubgraph {
    ControlVisualSubgraph {
        id: id.to_string(),
        elements: Vec::new(),
        ..base_plain_subgraph(ControlSubgraphKind::Else)
    }
    .into()
}

#[test]
fn emits_subgraph_open_direction_line_child_nodes_and_close() {
    let mut state = base_render_state();
    let sg: VisualSubgraph = ControlVisualSubgraph {
        id: "s_x".to_string(),
        direction: Direction::TB,
        elements: vec![node("n_a"), node("n_b")],
        ..base_plain_subgraph(ControlSubgraphKind::If)
    }
    .into();
    emit_plain_subgraph(&mut state, &sg, "  ", 1);
    assert_eq!(state.lines[0], "  subgraph s_x[\"if L1\"]");
    assert_eq!(state.lines[1], "    direction TB");
    assert_eq!(state.lines.last().unwrap(), "  end");
    assert_eq!(state.lines.len(), 5);
}

#[test]
fn emits_child_nodes_before_nested_subgraphs() {
    let mut state = base_render_state();
    let sg = if_subgraph("outer", vec![else_subgraph("inner").into(), node("n_a")]);
    emit_plain_subgraph(&mut state, &sg, "  ", 1);
    let node_idx = state
        .lines
        .iter()
        .position(|v| v.contains("n_a"))
        .expect("n_a emitted");
    let inner_idx = state
        .lines
        .iter()
        .position(|v| v.contains("subgraph inner"))
        .expect("inner subgraph emitted");
    assert!(node_idx < inner_idx);
}

#[test]
fn skips_child_nodes_whose_id_is_in_wrapped_owner_ids() {
    let mut state = base_render_state();
    state.wrapped_owner_ids.insert("n_owner".to_string());
    let sg = if_subgraph("s_x", vec![node("n_owner"), node("n_keep")]);
    emit_plain_subgraph(&mut state, &sg, "  ", 1);
    assert!(!state.lines.iter().any(|v| v.contains("n_owner")));
    assert!(state.lines.iter().any(|v| v.contains("n_keep")));
}

#[test]
fn invokes_empty_subgraph_placeholder_when_there_are_no_emitted_children() {
    let mut state = base_render_state();
    // Elk's strategy always emits a placeholder for empty subgraphs.
    state.strategy = MermaidStrategy::Elk;
    let sg = if_subgraph("empty", Vec::new());
    emit_plain_subgraph(&mut state, &sg, "  ", 1);
    assert!(state
        .lines
        .iter()
        .any(|v| v.contains("elk_empty_empty[\"No nodes\"]")));
    assert_eq!(state.placeholder_ids, vec!["elk_empty_empty".to_string()]);
}

#[test]
fn does_not_invoke_the_placeholder_when_at_least_one_child_was_emitted() {
    let mut state = base_render_state();
    state.strategy = MermaidStrategy::Elk;
    let sg = if_subgraph("s_x", vec![node("n_a")]);
    emit_plain_subgraph(&mut state, &sg, "  ", 1);
    assert!(!state
        .lines
        .iter()
        .any(|v| v.starts_with("    elk_empty_") && v.contains("No nodes")));
    assert!(state.placeholder_ids.is_empty());
}

#[test]
fn placeholder_returning_none_inserts_no_line_and_registers_no_id() {
    let mut state = base_render_state();
    // Dagre's strategy returns None for empty subgraphs.
    let sg = if_subgraph("empty", Vec::new());
    let before = state.lines.len();
    emit_plain_subgraph(&mut state, &sg, "  ", 1);
    // open + direction + end = 3 lines, no placeholder
    assert_eq!(state.lines.len() - before, 3);
    assert!(state.placeholder_ids.is_empty());
}

#[test]
fn records_the_subgraph_id_under_its_one_based_depth_in_nest_class_map() {
    let mut state = base_render_state();
    let sg = if_subgraph("s_at_depth1", Vec::new());
    emit_plain_subgraph(&mut state, &sg, "  ", 1);
    assert_eq!(
        state.nest_class_map.get(&0),
        Some(&vec!["s_at_depth1".to_string()])
    );
}

#[test]
fn recurses_into_nested_subgraphs_with_depth_plus_one() {
    let mut state = base_render_state();
    let sg = if_subgraph("outer", vec![else_subgraph("inner").into()]);
    emit_plain_subgraph(&mut state, &sg, "  ", 1);
    assert_eq!(
        state.nest_class_map.get(&0),
        Some(&vec!["outer".to_string()])
    );
    assert_eq!(
        state.nest_class_map.get(&1),
        Some(&vec!["inner".to_string()])
    );
}

#[test]
fn wraps_to_slot_zero_when_depth_exceeds_the_palette_length() {
    let mut state = base_render_state();
    let palette_length = state.theme.nest_palette.len();
    let overflow_depth = (palette_length + 1) as u32;
    let sg = if_subgraph("s_overflow", Vec::new());
    emit_plain_subgraph(&mut state, &sg, "  ", overflow_depth);
    assert_eq!(
        state.nest_class_map.get(&0),
        Some(&vec!["s_overflow".to_string()])
    );
}
