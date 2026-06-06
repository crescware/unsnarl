//! `mermaid` format emit: route the parsed IR through
//! [`MermaidEmitter`] with the caller-chosen strategy / theme.

use unsnarl_emitter_mermaid::strategy::MermaidStrategy;
use unsnarl_emitter_mermaid::theme::ColorTheme;
use unsnarl_emitter_mermaid::MermaidEmitter;
use unsnarl_ir::Language;
use unsnarl_oxc_boundary::parser::ParseError;

use super::emit_pruning_aware_with::{emit_pruning_aware_with, EmitOptionsBase};
use super::{PipelineRunDetails, PipelineRunOptions};

/// Same as [`super::emit_ir::emit_ir_text`] but routes the parsed IR
/// through [`MermaidEmitter`]. The strategy / theme decisions are made
/// by the caller (CLI flags `--mermaid-renderer` / `--color-theme`)
/// rather than baked into the pipeline.
pub fn emit_mermaid_text(
    code: &str,
    source_path: &str,
    language: Language,
    strategy: MermaidStrategy,
    theme: &'static ColorTheme,
    debug: bool,
    run: PipelineRunOptions<'_>,
) -> Result<String, ParseError> {
    emit_mermaid_detailed(code, source_path, language, strategy, theme, debug, run).map(|d| d.text)
}

/// Detailed variant of [`emit_mermaid_text`].
pub fn emit_mermaid_detailed(
    code: &str,
    source_path: &str,
    language: Language,
    strategy: MermaidStrategy,
    theme: &'static ColorTheme,
    debug: bool,
    run: PipelineRunOptions<'_>,
) -> Result<PipelineRunDetails, ParseError> {
    let emitter = MermaidEmitter::new(strategy, theme);
    emit_pruning_aware_with(
        code,
        source_path,
        language,
        &emitter,
        run,
        EmitOptionsBase {
            pretty_json: false,
            debug,
        },
    )
}
