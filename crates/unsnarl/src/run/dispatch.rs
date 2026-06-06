//! Emitter construction and pipeline dispatch for a CLI run.

use unsnarl_emitter::Emitter;
use unsnarl_emitter_ir::IrEmitter;
use unsnarl_emitter_json::JsonEmitter;
use unsnarl_emitter_markdown::MarkdownEmitter;
use unsnarl_emitter_mermaid::strategy::MermaidStrategy;
use unsnarl_emitter_mermaid::theme::{ColorTheme, DARK_THEME, LIGHT_THEME};
use unsnarl_emitter_mermaid::MermaidEmitter;
use unsnarl_emitter_stats::StatsEmitter;
use unsnarl_ir::Language;
use unsnarl_oxc_boundary::parser::ParseError;
use unsnarl_plugin::UnsnarlPlugin;

use crate::cli::args::{Args, CliColorTheme, CliFormat, CliMermaidRenderer};
use crate::pipeline::{
    emit_ir_detailed, emit_json_detailed, emit_markdown_detailed, emit_mermaid_detailed,
    emit_stats_detailed, PipelineRunDetails, PipelineRunOptions,
};

use super::run_options::{depths_from_args, highlight_from_args, pruning_from_args};

/// Build the active emitter from `args.format` + renderer / theme.
/// Returns the single emitter the CLI dispatches to per run.
pub(super) fn build_emitter(args: &Args) -> Box<dyn Emitter> {
    let strategy = mermaid_strategy_for(args.mermaid_renderer.as_ref());
    let theme = color_theme_for(&args.color_theme);
    match args.format {
        CliFormat::Mermaid => Box::new(MermaidEmitter::new(strategy, theme)),
        CliFormat::Ir => Box::new(IrEmitter),
        CliFormat::Json => Box::new(JsonEmitter),
        CliFormat::Markdown => Box::new(MarkdownEmitter::new(MermaidEmitter::new(strategy, theme))),
        CliFormat::Stats => Box::new(StatsEmitter),
    }
}

/// Run the configured pipeline and return its detailed result.
pub(super) fn dispatch_pipeline(
    args: &Args,
    code: &str,
    source_path: &str,
    language: Language,
    plugins: &[&dyn UnsnarlPlugin],
) -> Result<PipelineRunDetails, ParseError> {
    let strategy = mermaid_strategy_for(args.mermaid_renderer.as_ref());
    let theme = color_theme_for(&args.color_theme);
    let pruning = pruning_from_args(args);
    let depths = depths_from_args(args);
    let highlight = highlight_from_args(args);
    let run = PipelineRunOptions {
        pruning: pruning.as_ref(),
        depths: Some(&depths),
        highlight: highlight.as_ref(),
        plugins,
    };
    match args.format {
        CliFormat::Mermaid => emit_mermaid_detailed(
            code,
            source_path,
            language,
            strategy,
            theme,
            args.debug,
            run,
        ),
        CliFormat::Ir => emit_ir_detailed(code, source_path, language, args.pretty_json, plugins),
        CliFormat::Json => emit_json_detailed(code, source_path, language, args.pretty_json, run),
        CliFormat::Markdown => emit_markdown_detailed(
            code,
            source_path,
            language,
            strategy,
            theme,
            args.debug,
            run,
        ),
        CliFormat::Stats => emit_stats_detailed(code, source_path, language, run),
    }
}

/// Default to elk when the flag is omitted.
fn mermaid_strategy_for(cli: Option<&CliMermaidRenderer>) -> MermaidStrategy {
    match cli {
        Some(CliMermaidRenderer::Dagre) => MermaidStrategy::Dagre,
        Some(CliMermaidRenderer::Elk) | None => MermaidStrategy::Elk,
    }
}

pub(super) fn color_theme_for(cli: &CliColorTheme) -> &'static ColorTheme {
    match cli {
        CliColorTheme::Dark => &DARK_THEME,
        CliColorTheme::Light => &LIGHT_THEME,
    }
}
