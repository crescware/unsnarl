//! `VisualEdge`: a `from -label-> to` edge in the visual graph.

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct VisualEdge {
    pub from: String,
    pub to: String,
    pub label: String,
}

impl VisualEdge {
    pub fn new(from: impl Into<String>, to: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            label: label.into(),
        }
    }
}
