use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_node::{
    BindingVisualNode, SyntheticNodeKind, SyntheticVisualNode, VisualNode,
};

use super::render_synthetic_node_block;
use crate::mermaid_fixtures::{
    base_const_binding, base_graph, base_render_state, base_simple_synthetic,
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

/// Splits a rendered node line on `[` or `(` and returns the head id.
fn head_of(line: &str) -> String {
    let trimmed = line.trim_start();
    let end = trimmed.find(['[', '(']).unwrap_or(trimmed.len());
    trimmed[..end].to_string()
}

#[test]
fn emits_only_synthetic_top_level_nodes() {
    let mut state = base_render_state();
    let mut g = base_graph();
    g.elements = vec![
        synthetic("mod_a", SyntheticNodeKind::SyntheticModuleSource),
        binding("n_a"),
        synthetic("import_b", SyntheticNodeKind::SyntheticImportIntermediate),
        synthetic(MODULE_ROOT_ID, SyntheticNodeKind::SyntheticModuleSink),
    ];
    render_synthetic_node_block(&mut state, &g);
    let mut got: Vec<String> = state.lines.iter().map(|v| head_of(v)).collect();
    got.sort();
    assert_eq!(
        got,
        vec![
            "import_b".to_string(),
            "mod_a".to_string(),
            MODULE_ROOT_ID.to_string(),
        ]
    );
}

#[test]
fn skips_non_synthetic_nodes_entirely() {
    let mut state = base_render_state();
    let mut g = base_graph();
    g.elements = vec![binding("n_x")];
    render_synthetic_node_block(&mut state, &g);
    assert!(state.lines.is_empty());
}

#[test]
fn preserves_graph_element_order() {
    let mut state = base_render_state();
    let mut g = base_graph();
    g.elements = vec![
        synthetic("mod_first", SyntheticNodeKind::SyntheticModuleSource),
        synthetic(
            "import_second",
            SyntheticNodeKind::SyntheticImportIntermediate,
        ),
    ];
    render_synthetic_node_block(&mut state, &g);
    let got: Vec<String> = state.lines.iter().map(|v| head_of(v)).collect();
    assert_eq!(
        got,
        vec!["mod_first".to_string(), "import_second".to_string()]
    );
}
