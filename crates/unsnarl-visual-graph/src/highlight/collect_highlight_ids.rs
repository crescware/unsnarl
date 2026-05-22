//! Walk a [`VisualGraph`] and return the ids of every visible node
//! that satisfies at least one of the supplied queries.
//!
//! The query grammar matches `-r/--roots`, but the matching predicate
//! is the highlight-specific [`node_matches_highlight_query`], which
//! deliberately does not apply the `NAME_QUERY_EXCLUDED` filter so
//! `-H counter` paints every place `counter` appears, use-sites
//! included. The caller is responsible for already having resolved
//! any `LineOrName` ambiguity (commonly via `resolve_ambiguous_queries`).

use std::collections::HashSet;

use unsnarl_root_query::ParsedRootQuery;

use crate::highlight::node_matches_highlight_query::node_matches_highlight_query;
use crate::prune::iterate_visual_nodes::iterate_visual_nodes;
use crate::visual_graph::VisualGraph;

/// The list is returned in `iterate_visual_nodes` walk order with
/// duplicates removed, mirroring the iteration order of the
/// `ReadonlySet<string>` returned by the TS port (TS `Set` preserves
/// insertion order). Downstream consumers — the mermaid emitter's
/// inline `style` block, in particular — rely on this order to
/// reproduce the TS baselines byte-for-byte.
pub fn collect_highlight_ids(graph: &VisualGraph, queries: &[ParsedRootQuery]) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    if queries.is_empty() {
        return ids;
    }
    let mut seen: HashSet<String> = HashSet::new();
    iterate_visual_nodes(&graph.elements, &mut |node| {
        for q in queries {
            if node_matches_highlight_query(node, q) {
                if seen.insert(node.id().to_string()) {
                    ids.push(node.id().to_string());
                }
                break;
            }
        }
    });
    ids
}

#[cfg(test)]
#[path = "collect_highlight_ids_test.rs"]
mod collect_highlight_ids_test;
