//! Emits top-level "tree" nodes (anything that isn't a synthetic
//! top-level import/module/sink and isn't a wrapped function-owner
//! name).
//!
//! Mirrors `ts/src/emitter/mermaid/render-top-level-nodes.ts`.

use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_graph::VisualGraph;

use crate::emit_node::emit_node;
use crate::render_state::RenderState;
use crate::renders_in_synthetic_block::renders_in_synthetic_block;

pub fn render_top_level_nodes(state: &mut RenderState<'_>, graph: &VisualGraph) {
    for e in &graph.elements {
        if let VisualElement::Node(n) = e {
            if !renders_in_synthetic_block(n) && !state.wrapped_owner_ids.contains(n.id()) {
                emit_node(state, n, "  ");
            }
        }
    }
}
