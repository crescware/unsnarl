//! Inputs and outputs for [`prune_visual_graph`](super::prune_visual_graph).
//!
//! Mirrors `ts/src/visual-graph/prune/prune-options.ts`. `PruneOptions`
//! carries the resolved (post-`resolve_ambiguous_queries`) query list
//! plus the descendants/ancestors radii from the CLI's
//! `-A` / `-B` / `-C` flags.

use std::collections::HashSet;

use unsnarl_root_query::ParsedRootQuery;

use crate::visual_graph::VisualGraph;

pub struct PruneOptions {
    pub roots: Vec<ParsedRootQuery>,
    pub descendants: u32,
    pub ancestors: u32,
}

pub struct PerQueryMatch {
    pub query: ParsedRootQuery,
    pub matched: u32,
}

pub struct PruneResult {
    pub graph: VisualGraph,
    pub per_query: Vec<PerQueryMatch>,
    /// The exact id set the prune walk treated as "roots" — i.e. the
    /// nodes the queries matched directly (and any nodes swept in by
    /// a bare line query that lands on a subgraph's start line). The
    /// BFS descendants/ancestors are NOT included. Exposed so `-H`
    /// in roots mode can paint the same id set the user pinpointed
    /// via `-r`, inheriting the same use-site exclusions that
    /// pruning applies on a bare name query.
    pub root_ids: HashSet<String>,
}
