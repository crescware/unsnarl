//! Pipeline-level highlight orchestration.
//!
//! Mirrors `ts/src/pipeline/highlight/`. The TS file defines
//! `HighlightRunOptions` as the input the pipeline runner accepts
//! from `runDetailed({ highlight })` and then hands to the highlight
//! id collector. In the Rust workspace the type is hosted in
//! `unsnarl-visual-graph::highlight` so the emitter crates can reach
//! it through `EmitOptions`; this module re-exports it at the
//! pipeline path so callers can spell the import the same way they
//! do in TS.

pub mod highlight_run_options;

pub use highlight_run_options::HighlightRunOptions;
