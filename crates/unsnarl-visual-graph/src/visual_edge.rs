//! `VisualEdge`: a `from -label-> to` edge in the visual graph.
//!
//! Mirrors `ts/src/visual-graph/visual-edge.ts`.

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct VisualEdge {
    pub from: String,
    pub to: String,
    pub label: String,
}
