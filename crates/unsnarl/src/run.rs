use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

use unsnarl_emitter::DEFAULT_DEPTH;
use unsnarl_emitter_mermaid::strategy::MermaidStrategy;
use unsnarl_emitter_mermaid::theme::{ColorTheme, DARK_THEME, LIGHT_THEME};
use unsnarl_ir::nesting_kind::NestingDepths;
use unsnarl_plugin::UnsnarlPlugin;
use unsnarl_visual_graph::highlight::HighlightRunOptions;

use crate::cli::args::{
    Args, CliColorTheme, CliFormat, CliLanguage, CliMermaidRenderer, Highlight,
};
use crate::cli::run_cli::emit_out_flag_notice;
use crate::pipeline::plugin::default_registry;
use crate::pipeline::prune::PruningRunOptions;
use crate::pipeline::{
    emit_ir_text, emit_json_text, emit_markdown_text, emit_mermaid_text, emit_stats_text,
    language_for_path, PipelineRunOptions,
};

pub fn run(args: &Args) {
    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut out = stdout.lock();
    let mut err = stderr.lock();
    run_to(args, &mut out, &mut err);
}

pub(crate) fn run_to(args: &Args, out: &mut dyn Write, err: &mut dyn Write) {
    emit_out_flag_notice(args.out_dir.as_deref(), err);
    let registry = default_registry();
    let plugins: Vec<&dyn UnsnarlPlugin> = match registry.activate_all(&args.plugins) {
        Ok(v) => v,
        Err(e) => {
            writeln!(err, "uns: error: {e}").ok();
            return;
        }
    };
    let handler = select_handler(&args.format);
    handler(args, &plugins, out, err);
}

type Handler = fn(&Args, &[&dyn UnsnarlPlugin], &mut dyn Write, &mut dyn Write);

fn select_handler(format: &CliFormat) -> Handler {
    match format {
        CliFormat::Mermaid => emit_mermaid,
        CliFormat::Ir => emit_ir,
        CliFormat::Json => emit_json,
        CliFormat::Markdown => emit_markdown,
        CliFormat::Stats => emit_stats,
    }
}

fn emit_mermaid(
    args: &Args,
    plugins: &[&dyn UnsnarlPlugin],
    out: &mut dyn Write,
    err: &mut dyn Write,
) {
    let Some((code, source_path, language)) = read_source(args, err) else {
        return;
    };
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
    match emit_mermaid_text(
        &code,
        &source_path,
        language,
        strategy,
        theme,
        args.debug,
        run,
    ) {
        Ok(text) => {
            out.write_all(text.as_bytes())
                .expect("write mermaid output");
        }
        Err(e) => {
            writeln!(err, "uns: error: {e}").expect("write mermaid error");
        }
    }
}

/// Default generations used when the user gives `-r/--roots` but no
/// `-A`/`-B`/`-C`. Mirrors `DEFAULT_GENERATIONS` in
/// `ts/src/cli/args/default-generations.ts`.
const DEFAULT_GENERATIONS: u32 = 10;

/// Translate the CLI's `--depth` / `--depth-function` / `--depth-block`
/// flags into a [`NestingDepths`]. Mirrors `resolveDepths` in
/// `ts/src/cli/run-cli/normalize-cli-options.ts`: `--depth <N>` seeds
/// both axes, then `--depth-function` / `--depth-block` override
/// their respective halves. Unset fields fall back to
/// [`DEFAULT_DEPTH`].
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
/// [`HighlightRunOptions`]. Mirrors the TS `buildRunOpts` mapping:
/// `Highlight::Absent` -> `None`, `Highlight::NoValue` -> `Roots`
/// (the highlight follows `-r/--roots`), `Highlight::Value(queries)`
/// -> `Queries(queries)`.
fn highlight_from_args(args: &Args) -> Option<HighlightRunOptions> {
    match &args.highlight {
        Highlight::Absent => None,
        Highlight::NoValue => Some(HighlightRunOptions::Roots),
        Highlight::Value(queries) => Some(HighlightRunOptions::Queries(queries.clone())),
    }
}

/// Translate the CLI's `-r/-A/-B/-C` flags into the pipeline's
/// [`PruningRunOptions`]. Mirrors `resolveGenerations` in
/// `ts/src/cli/run-cli/resolve-generations.ts`. Returns `None` when
/// no `-r` queries are present so the pipeline skips the prune step
/// entirely (matching the TS guard `pruning ?? null`).
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

/// Default to elk when the flag is omitted. Matches the TS pipeline
/// default at `ts/src/cli/args/cli-mermaid-renderer.ts`.
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

fn emit_ir(args: &Args, plugins: &[&dyn UnsnarlPlugin], out: &mut dyn Write, err: &mut dyn Write) {
    let Some((code, source_path, language)) = read_source(args, err) else {
        return;
    };
    match emit_ir_text(&code, &source_path, language, args.pretty_json, plugins) {
        Ok(text) => {
            out.write_all(text.as_bytes()).expect("write ir output");
        }
        Err(e) => {
            writeln!(err, "uns: error: {e}").expect("write ir error");
        }
    }
}

fn emit_json(
    args: &Args,
    plugins: &[&dyn UnsnarlPlugin],
    out: &mut dyn Write,
    err: &mut dyn Write,
) {
    let Some((code, source_path, language)) = read_source(args, err) else {
        return;
    };
    let pruning = pruning_from_args(args);
    let depths = depths_from_args(args);
    let highlight = highlight_from_args(args);
    let run = PipelineRunOptions {
        pruning: pruning.as_ref(),
        depths: Some(&depths),
        highlight: highlight.as_ref(),
        plugins,
    };
    match emit_json_text(&code, &source_path, language, args.pretty_json, run) {
        Ok(text) => {
            out.write_all(text.as_bytes()).expect("write json output");
        }
        Err(e) => {
            writeln!(err, "uns: error: {e}").expect("write json error");
        }
    }
}

fn emit_markdown(
    args: &Args,
    plugins: &[&dyn UnsnarlPlugin],
    out: &mut dyn Write,
    err: &mut dyn Write,
) {
    let Some((code, source_path, language)) = read_source(args, err) else {
        return;
    };
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
    match emit_markdown_text(
        &code,
        &source_path,
        language,
        strategy,
        theme,
        args.debug,
        run,
    ) {
        Ok(text) => {
            out.write_all(text.as_bytes())
                .expect("write markdown output");
        }
        Err(e) => {
            writeln!(err, "uns: error: {e}").expect("write markdown error");
        }
    }
}

fn emit_stats(
    args: &Args,
    plugins: &[&dyn UnsnarlPlugin],
    out: &mut dyn Write,
    err: &mut dyn Write,
) {
    let Some((code, source_path, language)) = read_source(args, err) else {
        return;
    };
    let pruning = pruning_from_args(args);
    let depths = depths_from_args(args);
    let highlight = highlight_from_args(args);
    let run = PipelineRunOptions {
        pruning: pruning.as_ref(),
        depths: Some(&depths),
        highlight: highlight.as_ref(),
        plugins,
    };
    match emit_stats_text(&code, &source_path, language, run) {
        Ok(text) => {
            out.write_all(text.as_bytes()).expect("write stats output");
        }
        Err(e) => {
            writeln!(err, "uns: error: {e}").expect("write stats error");
        }
    }
}

/// Pull the source to feed the pipeline plus the path / language pair
/// the emitter records inside `SerializedSource`. Mirrors the
/// `--stdin` / file argument split in `ts/src/cli/`. Returns `None`
/// and writes a CLI-style error to `err` when neither input is
/// available (the same behaviour as the TS commander layer).
fn read_source(args: &Args, err: &mut dyn Write) -> Option<(String, String, unsnarl_ir::Language)> {
    if args.stdin {
        let mut buf = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut buf) {
            writeln!(err, "uns: error: failed to read stdin: {e}").ok();
            return None;
        }
        let language = match args.stdin_lang {
            CliLanguage::Ts => unsnarl_ir::Language::Ts,
            CliLanguage::Tsx => unsnarl_ir::Language::Tsx,
            CliLanguage::Js => unsnarl_ir::Language::Js,
            CliLanguage::Jsx => unsnarl_ir::Language::Jsx,
        };
        return Some((buf, "<stdin>".to_string(), language));
    }
    let Some(file) = args.file.as_ref() else {
        writeln!(
            err,
            "uns: error: no input file (pass a positional path or --stdin)"
        )
        .ok();
        return None;
    };
    let code = match fs::read_to_string(file) {
        Ok(c) => c,
        Err(e) => {
            writeln!(err, "uns: error: failed to read {}: {e}", file.display()).ok();
            return None;
        }
    };
    let source_path = file.to_string_lossy().into_owned();
    let Some(language) = language_for_path(&source_path) else {
        let ext = Path::new(&source_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("(none)");
        writeln!(err, "uns: error: unsupported language extension: {ext}").ok();
        return None;
    };
    Some((code, source_path, language))
}

#[cfg(test)]
#[path = "run_test.rs"]
mod run_test;
