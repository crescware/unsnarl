//! Mirrors `ts/src/emitter/mermaid/emit-subgraph.test.ts`.

use unsnarl_visual_graph::visual_node::{BindingNodeKind, BindingVisualNode, VisualNode};
use unsnarl_visual_graph::visual_subgraph::{
    ControlSubgraphKind, OwnedExtras, OwnedVisualSubgraph, VisualSubgraph,
};

use super::emit_subgraph;
use crate::testing::{
    base_function_subgraph, base_plain_subgraph, base_render_state, base_simple_binding,
};

fn function_owner() -> VisualNode {
    VisualNode::Binding(BindingVisualNode {
        id: "n_owner".to_string(),
        name: "f".to_string(),
        ..base_simple_binding(BindingNodeKind::FunctionDeclaration)
    })
}

fn function_subgraph(id: &str, owner_node_id: Option<&str>) -> VisualSubgraph {
    let sg = OwnedVisualSubgraph {
        id: id.to_string(),
        extras: OwnedExtras::Function {
            owner_node_id: owner_node_id.map(str::to_string),
            owner_name: "f".to_string(),
        },
        ..base_function_subgraph()
    };
    VisualSubgraph::Owned(sg)
}

#[test]
fn function_with_known_owner_node_id_is_wrapped_in_wrap_id_subgraph() {
    let owner = function_owner();
    let mut state = base_render_state();
    state.node_map.insert(owner.id().to_string(), &owner);
    let sg = function_subgraph("s_fn", Some("n_owner"));
    emit_subgraph(&mut state, &sg, "  ", 1);
    assert_eq!(state.lines[0], "  subgraph wrap_s_fn[\" \"]");
    assert_eq!(state.lines[1], "    direction TB");
    assert_eq!(state.lines.last().unwrap(), "  end");
}

#[test]
fn function_without_an_owner_node_in_the_map_falls_back_to_plain_emission() {
    let mut state = base_render_state();
    let sg = function_subgraph("s_fn", Some("n_missing"));
    emit_subgraph(&mut state, &sg, "  ", 1);
    assert!(state
        .lines
        .iter()
        .any(|v| v.starts_with("  subgraph s_fn[\"")));
}

#[test]
fn non_function_subgraphs_are_emitted_plainly_without_a_wrapper() {
    let mut state = base_render_state();
    let sg = VisualSubgraph::Control(
        unsnarl_visual_graph::visual_subgraph::ControlVisualSubgraph {
            id: "s_if".to_string(),
            ..base_plain_subgraph(ControlSubgraphKind::If)
        },
    );
    emit_subgraph(&mut state, &sg, "  ", 1);
    assert!(state
        .lines
        .iter()
        .any(|v| v.starts_with("  subgraph s_if[\"")));
}

#[test]
fn function_wrapper_sits_one_palette_slot_above_its_body_subgraph() {
    let owner = function_owner();
    let mut state = base_render_state();
    state.node_map.insert(owner.id().to_string(), &owner);
    let sg = function_subgraph("s_fn", Some("n_owner"));
    emit_subgraph(&mut state, &sg, "  ", 2);
    // Slot 1 corresponds to depth 2 (wrap), slot 2 to depth 3 (body).
    assert_eq!(
        state.nest_class_map.get(&1),
        Some(&vec!["wrap_s_fn".to_string()])
    );
    assert_eq!(
        state.nest_class_map.get(&2),
        Some(&vec!["s_fn".to_string()])
    );
}

#[test]
fn the_owner_node_line_appears_inside_the_wrapper_before_the_function_body_subgraph() {
    let owner = function_owner();
    let mut state = base_render_state();
    state.node_map.insert(owner.id().to_string(), &owner);
    let sg = function_subgraph("s_fn", Some("n_owner"));
    emit_subgraph(&mut state, &sg, "  ", 1);
    let owner_idx = state
        .lines
        .iter()
        .position(|v| v.contains("n_owner"))
        .expect("owner line present");
    let inner_idx = state
        .lines
        .iter()
        .position(|v| v.contains("subgraph s_fn"))
        .expect("inner subgraph present");
    assert!(owner_idx > 0);
    assert!(owner_idx < inner_idx);
}

#[test]
fn non_function_subgraphs_occupy_a_palette_slot_at_the_supplied_depth() {
    let mut state = base_render_state();
    let sg = VisualSubgraph::Control(
        unsnarl_visual_graph::visual_subgraph::ControlVisualSubgraph {
            id: "s_if".to_string(),
            ..base_plain_subgraph(ControlSubgraphKind::If)
        },
    );
    emit_subgraph(&mut state, &sg, "  ", 3);
    assert_eq!(
        state.nest_class_map.get(&2),
        Some(&vec!["s_if".to_string()])
    );
}
