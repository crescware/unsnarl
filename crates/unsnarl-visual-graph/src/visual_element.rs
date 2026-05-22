//! `VisualElement`: union of [`VisualNode`] and [`VisualSubgraph`].
//!
//! The wrapping enum is untagged because the variants already
//! serialize their own `type` discriminator.

use serde::Serialize;

use crate::visual_node::VisualNode;
use crate::visual_subgraph::VisualSubgraph;

#[derive(Clone, Serialize)]
#[serde(untagged)]
pub enum VisualElement {
    Node(VisualNode),
    Subgraph(VisualSubgraph),
}

impl VisualElement {
    pub fn id(&self) -> &str {
        match self {
            Self::Node(n) => n.id(),
            Self::Subgraph(s) => s.id(),
        }
    }
}
