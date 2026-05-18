//! `uns` CLI argument definitions. Mirrors the TS commander source at
//! `ts/src/cli/args/build-command.ts` and the per-option files alongside
//! it.

use clap::Parser;
use serde::Serialize;
use unsnarl_root_query::{parse_root_queries, ParsedRootQuery};

use crate::cli::run_cli::derive_output_basename;

pub mod cli_color_theme;
pub mod cli_format;
pub mod cli_language;
pub mod cli_mermaid_renderer;
pub mod collect_plugins;
pub mod highlight;
pub mod parse_generation_count;

pub use cli_color_theme::CliColorTheme;
pub use cli_format::CliFormat;
pub use cli_language::CliLanguage;
pub use cli_mermaid_renderer::CliMermaidRenderer;
pub use highlight::Highlight;

use collect_plugins::parse_plugin_occurrence;
use parse_generation_count::parse_generation_count;

#[derive(Parser, Debug, Serialize)]
#[command(
    name = "unsnarl",
    bin_name = "uns",
    version,
    about = "Generate visual graphs from JS/TS source",
    disable_version_flag = true
)]
#[serde(rename_all = "camelCase")]
pub struct Args {
    /// Input file
    pub file: Option<String>,

    /// Emitter format (mermaid, ir, json, markdown, stats)
    #[arg(
        short = 'f',
        long = "format",
        value_name = "id",
        value_enum,
        default_value = "mermaid"
    )]
    pub format: CliFormat,

    /// Disable pretty-printed JSON output
    #[arg(long = "no-pretty-json", action = clap::ArgAction::SetFalse)]
    pub pretty_json: bool,

    /// Layout engine for Mermaid output
    #[arg(long = "mermaid-renderer", value_name = "renderer", value_enum)]
    pub mermaid_renderer: Option<CliMermaidRenderer>,

    /// Color theme for Mermaid output (dark, light)
    #[arg(
        long = "color-theme",
        value_name = "theme",
        value_enum,
        default_value = "dark"
    )]
    pub color_theme: CliColorTheme,

    /// Read from stdin
    #[arg(long = "stdin", action = clap::ArgAction::SetTrue)]
    pub stdin: bool,

    /// Language for stdin input
    #[arg(
        long = "stdin-lang",
        value_name = "lang",
        value_enum,
        default_value = "ts"
    )]
    pub stdin_lang: CliLanguage,

    /// Comma-separated root queries (repeatable)
    #[arg(
        short = 'r',
        long = "roots",
        value_name = "queries",
        action = clap::ArgAction::Append
    )]
    #[serde(skip)]
    raw_roots: Vec<String>,

    /// Parsed `-r` / `--roots` queries. The clap-facing `raw_roots`
    /// field is folded into this typed `Vec<ParsedRootQuery>` by
    /// `finalize`.
    #[arg(skip)]
    pub roots: Vec<ParsedRootQuery>,

    /// Highlight matching nodes and adjacent edges (defaults to the -r/--roots queries).
    ///
    /// Tri-state mirroring the TS commander result type
    /// `RawHighlight = false | true | readonly ParsedRootQuery[]`:
    /// - `None`          : flag absent
    /// - `Some(None)`    : flag given with no value (follow `-r/--roots`)
    /// - `Some(Some(s))` : flag given with inline value (parsed by `finalize`)
    #[arg(
        short = 'H',
        long = "highlight",
        value_name = "queries",
        num_args = 0..=1,
    )]
    #[serde(skip)]
    raw_highlight: Option<Option<String>>,

    /// Parsed `-H` / `--highlight` value, mirroring the TS commander
    /// `RawHighlight = false | true | readonly ParsedRootQuery[]` form.
    /// Populated by `finalize`.
    #[arg(skip)]
    pub highlight: Highlight,

    /// Descendants generations
    #[arg(
        short = 'A',
        long = "descendants",
        value_name = "N",
        value_parser = parse_generation_count
    )]
    pub descendants: Option<u32>,

    /// Ancestors generations
    #[arg(
        short = 'B',
        long = "ancestors",
        value_name = "N",
        value_parser = parse_generation_count
    )]
    pub ancestors: Option<u32>,

    /// Context generations (-A and -B shorthand)
    #[arg(
        short = 'C',
        long = "context",
        value_name = "N",
        value_parser = parse_generation_count
    )]
    pub context: Option<u32>,

    /// Sugar: set both --depth-function and --depth-block to <N>
    #[arg(
        long = "depth",
        value_name = "N",
        value_parser = parse_generation_count
    )]
    pub depth: Option<u32>,

    /// Max function-scope nesting depth before scopes collapse to a single node
    #[arg(
        long = "depth-function",
        value_name = "N",
        value_parser = parse_generation_count
    )]
    pub depth_function: Option<u32>,

    /// Max block-scope nesting depth (applies to if/for/while/switch/try-catch-finally/block) before scopes collapse to a single node
    #[arg(
        long = "depth-block",
        value_name = "N",
        value_parser = parse_generation_count
    )]
    pub depth_block: Option<u32>,

    /// Write output to <dir>/<auto-name>.<ext>
    #[arg(
        short = 'o',
        long = "out-dir",
        value_name = "dir",
        conflicts_with = "out_file"
    )]
    pub out_dir: Option<String>,

    /// Write output to <path> (full file path, no auto-naming)
    #[arg(long = "out-file", value_name = "path")]
    pub out_file: Option<String>,

    /// Basename derived from `-r` query tokens + `-A` / `-B` / `-C` (or the
    /// positional input file when no roots are given). Populated by
    /// `finalize` when `-o` / `--out-dir` is set; otherwise `None`. Mirrors
    /// the TS `deriveOutputBasename` result that resolveOutputPath would
    /// pass to the emitter for the `<dir>/<basename>.<ext>` filename.
    #[arg(skip)]
    pub derived_basename: Option<String>,

    /// Annotate Mermaid labels with the underlying NODE_KIND / SUBGRAPH_KIND
    #[arg(long = "debug", action = clap::ArgAction::SetTrue)]
    pub debug: bool,

    /// Enable plugin(s). Repeat the flag or comma-delimit for multiple. The 'unsnarl-plugin-' prefix may be omitted.
    ///
    /// Stored per-occurrence as the validated list returned by
    /// `collect_plugins`. `finalize` folds the per-occurrence lists into
    /// the public `plugins` field.
    #[arg(
        long = "plugin",
        value_name = "names",
        action = clap::ArgAction::Append,
        value_parser = parse_plugin_occurrence,
    )]
    #[serde(skip)]
    plugin_occurrences: Vec<Vec<String>>,

    /// Flattened-and-deduped plugin list, mirroring the output of TS
    /// commander's `collectPlugins` accumulator. Populated by `finalize`
    /// after clap parsing.
    #[arg(skip)]
    pub plugins: Vec<String>,

    /// Show version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    #[serde(skip)]
    pub version: (),
}

impl Args {
    // Inherent `parse` / `try_parse_from` shadow the same-named methods
    // provided by the derived `clap::Parser` impl. This makes it
    // impossible to obtain an `Args` without running `finalize`: the
    // `Parser` trait methods are still reachable via fully qualified
    // syntax (`<Args as clap::Parser>::parse()`), but a plain
    // `Args::parse()` call resolves to the inherent method.
    pub fn try_parse_from<I, T>(itr: I) -> Result<Self, clap::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let mut args = <Self as clap::Parser>::try_parse_from(itr)?;
        args.finalize()?;
        Ok(args)
    }

    pub fn parse() -> Self {
        Self::try_parse_from(std::env::args_os()).unwrap_or_else(|e| e.exit())
    }

    fn finalize(&mut self) -> Result<(), clap::Error> {
        let raw_plugins = std::mem::take(&mut self.plugin_occurrences);
        for occurrence in raw_plugins {
            for name in occurrence {
                if !self.plugins.contains(&name) {
                    self.plugins.push(name);
                }
            }
        }
        let raw_roots = std::mem::take(&mut self.raw_roots);
        for raw in raw_roots {
            match parse_root_queries(&raw) {
                Ok(qs) => self.roots.extend(qs),
                Err(msg) => return Err(value_validation_error(msg)),
            }
        }
        let raw_highlight = std::mem::take(&mut self.raw_highlight);
        self.highlight = match raw_highlight {
            None => Highlight::Absent,
            Some(None) => Highlight::NoValue,
            Some(Some(raw)) => match parse_root_queries(&raw) {
                Ok(qs) => Highlight::Value(qs),
                Err(msg) => return Err(value_validation_error(msg)),
            },
        };
        if self.out_dir.is_some() {
            // `-o` writes to `<dir>/<auto-basename>.<ext>`, so it needs
            // either a positional file (basename comes from it) or at
            // least one `-r` query (basename comes from the root tokens).
            // `--stdin` removes the positional path, leaving `-r` as the
            // only remaining basename source.
            if self.stdin && self.roots.is_empty() {
                return Err(value_validation_error(
                    "--out-dir requires either -r/--roots or an input file path".to_string(),
                ));
            }
            let input_path = if self.stdin {
                ""
            } else {
                self.file.as_deref().unwrap_or("")
            };
            self.derived_basename = Some(derive_output_basename(
                &self.roots,
                self.descendants,
                self.ancestors,
                self.context,
                input_path,
            ));
        }
        Ok(())
    }
}

fn value_validation_error(message: String) -> clap::Error {
    clap::Error::raw(
        clap::error::ErrorKind::ValueValidation,
        format!("{message}\n"),
    )
}

#[cfg(test)]
#[path = "args_test.rs"]
mod args_test;
