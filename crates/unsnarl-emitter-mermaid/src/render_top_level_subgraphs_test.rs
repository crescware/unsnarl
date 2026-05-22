use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_node::{BindingVisualNode, VisualNode};
use unsnarl_visual_graph::visual_subgraph::{
    ControlSubgraphKind, ControlVisualSubgraph, VisualSubgraph,
};

use super::render_top_level_subgraphs;
use crate::testing::{base_const_binding, base_graph, base_plain_subgraph, base_render_state};

fn subgraph(id: &str, kind: ControlSubgraphKind) -> VisualElement {
    VisualElement::Subgraph(VisualSubgraph::Control(ControlVisualSubgraph {
        id: id.to_string(),
        ..base_plain_subgraph(kind)
    }))
}

#[test]
fn delegates_to_emit_subgraph_for_each_top_level_subgraph() {
    let mut state = base_render_state();
    let mut g = base_graph();
    g.elements = vec![
        subgraph("s1", ControlSubgraphKind::If),
        subgraph("s2", ControlSubgraphKind::Else),
    ];
    render_top_level_subgraphs(&mut state, &g);
    let opens: Vec<&String> = state
        .lines
        .iter()
        .filter(|v| v.starts_with("  subgraph"))
        .collect();
    assert_eq!(opens.len(), 2);
}

#[test]
fn ignores_top_level_node_elements() {
    let mut state = base_render_state();
    let mut g = base_graph();
    g.elements = vec![VisualElement::Node(VisualNode::Binding(
        BindingVisualNode {
            id: "n_a".to_string(),
            ..base_const_binding()
        },
    ))];
    render_top_level_subgraphs(&mut state, &g);
    assert!(state.lines.is_empty());
}

#[test]
fn indents_at_two_spaces() {
    let mut state = base_render_state();
    let mut g = base_graph();
    g.elements = vec![subgraph("s1", ControlSubgraphKind::If)];
    render_top_level_subgraphs(&mut state, &g);
    assert!(state.lines[0].starts_with("  subgraph s1"));
}
