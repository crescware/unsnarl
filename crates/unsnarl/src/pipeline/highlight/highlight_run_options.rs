//! Re-export of [`HighlightRunOptions`].
//!
//! Mirrors `ts/src/pipeline/highlight/highlight-run-options.ts`. The
//! canonical type lives in `unsnarl-visual-graph::highlight` (see the
//! parent module's docstring for the rationale); this file exists so
//! the pipeline-side import path mirrors TS.

pub use unsnarl_visual_graph::highlight::HighlightRunOptions;
