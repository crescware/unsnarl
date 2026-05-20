//! `VisualGraph`: top-level shape produced by the builder.
//!
//! Mirrors `ts/src/visual-graph/visual-graph.ts`. The
//! `SerializedIrVersion` carried in `version` is re-used from
//! `unsnarl-ir` to keep the on-disk constant single-sourced.

use serde::Serialize;
use unsnarl_ir::language::Language;
use unsnarl_ir::serialized::serialized_ir::SerializedIrVersion;

use crate::direction::Direction;
use crate::visual_boundary_edge::VisualBoundaryEdge;
use crate::visual_edge::VisualEdge;
use crate::visual_element::VisualElement;
use crate::visual_graph_pruning::VisualGraphPruning;

#[derive(Clone, Serialize)]
pub struct VisualGraphSource {
    pub path: String,
    pub language: Language,
}

#[derive(Clone, Serialize)]
pub struct VisualGraph {
    pub version: SerializedIrVersion,
    pub source: VisualGraphSource,
    pub direction: Direction,
    pub elements: Vec<VisualElement>,
    pub edges: Vec<VisualEdge>,
    #[serde(rename = "boundaryEdges")]
    pub boundary_edges: Vec<VisualBoundaryEdge>,
    pub pruning: Option<VisualGraphPruning>,
}
