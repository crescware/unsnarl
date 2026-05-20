//! Visual graph model + builder.
//!
//! The data types are the on-disk shape of `-f json`: a `version` /
//! `source` / `direction` header, an `elements` tree of nodes and
//! subgraphs, an `edges` list, and pruning side-bands. The
//! [`build_visual_graph`] entry point (when populated) walks an
//! analyzed `SerializedIR` and produces a `VisualGraph` ready to
//! serialize via `serde_json`.
//!
//! Step status (per the #108 plan):
//!
//! - Step 13: builder + base graph (this crate's primary focus).
//! - Step 17: prune. The `boundary_edge_direction` module is
//!   already hosted here so [`VisualBoundaryEdge`] does not have a
//!   forward dependency on a prune module that does not yet exist.
//! - Step 18: depth control inside the builder.
//! - Step 19: highlight.

pub mod boundary_edge_direction;
pub mod builder;
pub mod direction;
pub mod node_kind;
pub mod subgraph_kind;
pub mod visual_boundary_edge;
pub mod visual_edge;
pub mod visual_element;
pub mod visual_element_type;
pub mod visual_graph;
pub mod visual_graph_pruning;
pub mod visual_node;
pub mod visual_subgraph;

pub use boundary_edge_direction::{BoundaryEdgeDirectionIn, BoundaryEdgeDirectionOut};
pub use direction::Direction;
pub use node_kind::NodeKind;
pub use subgraph_kind::SubgraphKind;
pub use visual_boundary_edge::VisualBoundaryEdge;
pub use visual_edge::VisualEdge;
pub use visual_element::VisualElement;
pub use visual_element_type::{NodeTypeTag, SubgraphTypeTag};
pub use visual_graph::{VisualGraph, VisualGraphSource};
pub use visual_graph_pruning::{PruningRoot, VisualGraphPruning};
pub use visual_node::{
    BindingExtras, BindingNodeKind, BindingVisualNode, SyntheticExtras, SyntheticNodeKind,
    SyntheticVisualNode, VisualNode,
};
pub use visual_subgraph::{
    ControlExtras, ControlSubgraphKind, ControlVisualSubgraph, OwnedExtras, OwnedSubgraphKind,
    OwnedVisualSubgraph, VisualSubgraph,
};
