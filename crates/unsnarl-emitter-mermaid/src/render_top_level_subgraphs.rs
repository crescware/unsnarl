//! Emits every top-level subgraph at depth 1.
//!
//! Mirrors `ts/src/emitter/mermaid/render-top-level-subgraphs.ts`.

use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_graph::VisualGraph;

use crate::emit_subgraph::emit_subgraph;
use crate::render_state::RenderState;

pub fn render_top_level_subgraphs(state: &mut RenderState<'_>, graph: &VisualGraph) {
    for e in &graph.elements {
        if let VisualElement::Subgraph(s) = e {
            // Top-level subgraphs sit at nesting depth 1 (palette
            // slot 0). The depth is 1-based throughout so it
            // matches the user-facing `nestL<n>` class names.
            emit_subgraph(state, s, "  ", 1);
        }
    }
}

#[cfg(test)]
#[path = "render_top_level_subgraphs_test.rs"]
mod render_top_level_subgraphs_test;
