//! Returns the subset of `subgraph_ids` that appear as the `to`
//! endpoint of at least one ordinary `VisualEdge`.
//!
//! Used to apply the `edgeTargetSubgraph` class so a subgraph
//! whose border genuinely receives an edge gets a visible stroke,
//! while subgraphs whose border is never an edge terminus keep
//! their per-depth `nestL<N>` transparent stroke.
//!
//! `VisualBoundaryEdge`s are intentionally excluded: they represent
//! pruning markers ("more context exists this way") rather than
//! real reads/writes / calls into the subgraph, so a border on the
//! subgraph would misrepresent them. Highlight edges are also
//! out-of-scope; the highlight pass paints its own
//! `linkStyle` overrides separately.

use std::collections::HashSet;

use unsnarl_visual_graph::visual_edge::VisualEdge;

pub fn collect_edge_target_subgraph_ids(
    edges: &[VisualEdge],
    subgraph_ids: &HashSet<String>,
) -> HashSet<String> {
    let mut out = HashSet::new();
    for e in edges {
        if subgraph_ids.contains(&e.to) {
            out.insert(e.to.clone());
        }
    }
    out
}

#[cfg(test)]
#[path = "collect_edge_target_subgraph_ids_test.rs"]
mod collect_edge_target_subgraph_ids_test;
