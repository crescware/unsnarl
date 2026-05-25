//! Deduplicates emitted edges by a `from -->|label| to` key so the
//! one-edge-per-(from, to, label) invariant is preserved across
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
    edges.push(VisualEdge::new(from, to, label));
}

#[cfg(test)]
#[path = "push_edge_test.rs"]
mod push_edge_test;
