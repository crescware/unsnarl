use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::CommandFactory;
use unsnarl_boundary_eslint_scope::parser::ParseError;
use unsnarl_emitter::{Emitter, DEFAULT_DEPTH};
use unsnarl_emitter_ir::IrEmitter;
use unsnarl_emitter_json::JsonEmitter;
use unsnarl_emitter_markdown::MarkdownEmitter;
use unsnarl_emitter_mermaid::strategy::MermaidStrategy;
use unsnarl_emitter_mermaid::theme::{ColorTheme, DARK_THEME, LIGHT_THEME};
use unsnarl_emitter_mermaid::MermaidEmitter;
use unsnarl_emitter_stats::StatsEmitter;
use unsnarl_ir::nesting_kind::NestingDepths;
use unsnarl_ir::Language;
use unsnarl_plugin::UnsnarlPlugin;
use unsnarl_visual_graph::highlight::HighlightRunOptions;

use crate::cli::args::{
    Args, CliColorTheme, CliFormat, CliLanguage, CliMermaidRenderer, Highlight,
};
use crate::cli::run_cli::{
    calc_source::{calc_source, ExecuteSource},
    cli_usage_error::CliUsageError,
    emit_analyzer_warnings, emit_out_flag_notice, emit_pruning_warnings, emit_resolution_notices,
    resolve_output_path, write_output,
};
use crate::pipeline::plugin::default_registry;
use crate::pipeline::prune::PruningRunOptions;
use crate::pipeline::{
    emit_ir_detailed, emit_json_detailed, emit_markdown_detailed, emit_mermaid_detailed,
    emit_stats_detailed, language_for_path, PipelineRunDetails, PipelineRunOptions,
};

/// Entry point for the `uns` binary. Returns the process exit code so
/// the binary can propagate it through `main()`.
pub fn run(args: &Args) -> ExitCode {
    if args.verbose {
        init_verbose_tracing();
    }
    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut out = stdout.lock();
    let mut err = stderr.lock();
    let mut stdin = io::stdin();
    let code = run_to(args, &mut stdin, &mut out, &mut err);
    ExitCode::from(code)
}

/// Install a stderr `tracing-subscriber` at INFO level, emitting span
/// close events so each pipeline stage prints its elapsed time
/// alongside the in-stage `info!` payload events (input/output
/// sizes, IR / graph counts) needed to spot bottlenecks. Called only
/// when `--verbose` is set. Wrapped in `try_init` so a stray double
/// install (e.g. an embedder calling `run` twice) is a no-op rather
/// than a panic.
fn init_verbose_tracing() {
    use tracing_subscriber::fmt::format::FmtSpan;
    let _ = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .try_init();
}

/// Library-level orchestration. The CLI binary calls this with the
/// process stdin / stdout / stderr; the unit tests in `run_test.rs`
/// supply in-memory readers / writers so they can assert on the
/// emitted bytes without touching the host environment.
pub(crate) fn run_to(
    args: &Args,
    stdin: &mut dyn io::Read,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> u8 {
    tracing::info!(
        format = ?args.format,
        stdin = args.stdin,
        out_file = ?args.out_file,
        out_dir = ?args.out_dir,
        roots = args.roots.len(),
        requested_plugins = args.plugins.len(),
        "run starting",
    );

    emit_out_flag_notice(args.out_dir.as_deref(), err);

    let registry = default_registry();
    let plugins: Vec<&dyn UnsnarlPlugin> = {
        let _span = tracing::info_span!("activate_plugins").entered();
        match registry.activate_all(&args.plugins) {
            Ok(v) => v,
            Err(e) => {
                writeln!(err, "error: {e}").ok();
                return 1;
            }
        }
    };

    let source = match calc_source(args, stdin, render_help) {
        Ok(s) => s,
        Err(e) => return handle_cli_usage_error(&e, err),
    };

    let emitter = build_emitter(args);
    let output_path = resolve_output_path(args, emitter.as_ref());

    let (code, source_path, language) = {
        let _span = tracing::info_span!("read_source").entered();
        match read_source_text(&source, err) {
            Some(t) => t,
            None => return 1,
        }
    };
    tracing::info!(
        path = %source_path,
        bytes = code.len(),
        language = language_str(language),
        plugins = plugins.len(),
        "source loaded",
    );

    let details = {
        let _span = tracing::info_span!("pipeline", format = ?args.format).entered();
        match dispatch_pipeline(args, &code, &source_path, language, &plugins) {
            Ok(d) => d,
            Err(e) => return handle_parse_error(&e, err),
        }
    };
    tracing::info!(
        output_bytes = details.text.len(),
        diagnostics = details.diagnostics.len(),
        "pipeline result",
    );

    emit_resolution_notices(details.resolutions.as_deref(), err);
    emit_pruning_warnings(details.pruning.as_deref(), err);
    emit_analyzer_warnings(&details.diagnostics, err);

    match write_output(output_path.as_deref(), &details.text, out) {
        Ok(()) => 0,
        Err(e) => {
            writeln!(err, "error: failed to write output: {e}").ok();
            1
        }
    }
}

fn render_help() -> String {
    Args::command().render_help().to_string()
}

fn handle_cli_usage_error(e: &CliUsageError, err: &mut dyn Write) -> u8 {
    writeln!(err, "error: {}", e.message).ok();
    if let Some(help) = &e.help {
        write!(err, "{help}").ok();
    }
    2
}

fn handle_parse_error(e: &ParseError, err: &mut dyn Write) -> u8 {
    writeln!(err, "parse error: {e}").ok();
    1
}

/// Build the active emitter from `args.format` + renderer / theme.
/// Returns the single emitter the CLI dispatches to per run.
fn build_emitter(args: &Args) -> Box<dyn Emitter> {
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
fn dispatch_pipeline(
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

/// Default generations used when the user gives `-r/--roots` but no
/// `-A`/`-B`/`-C`.
const DEFAULT_GENERATIONS: u32 = 10;

/// Translate the CLI's `--depth` / `--depth-function` / `--depth-block`
/// flags into a [`NestingDepths`]. `--depth <N>` seeds both axes,
/// then `--depth-function` / `--depth-block` override their
/// respective halves. Unset fields fall back to [`DEFAULT_DEPTH`].
fn depths_from_args(args: &Args) -> NestingDepths {
    let general = args.depth.unwrap_or(DEFAULT_DEPTH);
    let function = args.depth_function.unwrap_or(general);
    let block = args.depth_block.unwrap_or(general);
    NestingDepths {
        function,
        r#if: block,
        r#for: block,
        r#while: block,
        switch: block,
        try_catch_finally: block,
        block,
    }
}

/// Translate the CLI's `-H` / `--highlight` flag into the pipeline's
/// [`HighlightRunOptions`]: `Highlight::Absent` -> `None`,
/// `Highlight::NoValue` -> `Roots` (the highlight follows
/// `-r/--roots`), `Highlight::Value(queries)` -> `Queries(queries)`.
fn highlight_from_args(args: &Args) -> Option<HighlightRunOptions> {
    match &args.highlight {
        Highlight::Absent => None,
        Highlight::NoValue => Some(HighlightRunOptions::Roots),
        Highlight::Value(queries) => Some(HighlightRunOptions::Queries(queries.clone())),
    }
}

/// Translate the CLI's `-r/-A/-B/-C` flags into the pipeline's
/// [`PruningRunOptions`]. Returns `None` when no `-r` queries are
/// present so the pipeline skips the prune step entirely.
fn pruning_from_args(args: &Args) -> Option<PruningRunOptions> {
    if args.roots.is_empty() {
        return None;
    }
    let no_flag = args.descendants.is_none() && args.ancestors.is_none() && args.context.is_none();
    // grep -A/-B semantics: an explicit -A says "I asked for
    // descendants only," so the unspecified side falls to 0 instead
    // of the symmetric DEFAULT. -C still fills in for whichever
    // side is unspecified.
    let fallback = if no_flag {
        DEFAULT_GENERATIONS
    } else {
        args.context.map(|g| g.0).unwrap_or(0)
    };
    let descendants = args.descendants.map(|g| g.0).unwrap_or(fallback);
    let ancestors = args.ancestors.map(|g| g.0).unwrap_or(fallback);
    let roots: Vec<_> = args.roots.iter().map(clone_parsed_root_query).collect();
    Some(PruningRunOptions {
        roots,
        descendants,
        ancestors,
    })
}

fn clone_parsed_root_query(
    q: &unsnarl_root_query::ParsedRootQuery,
) -> unsnarl_root_query::ParsedRootQuery {
    use unsnarl_root_query::ParsedRootQuery;
    match q {
        ParsedRootQuery::Line { line, raw } => ParsedRootQuery::Line {
            line: *line,
            raw: raw.clone(),
        },
        ParsedRootQuery::LineName { line, name, raw } => ParsedRootQuery::LineName {
            line: *line,
            name: name.clone(),
            raw: raw.clone(),
        },
        ParsedRootQuery::Range { start, end, raw } => ParsedRootQuery::Range {
            start: *start,
            end: *end,
            raw: raw.clone(),
        },
        ParsedRootQuery::RangeName {
            start,
            end,
            name,
            raw,
        } => ParsedRootQuery::RangeName {
            start: *start,
            end: *end,
            name: name.clone(),
            raw: raw.clone(),
        },
        ParsedRootQuery::Name { name, raw } => ParsedRootQuery::Name {
            name: name.clone(),
            raw: raw.clone(),
        },
        ParsedRootQuery::LineOrName { line, name, raw } => ParsedRootQuery::LineOrName {
            line: *line,
            name: name.clone(),
            raw: raw.clone(),
        },
    }
}

/// Default to elk when the flag is omitted.
fn mermaid_strategy_for(cli: Option<&CliMermaidRenderer>) -> MermaidStrategy {
    match cli {
        Some(CliMermaidRenderer::Dagre) => MermaidStrategy::Dagre,
        Some(CliMermaidRenderer::Elk) | None => MermaidStrategy::Elk,
    }
}

fn color_theme_for(cli: &CliColorTheme) -> &'static ColorTheme {
    match cli {
        CliColorTheme::Dark => &DARK_THEME,
        CliColorTheme::Light => &LIGHT_THEME,
    }
}

/// Materialise the source bytes the pipeline runs against, together
/// with the path / language pair the emitter records inside
/// `SerializedSource`. Stdin contents are labelled `stdin.<lang>` so
/// the IR carries a stable, lang-aware path; file inputs map to the
/// on-disk path and the extension drives language detection.
fn read_source_text(
    src: &ExecuteSource,
    err: &mut dyn Write,
) -> Option<(String, String, Language)> {
    match src {
        ExecuteSource::Stdin { text, lang } => {
            let language = language_for_cli(lang);
            let source_path = format!("stdin.{}", cli_language_str(lang));
            Some((text.clone(), source_path, language))
        }
        ExecuteSource::File { path } => read_source_file(path, err),
    }
}

fn read_source_file(path: &PathBuf, err: &mut dyn Write) -> Option<(String, String, Language)> {
    let code = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            writeln!(err, "error: failed to read {}: {e}", path.display()).ok();
            return None;
        }
    };
    let source_path = path.to_string_lossy().into_owned();
    let Some(language) = language_for_path(&source_path) else {
        let ext = Path::new(&source_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("(none)");
        writeln!(err, "error: unsupported language extension: {ext}").ok();
        return None;
    };
    Some((code, source_path, language))
}

fn language_for_cli(lang: &CliLanguage) -> Language {
    match lang {
        CliLanguage::Ts => Language::Ts,
        CliLanguage::Tsx => Language::Tsx,
        CliLanguage::Js => Language::Js,
        CliLanguage::Jsx => Language::Jsx,
    }
}

fn cli_language_str(lang: &CliLanguage) -> &'static str {
    match lang {
        CliLanguage::Ts => "ts",
        CliLanguage::Tsx => "tsx",
        CliLanguage::Js => "js",
        CliLanguage::Jsx => "jsx",
    }
}

fn language_str(lang: Language) -> &'static str {
    match lang {
        Language::Ts => "ts",
        Language::Tsx => "tsx",
        Language::Js => "js",
        Language::Jsx => "jsx",
    }
}

#[cfg(test)]
#[path = "run_test.rs"]
mod run_test;
