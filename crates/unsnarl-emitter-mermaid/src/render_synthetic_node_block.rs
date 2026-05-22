//! Emits the trailing "synthetic node" block (module sinks /
//! sources / intermediates / expression statements).

use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_graph::VisualGraph;

use crate::emit_node::emit_node;
use crate::render_state::RenderState;
use crate::renders_in_synthetic_block::renders_in_synthetic_block;

pub fn render_synthetic_node_block(state: &mut RenderState<'_>, graph: &VisualGraph) {
    for e in &graph.elements {
        if let VisualElement::Node(n) = e {
            if renders_in_synthetic_block(n) {
                emit_node(state, n, "  ");
            }
        }
    }
}

#[cfg(test)]
#[path = "render_synthetic_node_block_test.rs"]
mod render_synthetic_node_block_test;
