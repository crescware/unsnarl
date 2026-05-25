//! `VisualGraph`: top-level shape produced by the builder.
//!
//! The `SerializedIrVersion` carried in `version` is re-used from
//! `unsnarl-ir` to keep the on-disk constant single-sourced.

use serde::Serialize;
use unsnarl_ir::language::Language;
use unsnarl_ir::serialized::serialized_ir::{SerializedIrVersion, SERIALIZED_IR_VERSION};

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

impl VisualGraph {
    pub fn new(
        path: impl Into<String>,
        language: Language,
        direction: Direction,
        elements: Vec<VisualElement>,
        edges: Vec<VisualEdge>,
        boundary_edges: Vec<VisualBoundaryEdge>,
    ) -> Self {
        Self {
            version: SERIALIZED_IR_VERSION,
            source: VisualGraphSource {
                path: path.into(),
                language,
            },
            direction,
            elements,
            edges,
            boundary_edges,
            pruning: None,
        }
    }
}
