//! `VisualBoundaryEdge`: a marker emitted by pruning to indicate
//! "more context exists in this direction" without dragging the next
//! generation of nodes back into the graph.
//!
//! Mirrors `ts/src/visual-graph/visual-boundary-edge.ts`. The TS
//! shape is a discriminated union on `direction`. The Rust enum
//! uses `#[serde(untagged)]` so each variant serializes its own
//! field order, matching the JS object-literal output:
//!
//! - `Out` direction: `{ inside, direction: "out" }`
//! - `In`  direction: `{ inside, direction: "in", label }`

use serde::Serialize;

use crate::boundary_edge_direction::{BoundaryEdgeDirectionIn, BoundaryEdgeDirectionOut};

#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum VisualBoundaryEdge {
    Out {
        inside: String,
        direction: BoundaryEdgeDirectionOut,
    },
    In {
        inside: String,
        direction: BoundaryEdgeDirectionIn,
        label: String,
    },
}
