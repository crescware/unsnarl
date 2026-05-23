//! `uns` CLI argument definitions.
//!
//! The struct shape mirrors the TS parity surface
//! (`ts/src/cli/parsed-cli-options.ts`) via `#[derive(Serialize)]`
//! with `camelCase` field names. The actual argv parsing is hand-
//! rolled in [`parse_argv`] — we used to lean on `clap::Parser`
//! derive but that pulled ~190KB of `__text` into the release
//! binary (Command tree builder, help renderer, validator, error
//! formatter), all paid on every `uns` invocation.

use std::path::PathBuf;

use serde::Serialize;
use unsnarl_ir::NestingDepth;
use unsnarl_root_query::{GenerationCount, ParsedRootQuery};

pub mod cli_color_theme;
pub mod cli_format;
pub mod cli_language;
pub mod cli_mermaid_renderer;
pub mod collect_plugins;
pub mod help_text;
pub mod highlight;
pub mod parse_argv;
pub mod parse_error;
pub mod parse_generation_count;

pub use cli_color_theme::CliColorTheme;
pub use cli_format::CliFormat;
pub use cli_language::CliLanguage;
pub use cli_mermaid_renderer::CliMermaidRenderer;
pub use highlight::Highlight;
pub use parse_error::{ParseError, ParseErrorKind};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Args {
    /// Input file (positional, optional).
    pub file: Option<PathBuf>,
    /// Emitter format. Default: `Mermaid`.
    pub format: CliFormat,
    /// Pretty-print JSON. Default: `true`; `--no-pretty-json` flips it.
    pub pretty_json: bool,
    /// Layout engine override for Mermaid output.
    pub mermaid_renderer: Option<CliMermaidRenderer>,
    /// Color theme for Mermaid output. Default: `Dark`.
    pub color_theme: CliColorTheme,
    /// Read from stdin instead of a file.
    pub stdin: bool,
    /// Language label applied to stdin input. Default: `Ts`.
    pub stdin_lang: CliLanguage,
    /// Raw `-r` / `--roots` strings, folded into [`Self::roots`] by
    /// [`Self::finalize`].
    #[serde(skip)]
    pub(super) raw_roots: Vec<String>,
    /// Typed parsed-root-query list. Populated by [`Self::finalize`].
    pub roots: Vec<ParsedRootQuery>,
    /// `-H` / `--highlight` raw value: `None` = absent, `Some(None)`
    /// = present with no value, `Some(Some(s))` = present with an
    /// inline value. Folded into [`Self::highlight`] by
    /// [`Self::finalize`].
    #[serde(skip)]
    pub(super) raw_highlight: Option<Option<String>>,
    /// Typed highlight selection. Populated by [`Self::finalize`].
    pub highlight: Highlight,
    /// `-A` / `--descendants` generations.
    pub descendants: Option<GenerationCount>,
    /// `-B` / `--ancestors` generations.
    pub ancestors: Option<GenerationCount>,
    /// `-C` / `--context` generations (`-A` / `-B` shorthand).
    pub context: Option<GenerationCount>,
    /// `--depth` sugar: seeds `--depth-function` and `--depth-block`.
    pub depth: Option<NestingDepth>,
    /// Max function-scope nesting depth before scopes collapse.
    pub depth_function: Option<NestingDepth>,
    /// Max block-scope nesting depth before scopes collapse.
    pub depth_block: Option<NestingDepth>,
    /// `-o` / `--out-dir` (write `<dir>/<auto-name>.<ext>`).
    pub out_dir: Option<PathBuf>,
    /// `--out-file` (write `<path>`, no auto-naming).
    pub out_file: Option<PathBuf>,
    /// Basename derived from `-r` query tokens + `-A` / `-B` / `-C`
    /// (or the positional input file when no roots are given).
    /// Populated by [`Self::finalize`] when `-o` / `--out-dir` is
    /// set; otherwise `None`. Feeds the emitter's
    /// `<dir>/<basename>.<ext>` filename.
    pub derived_basename: Option<String>,
    /// `--debug` Mermaid label annotation.
    pub debug: bool,
    /// `--verbose` diagnostic stream. Skipped from serialization so
    /// it stays out of the `parsed-cli-options.ts` parity snapshot.
    #[serde(skip)]
    pub verbose: bool,
    /// Raw per-occurrence plugin lists collected from each `--plugin`
    /// flag, folded into [`Self::plugins`] by [`Self::finalize`].
    #[serde(skip)]
    pub(super) plugin_occurrences: Vec<Vec<String>>,
    /// Flattened-and-deduped plugin list.
    pub plugins: Vec<String>,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            file: None,
            format: CliFormat::Mermaid,
            pretty_json: true,
            mermaid_renderer: None,
            color_theme: CliColorTheme::Dark,
            stdin: false,
            stdin_lang: CliLanguage::Ts,
            raw_roots: Vec::new(),
            roots: Vec::new(),
            raw_highlight: None,
            highlight: Highlight::Absent,
            descendants: None,
            ancestors: None,
            context: None,
            depth: None,
            depth_function: None,
            depth_block: None,
            out_dir: None,
            out_file: None,
            derived_basename: None,
            debug: false,
            verbose: false,
            plugin_occurrences: Vec::new(),
            plugins: Vec::new(),
        }
    }
}

impl Args {
    /// Inherent parser entry, equivalent to the previous
    /// clap-derived `try_parse_from`. The full parsing + finalisation
    /// pipeline lives in [`parse_argv::parse_argv`].
    pub fn try_parse_from<I, T>(itr: I) -> Result<Self, ParseError>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString>,
    {
        parse_argv::parse_argv(itr)
    }

    /// Inherent parser shortcut for `main`. Mirrors the previous
    /// clap-derived `Args::parse` (which delegated to
    /// `Error::exit`): if parsing fails, render the message and exit
    /// with the appropriate code rather than returning to the caller.
    pub fn parse() -> Self {
        Self::try_parse_from(std::env::args_os()).unwrap_or_else(|e| e.exit())
    }
}

#[cfg(test)]
#[path = "args_test.rs"]
mod args_test;
