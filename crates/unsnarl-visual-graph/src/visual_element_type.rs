//! Constant `type` tags carried by [`VisualNode`](crate::visual_node::VisualNode)
//! and [`VisualSubgraph`](crate::visual_subgraph::VisualSubgraph).
//!
//! Mirrors `ts/src/visual-graph/visual-element-type.ts`. The TS
//! constants double as the on-disk JSON discriminator strings
//! (`"node"` / `"subgraph"`); each side is its own unit enum here
//! so it sits in a struct's `type` field as a const string.

use serde::Serialize;

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum NodeTypeTag {
    #[serde(rename = "node")]
    Node,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SubgraphTypeTag {
    #[serde(rename = "subgraph")]
    Subgraph,
}
