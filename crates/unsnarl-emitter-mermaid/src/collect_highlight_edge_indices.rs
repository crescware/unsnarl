//! Walks body / import / boundary edges in the order the renderer
//! emits them and returns the indices of edges that touch a
//! highlighted node id.
//!
//! Mirrors `ts/src/emitter/mermaid/collect-highlight-edge-indices.ts`.

use std::collections::HashSet;

use unsnarl_visual_graph::visual_boundary_edge::VisualBoundaryEdge;
use unsnarl_visual_graph::visual_edge::VisualEdge;

pub fn collect_highlight_edge_indices(
    body_edges: &[&VisualEdge],
    import_edges: &[&VisualEdge],
    boundary_edges: &[VisualBoundaryEdge],
    highlight_ids: &HashSet<String>,
) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    if highlight_ids.is_empty() {
        return out;
    }
    let mut i = 0usize;
    for e in body_edges {
        if highlight_ids.contains(&e.from) || highlight_ids.contains(&e.to) {
            out.push(i);
        }
        i += 1;
    }
    for e in import_edges {
        if highlight_ids.contains(&e.from) || highlight_ids.contains(&e.to) {
            out.push(i);
        }
        i += 1;
    }
    for be in boundary_edges {
        let inside = match be {
            VisualBoundaryEdge::Out { inside, .. } | VisualBoundaryEdge::In { inside, .. } => {
                inside
            }
        };
        if highlight_ids.contains(inside) {
            out.push(i);
        }
        i += 1;
    }
    out
}
