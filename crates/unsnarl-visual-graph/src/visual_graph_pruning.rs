//! `VisualGraphPruning`: summary block that pruning attaches to
//! `VisualGraph.pruning`.
//!
//! Mirrors `ts/src/visual-graph/visual-graph-pruning.ts`. Step 13
//! never populates this — `pruning` is always `null` in the base
//! graph. Pruning fills it in Step 17.

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
