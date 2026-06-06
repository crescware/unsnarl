//! `stats` format emit: build a `VisualGraph` and render a wc-like TSV
//! table of per-node edge counts via [`StatsEmitter`].

use unsnarl_emitter_stats::StatsEmitter;
use unsnarl_ir::Language;
use unsnarl_oxc_boundary::parser::ParseError;

use super::emit_pruning_aware_with::{emit_pruning_aware_with, EmitOptionsBase};
use super::{PipelineRunDetails, PipelineRunOptions};

/// Same as [`super::emit_ir::emit_ir_text`] but routes the parsed IR
/// through [`StatsEmitter`], which builds a `VisualGraph` and renders a
/// wc-like TSV table of per-node edge counts. Used by the `-f stats`
/// CLI handler and the parity harness's `expected.stats` comparison.
pub fn emit_stats_text(
    code: &str,
    source_path: &str,
    language: Language,
    run: PipelineRunOptions<'_>,
) -> Result<String, ParseError> {
    emit_stats_detailed(code, source_path, language, run).map(|d| d.text)
}

/// Detailed variant of [`emit_stats_text`].
pub fn emit_stats_detailed(
    code: &str,
    source_path: &str,
    language: Language,
    run: PipelineRunOptions<'_>,
) -> Result<PipelineRunDetails, ParseError> {
    emit_pruning_aware_with(
        code,
        source_path,
        language,
        &StatsEmitter,
        run,
        EmitOptionsBase {
            pretty_json: false,
            debug: false,
        },
    )
}
