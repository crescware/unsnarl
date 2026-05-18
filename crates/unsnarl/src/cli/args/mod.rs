//! `uns` CLI argument definitions. Mirrors the TS commander source at
//! `ts/src/cli/args/build-command.ts` and the per-option files alongside
//! it.

use clap::Parser;
use serde::{Serialize, Serializer};

pub mod cli_color_theme;
pub mod cli_format;
pub mod cli_language;
pub mod cli_mermaid_renderer;
pub mod collect_plugins;
pub mod parse_generation_count;

pub use cli_color_theme::CliColorTheme;
pub use cli_format::CliFormat;
pub use cli_language::CliLanguage;
pub use cli_mermaid_renderer::CliMermaidRenderer;

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
    pub roots: Vec<String>,

    /// Highlight matching nodes and adjacent edges (defaults to the -r/--roots queries).
    ///
    /// Tri-state mirroring the TS commander result type
    /// `RawHighlight = false | true | string`:
    /// - `None`          : flag absent
    /// - `Some(None)`    : flag given with no value (follow `-r/--roots`)
    /// - `Some(Some(s))` : flag given with inline value
    #[arg(
        short = 'H',
        long = "highlight",
        value_name = "queries",
        num_args = 0..=1,
    )]
    #[serde(serialize_with = "serialize_highlight")]
    pub highlight: Option<Option<String>>,

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
        args.finalize();
        Ok(args)
    }

    pub fn parse() -> Self {
        Self::try_parse_from(std::env::args_os()).unwrap_or_else(|e| e.exit())
    }

    fn finalize(&mut self) {
        let raw = std::mem::take(&mut self.plugin_occurrences);
        for occurrence in raw {
            for name in occurrence {
                if !self.plugins.contains(&name) {
                    self.plugins.push(name);
                }
            }
        }
    }
}

// Mirrors TS commander's RawHighlight = false | true | string:
//   absent             -> false (None)
//   present, no value  -> true  (Some(None))
//   present with value -> string (Some(Some(...)))
fn serialize_highlight<S>(value: &Option<Option<String>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        None => s.serialize_bool(false),
        Some(None) => s.serialize_bool(true),
        Some(Some(v)) => s.serialize_str(v),
    }
}

#[cfg(test)]
mod test;
