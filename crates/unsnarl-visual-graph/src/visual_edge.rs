//! `VisualEdge`: a `from -label-> to` edge in the visual graph.

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct VisualEdge {
    pub from: String,
    pub to: String,
    pub label: String,
}
