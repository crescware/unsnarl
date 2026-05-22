//! `VisualBoundaryEdge`: a marker emitted by pruning to indicate
//! "more context exists in this direction" without dragging the next
//! generation of nodes back into the graph.
//!
//! Modeled as a discriminated union on `direction`. The enum uses
//! `#[serde(untagged)]` so each variant serializes its own field
//! order:
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
