//! `markdown` format emit: route the parsed IR through
//! [`MarkdownEmitter`], which embeds a configured [`MermaidEmitter`].

use unsnarl_emitter_markdown::MarkdownEmitter;
use unsnarl_emitter_mermaid::strategy::MermaidStrategy;
use unsnarl_emitter_mermaid::theme::ColorTheme;
use unsnarl_emitter_mermaid::MermaidEmitter;
use unsnarl_ir::Language;
use unsnarl_oxc_boundary::parser::ParseError;

use super::emit_pruning_aware_with::{emit_pruning_aware_with, EmitOptionsBase};
use super::{PipelineRunDetails, PipelineRunOptions};

/// Same as [`super::emit_ir::emit_ir_text`] but routes the parsed IR
/// through [`MarkdownEmitter`]. The markdown emitter composes a
/// configured [`MermaidEmitter`] so the caller picks the renderer /
/// theme on behalf of the embedded `## Mermaid` block — the same
/// configured `MermaidEmitter` instance that `-f mermaid` uses directly.
pub fn emit_markdown_text(
    code: &str,
    source_path: &str,
    language: Language,
    strategy: MermaidStrategy,
    theme: &'static ColorTheme,
    debug: bool,
    run: PipelineRunOptions<'_>,
) -> Result<String, ParseError> {
    emit_markdown_detailed(code, source_path, language, strategy, theme, debug, run).map(|d| d.text)
}

/// Detailed variant of [`emit_markdown_text`].
pub fn emit_markdown_detailed(
    code: &str,
    source_path: &str,
    language: Language,
    strategy: MermaidStrategy,
    theme: &'static ColorTheme,
    debug: bool,
    run: PipelineRunOptions<'_>,
) -> Result<PipelineRunDetails, ParseError> {
    let mermaid = MermaidEmitter::new(strategy, theme);
    let emitter = MarkdownEmitter::new(mermaid);
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
