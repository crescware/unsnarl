//! Pipeline-level highlight orchestration.
//!
//! `HighlightRunOptions` is the input the pipeline runner accepts
//! and hands to the highlight id collector. The type is hosted in
//! `unsnarl-visual-graph::highlight` so the emitter crates can
//! reach it through `EmitOptions`; this module re-exports it at the
//! pipeline path so call sites can spell the import alongside the
//! other `pipeline::*` types.

pub mod highlight_run_options;

pub use highlight_run_options::HighlightRunOptions;
