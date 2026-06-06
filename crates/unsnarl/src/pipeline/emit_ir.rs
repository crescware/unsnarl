//! `ir` format emit: render the serialized IR directly via
//! [`IrEmitter`] (no visual-graph build).

use unsnarl_emitter::{EmitOptions, Emitter};
use unsnarl_emitter_ir::IrEmitter;
use unsnarl_ir::Language;
use unsnarl_oxc_boundary::parser::ParseError;
use unsnarl_plugin::UnsnarlPlugin;

use crate::pipeline::plugin::apply_plugins;

use super::serialize_ir::serialize_ir;
use super::PipelineRunDetails;

/// Run the full parse -> analyse -> serialize -> emit pipeline for the
/// `ir` format and return the rendered text.
///
/// The `code` slice is owned by the caller; the AST and IR are built
/// inside this function and dropped before it returns, so no oxc
/// lifetime escapes to the caller.
pub fn emit_ir_text(
    code: &str,
    source_path: &str,
    language: Language,
    pretty_json: bool,
    plugins: &[&dyn UnsnarlPlugin],
) -> Result<String, ParseError> {
    emit_ir_detailed(code, source_path, language, pretty_json, plugins).map(|d| d.text)
}

/// Detailed variant of [`emit_ir_text`]. Returns the rendered text
/// together with the analyser diagnostics so the CLI orchestration
/// can emit `var`-detected warnings on stderr. The IR branch leaves
/// `pruning` / `resolutions` at `None`.
pub fn emit_ir_detailed(
    code: &str,
    source_path: &str,
    language: Language,
    pretty_json: bool,
    plugins: &[&dyn UnsnarlPlugin],
) -> Result<PipelineRunDetails, ParseError> {
    let serialized = apply_plugins(serialize_ir(code, source_path, language)?, plugins);
    let diagnostics = serialized.diagnostics.clone();
    let text = {
        let _span = unsnarl_instrumentation::span!("emit", format = "ir");
        IrEmitter.emit(
            &serialized,
            &EmitOptions {
                pretty_json,
                debug: false,
                pruned_graph: None,
                resolutions: None,
                depths: None,
                highlight_ids: None,
                highlight_point_ids: None,
                highlight: None,
            },
        )
    };
    Ok(PipelineRunDetails {
        text,
        pruning: None,
        resolutions: None,
        highlight_warnings: None,
        diagnostics,
    })
}
