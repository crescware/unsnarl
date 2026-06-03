//! Returns the subset of `subgraph_ids` that appear as the `from`
//! endpoint of at least one ordinary `VisualEdge`.
//!
//! Mirror of [`crate::collect_edge_target_subgraph_ids`] for the
//! opposite endpoint: a subgraph whose border is the *origin* of an
//! edge gets the same visible stroke as one whose border is the
//! terminus, so an arrow leaving a subgraph reads as clearly as one
//! arriving. Subgraphs that are neither a source nor a target keep
//! their per-depth `nestL<N>` transparent stroke and stay
//! border-less.
//!
//! `VisualBoundaryEdge`s are intentionally excluded for the same
//! reason as in the target collector: they are pruning markers
//! ("more context exists this way") rather than real reads/writes /
//! calls out of the subgraph, so a border would misrepresent them.
//! Highlight edges are likewise out-of-scope; the highlight pass
//! paints its own `linkStyle` overrides separately.

use std::collections::HashSet;

use unsnarl_visual_graph::visual_edge::VisualEdge;

pub fn collect_edge_source_subgraph_ids(
    edges: &[VisualEdge],
    subgraph_ids: &HashSet<String>,
) -> HashSet<String> {
    let mut out = HashSet::new();
    for e in edges {
        if subgraph_ids.contains(&e.from) {
            out.insert(e.from.clone());
        }
    }
    out
}

#[cfg(test)]
#[path = "collect_edge_source_subgraph_ids_test.rs"]
mod collect_edge_source_subgraph_ids_test;
