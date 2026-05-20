//! Boundary-edge direction tags.
//!
//! Mirrors `ts/src/visual-graph/prune/boundary-edge-direction.ts`.
//! In the TS port the constants live under `prune/`, but pruning
//! lands later (Step 17). Hosting them in `unsnarl-visual-graph`
//! here lets [`VisualBoundaryEdge`](crate::visual_boundary_edge)
//! reference them without a forward dependency on a prune module
//! that does not yet exist.
//!
//! The two tags double as on-disk JSON values (`"out"` / `"in"`).
//! Each is its own unit enum so it can sit in a struct field whose
//! serialization is a const string, mirroring the TS `as const`
//! pattern.

use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BoundaryEdgeDirectionOut {
    #[serde(rename = "out")]
    Out,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BoundaryEdgeDirectionIn {
    #[serde(rename = "in")]
    In,
}
