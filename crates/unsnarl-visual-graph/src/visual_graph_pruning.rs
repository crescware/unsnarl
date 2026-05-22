//! `VisualGraphPruning`: summary block that pruning attaches to
//! `VisualGraph.pruning`.
//!
//! The base visual-graph build leaves `pruning` at `None`; only the
//! pruning pass populates it.

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct PruningRoot {
    pub query: String,
    pub matched: u32,
}

#[derive(Clone, Serialize)]
pub struct VisualGraphPruning {
    pub roots: Vec<PruningRoot>,
    pub descendants: u32,
    pub ancestors: u32,
}
