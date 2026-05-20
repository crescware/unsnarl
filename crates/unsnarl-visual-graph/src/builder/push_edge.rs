//! Mirrors `ts/src/visual-graph/builder/push-edge.ts`.
//!
//! Deduplicates emitted edges by a `from -->|label| to` key so the
//! TS one-edge-per-from/to/label invariant is preserved across
//! every site that pushes into `state.edges`.

use std::collections::HashSet;

use crate::visual_edge::VisualEdge;

pub fn push_edge(
    emitted_edges: &mut HashSet<String>,
    edges: &mut Vec<VisualEdge>,
    from: &str,
    label: &str,
    to: &str,
) {
    let key = format!("{from} -->|{label}| {to}");
    if !emitted_edges.insert(key) {
        return;
    }
    edges.push(VisualEdge {
        from: from.to_string(),
        to: to.to_string(),
        label: label.to_string(),
    });
}
