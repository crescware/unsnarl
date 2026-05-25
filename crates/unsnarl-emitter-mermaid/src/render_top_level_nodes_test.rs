use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_node::{
    BindingVisualNode, SyntheticNodeKind, SyntheticVisualNode, VisualNode,
};
use unsnarl_visual_graph::visual_subgraph::{OwnedVisualSubgraph, VisualSubgraph};

use super::render_top_level_nodes;
use crate::testing::{
    base_const_binding, base_function_subgraph, base_graph, base_render_state,
    base_simple_synthetic,
};

const MODULE_ROOT_ID: &str = "module_root";

fn synthetic(id: &str, kind: SyntheticNodeKind) -> VisualElement {
    VisualNode::from(SyntheticVisualNode {
        id: id.to_string(),
        ..base_simple_synthetic(kind)
    })
    .into()
}

fn binding(id: &str) -> VisualElement {
    VisualNode::from(BindingVisualNode {
        id: id.to_string(),
        ..base_const_binding()
    })
    .into()
}

#[test]
fn emits_non_synthetic_non_wrapped_top_level_nodes() {
    let mut state = base_render_state();
    let mut g = base_graph();
    g.elements = vec![binding("n_a"), binding("n_b")];
    render_top_level_nodes(&mut state, &g);
    let got: Vec<String> = state
        .lines
        .iter()
        .map(|v| {
            let trimmed = v.trim_start();
            let end = trimmed.find(['[', '(']).unwrap_or(trimmed.len());
            trimmed[..end].to_string()
        })
        .collect();
    assert_eq!(got, vec!["n_a".to_string(), "n_b".to_string()]);
}

#[test]
fn skips_synthetic_node_kinds() {
    let mut state = base_render_state();
    let mut g = base_graph();
    g.elements = vec![
        synthetic("mod_a", SyntheticNodeKind::SyntheticModuleSource),
        binding("n_b"),
        synthetic(MODULE_ROOT_ID, SyntheticNodeKind::SyntheticModuleSink),
    ];
    render_top_level_nodes(&mut state, &g);
    assert_eq!(state.lines.len(), 1);
    assert!(state.lines[0].trim_start().starts_with("n_b"));
}

#[test]
fn skips_nodes_whose_id_is_in_wrapped_owner_ids() {
    let mut state = base_render_state();
    state.wrapped_owner_ids.insert("n_owner".to_string());
    let mut g = base_graph();
    g.elements = vec![binding("n_owner"), binding("n_keep")];
    render_top_level_nodes(&mut state, &g);
    assert_eq!(state.lines.len(), 1);
    assert!(state.lines[0].trim_start().starts_with("n_keep"));
}

#[test]
fn ignores_top_level_subgraph_elements() {
    let mut state = base_render_state();
    let mut g = base_graph();
    let sg = base_function_subgraph();
    g.elements = vec![VisualSubgraph::from(OwnedVisualSubgraph {
        id: "sg".to_string(),
        ..sg
    })
    .into()];
    render_top_level_nodes(&mut state, &g);
    assert!(state.lines.is_empty());
}

#[test]
fn uses_two_space_indent_at_the_top_level() {
    let mut state = base_render_state();
    let mut g = base_graph();
    g.elements = vec![binding("n_a")];
    render_top_level_nodes(&mut state, &g);
    assert!(state.lines[0].starts_with("  n_a"));
}
