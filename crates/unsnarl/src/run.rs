use std::io::{self, Write};
use std::process::ExitCode;

use unsnarl_ir::Language;
use unsnarl_oxc_boundary::parser::ParseError;
use unsnarl_plugin::UnsnarlPlugin;

use crate::cli::args::Args;
use crate::cli::run_cli::{
    calc_source::calc_source, cli_usage_error::CliUsageError, emit_analyzer_warnings,
    emit_highlight_warnings, emit_out_flag_notice, emit_pruning_warnings, emit_resolution_notices,
    emit_stats_deprecation_notice, resolve_output_path, write_output,
};
use crate::pipeline::plugin::default_registry;

mod dispatch;
mod read_source;
mod run_options;

use dispatch::{build_emitter, dispatch_pipeline};
use read_source::read_source_text;

#[cfg(test)]
use crate::cli::args::CliColorTheme;
#[cfg(test)]
use dispatch::color_theme_for;
#[cfg(test)]
use run_options::{depths_from_args, pruning_from_args, DEFAULT_GENERATIONS};
#[cfg(test)]
use unsnarl_emitter::DEFAULT_DEPTH;

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
/// install (e.g. an embedder calling `run` twice, or `main` having
/// already installed it to time the pre-`Args::parse` window) is a
/// no-op rather than a panic.
pub fn init_verbose_tracing() {
    use tracing_subscriber::fmt::format::FmtSpan;
    // Flip the process-global verbose gate so the `unsnarl-instrumentation`
    // probes start recording. The gate has no effect on `tracing` itself
    // (the subscriber below handles that side), but it short-circuits the
    // `Instant::now` / atomic-counter / `TimingScope` primitives scattered
    // across the workspace, which would otherwise pay their full cost on
    // every `uns` run regardless of `--verbose`.
    unsnarl_instrumentation::set_verbose(true);
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
        let _span = unsnarl_instrumentation::span!("activate_plugins");
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
        let _span = unsnarl_instrumentation::span!("read_source");
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
        let _span = unsnarl_instrumentation::span!("pipeline", format = ?args.format);
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
    emit_highlight_warnings(details.highlight_warnings.as_deref(), err);
    emit_analyzer_warnings(&details.diagnostics, err);
    // Deprecation notice goes last, after every other warning, so the
    // stats run closes with it. Post-pipeline placement also means it
    // only fires once stats output is actually produced.
    emit_stats_deprecation_notice(&args.format, err);

    match write_output(output_path.as_deref(), &details.text, out) {
        Ok(()) => 0,
        Err(e) => {
            writeln!(err, "error: failed to write output: {e}").ok();
            1
        }
    }
}

fn render_help() -> String {
    crate::cli::args::help_text::HELP_TEXT.to_string()
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
