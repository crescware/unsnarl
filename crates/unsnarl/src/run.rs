use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

use unsnarl_emitter_mermaid::strategy::MermaidStrategy;
use unsnarl_emitter_mermaid::theme::{ColorTheme, DARK_THEME, LIGHT_THEME};

use crate::cli::args::{Args, CliColorTheme, CliFormat, CliLanguage, CliMermaidRenderer};
use crate::cli::run_cli::emit_out_flag_notice;
use crate::pipeline::{emit_ir_text, emit_json_text, emit_mermaid_text, language_for_path};

pub fn run(args: &Args) {
    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut out = stdout.lock();
    let mut err = stderr.lock();
    run_to(args, &mut out, &mut err);
}

pub(crate) fn run_to(args: &Args, out: &mut dyn Write, err: &mut dyn Write) {
    emit_out_flag_notice(args.out_dir.as_deref(), err);
    let handler = select_handler(&args.format);
    handler(args, out, err);
}

type Handler = fn(&Args, &mut dyn Write, &mut dyn Write);

fn select_handler(format: &CliFormat) -> Handler {
    match format {
        CliFormat::Mermaid => emit_mermaid,
        CliFormat::Ir => emit_ir,
        CliFormat::Json => emit_json,
        CliFormat::Markdown => emit_markdown,
        CliFormat::Stats => emit_stats,
    }
}

fn emit_mermaid(args: &Args, out: &mut dyn Write, err: &mut dyn Write) {
    let Some((code, source_path, language)) = read_source(args, err) else {
        return;
    };
    let strategy = mermaid_strategy_for(args.mermaid_renderer.as_ref());
    let theme = color_theme_for(&args.color_theme);
    match emit_mermaid_text(&code, &source_path, language, strategy, theme, args.debug) {
        Ok(text) => {
            out.write_all(text.as_bytes())
                .expect("write mermaid output");
        }
        Err(e) => {
            writeln!(err, "uns: error: {e}").expect("write mermaid error");
        }
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

fn emit_ir(args: &Args, out: &mut dyn Write, err: &mut dyn Write) {
    let Some((code, source_path, language)) = read_source(args, err) else {
        return;
    };
    match emit_ir_text(&code, &source_path, language, args.pretty_json) {
        Ok(text) => {
            out.write_all(text.as_bytes()).expect("write ir output");
        }
        Err(e) => {
            writeln!(err, "uns: error: {e}").expect("write ir error");
        }
    }
}

fn emit_json(args: &Args, out: &mut dyn Write, err: &mut dyn Write) {
    let Some((code, source_path, language)) = read_source(args, err) else {
        return;
    };
    match emit_json_text(&code, &source_path, language, args.pretty_json) {
        Ok(text) => {
            out.write_all(text.as_bytes()).expect("write json output");
        }
        Err(e) => {
            writeln!(err, "uns: error: {e}").expect("write json error");
        }
    }
}

fn emit_markdown(args: &Args, out: &mut dyn Write, _err: &mut dyn Write) {
    emit_stub("markdown emitter", args, out);
}

fn emit_stats(args: &Args, out: &mut dyn Write, _err: &mut dyn Write) {
    emit_stub("stats emitter", args, out);
}

fn emit_stub(label: &str, args: &Args, out: &mut dyn Write) {
    let json = serde_json::to_string_pretty(args).expect("serialize CLI args");
    writeln!(out, "Not implemented yet: {label}").expect("write stub label");
    writeln!(out, "{json}").expect("write CLI args JSON");
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
