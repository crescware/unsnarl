//! `json` format emit: build a `VisualGraph` and serialise it as JSON
//! via [`JsonEmitter`].

use unsnarl_emitter_json::JsonEmitter;
use unsnarl_ir::Language;
use unsnarl_oxc_boundary::parser::ParseError;

use super::emit_pruning_aware_with::{emit_pruning_aware_with, EmitOptionsBase};
use super::{PipelineRunDetails, PipelineRunOptions};

/// Same as [`super::emit_ir::emit_ir_text`] but routes the parsed IR
/// through [`JsonEmitter`], which builds a `VisualGraph` and serialises
/// it as JSON. Used by the `-f json` CLI handler and the parity
/// harness's `expected.json` comparison.
pub fn emit_json_text(
    code: &str,
    source_path: &str,
    language: Language,
    pretty_json: bool,
    run: PipelineRunOptions<'_>,
) -> Result<String, ParseError> {
    emit_json_detailed(code, source_path, language, pretty_json, run).map(|d| d.text)
}

/// Detailed variant of [`emit_json_text`]. Returns text + warnings
/// for the CLI orchestration to surface on stderr.
pub fn emit_json_detailed(
    code: &str,
    source_path: &str,
    language: Language,
    pretty_json: bool,
    run: PipelineRunOptions<'_>,
) -> Result<PipelineRunDetails, ParseError> {
    emit_pruning_aware_with(
        code,
        source_path,
        language,
        &JsonEmitter,
        run,
        EmitOptionsBase {
            pretty_json,
            debug: false,
        },
    )
}
