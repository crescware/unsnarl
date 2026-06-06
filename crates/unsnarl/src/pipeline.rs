//! End-to-end pipeline helpers for the CLI and the parity harness.
//!
//! Parse the source with `OxcParser`, analyse it with
//! [`run_analysis`](unsnarl_analyzer::run_analysis), hand the analysed
//! IR + annotations to `FlatSerializer`, and render the result with
//! `IrEmitter` (or one of the other emitters, with optional
//! visual-graph pruning / highlight applied beforehand).
//!
//! The per-format entry points live in sibling modules
//! (`emit_ir`, `emit_json`, `emit_mermaid`, `emit_markdown`,
//! `emit_stats`) and are re-exported here; the shared stages
//! (`serialize_ir`, `prepare_emit`, `emit_pruning_aware_with`) back
//! them.

pub mod highlight;
pub mod plugin;
pub mod prune;

mod emit_ir;
mod emit_json;
mod emit_markdown;
mod emit_mermaid;
mod emit_pruning_aware_with;
mod emit_stats;
mod language_for_path;
mod prepare_emit;
mod serialize_ir;

use unsnarl_ir::diagnostic::Diagnostic;
use unsnarl_ir::nesting_kind::NestingDepths;
use unsnarl_plugin::UnsnarlPlugin;
use unsnarl_visual_graph::highlight::{HighlightRunOptions, HighlightWarning};
use unsnarl_visual_graph::prune::RootQueryResolution;

use crate::pipeline::prune::PruningRunOptions;

pub use emit_ir::{emit_ir_detailed, emit_ir_text};
pub use emit_json::{emit_json_detailed, emit_json_text};
pub use emit_markdown::{emit_markdown_detailed, emit_markdown_text};
pub use emit_mermaid::{emit_mermaid_detailed, emit_mermaid_text};
pub use emit_stats::{emit_stats_detailed, emit_stats_text};
pub use language_for_path::{language_for_path, source_type_from_path};

/// Per-emit run knobs that the emitter helpers consume directly.
/// Optional fields keep individual `emit_*_text` call sites stable
/// when new transforms enter the visual-graph build.
#[derive(Default, Clone, Copy)]
pub struct PipelineRunOptions<'a> {
    pub pruning: Option<&'a PruningRunOptions>,
    pub depths: Option<&'a NestingDepths>,
    pub highlight: Option<&'a HighlightRunOptions>,
    /// Activated plugins to fold over the serialized IR. The slice
    /// is `&[&dyn UnsnarlPlugin]` so callers can pass the activated
    /// set returned by [`plugin::activate`] without taking ownership
    /// of the trait objects.
    pub plugins: &'a [&'a dyn UnsnarlPlugin],
}

/// Detailed result of a pipeline run.
///
/// The IR path leaves `pruning` / `resolutions` at `None` (those
/// fields belong to the visual-graph-aware emit paths).
pub struct PipelineRunDetails {
    pub text: String,
    pub pruning: Option<Vec<PrunePerQueryDetail>>,
    pub resolutions: Option<Vec<RootQueryResolution>>,
    /// No-match / no-path warnings raised by a `-H <queries>` path /
    /// direction query, surfaced on stderr by the CLI layer. `None` on
    /// every path that does not run the highlight collector.
    pub highlight_warnings: Option<Vec<HighlightWarning>>,
    pub diagnostics: Vec<Diagnostic>,
}

/// Per-query match count surfaced alongside the rendered text:
/// `query` is the raw token the user typed (after `-r` parsing) and
/// `matched` is the number of nodes the prune walk treated as roots
/// for that query.
pub struct PrunePerQueryDetail {
    pub query: String,
    pub matched: u32,
}

#[cfg(test)]
#[path = "pipeline_test.rs"]
mod pipeline_test;
