//! Re-export of [`HighlightRunOptions`].
//!
//! The canonical type lives in `unsnarl-visual-graph::highlight`
//! (see the parent module's docstring for the rationale); this file
//! exists so the pipeline-side import path is available alongside
//! other `pipeline::*` types.

pub use unsnarl_visual_graph::highlight::HighlightRunOptions;
