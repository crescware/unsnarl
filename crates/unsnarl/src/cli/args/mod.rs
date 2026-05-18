//! `uns` CLI argument definitions. Mirrors the TS commander source at
//! `ts/src/cli/args/build-command.ts` and the per-option files alongside
//! it.

use clap::Parser;
use serde::{Serialize, Serializer};

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
        default_value = "mermaid"
    )]
    pub format: String,

    /// Disable pretty-printed JSON output
    #[arg(long = "no-pretty-json", action = clap::ArgAction::SetFalse)]
    pub pretty_json: bool,

    /// Layout engine for Mermaid output
    #[arg(long = "mermaid-renderer", value_name = "renderer")]
    pub mermaid_renderer: Option<String>,

    /// Color theme for Mermaid output (dark, light)
    #[arg(long = "color-theme", value_name = "theme", default_value = "dark")]
    pub color_theme: String,

    /// Read from stdin
    #[arg(long = "stdin", action = clap::ArgAction::SetTrue)]
    pub stdin: bool,

    /// Language for stdin input
    #[arg(long = "stdin-lang", value_name = "lang", default_value = "ts")]
    pub stdin_lang: String,

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
    #[arg(short = 'A', long = "descendants", value_name = "N")]
    pub descendants: Option<String>,

    /// Ancestors generations
    #[arg(short = 'B', long = "ancestors", value_name = "N")]
    pub ancestors: Option<String>,

    /// Context generations (-A and -B shorthand)
    #[arg(short = 'C', long = "context", value_name = "N")]
    pub context: Option<String>,

    /// Sugar: set both --depth-function and --depth-block to <N>
    #[arg(long = "depth", value_name = "N")]
    pub depth: Option<String>,

    /// Max function-scope nesting depth before scopes collapse to a single node
    #[arg(long = "depth-function", value_name = "N")]
    pub depth_function: Option<String>,

    /// Max block-scope nesting depth (applies to if/for/while/switch/try-catch-finally/block) before scopes collapse to a single node
    #[arg(long = "depth-block", value_name = "N")]
    pub depth_block: Option<String>,

    /// Write output to <dir>/<auto-name>.<ext>
    #[arg(short = 'o', long = "out-dir", value_name = "dir")]
    pub out_dir: Option<String>,

    /// Write output to <path> (full file path, no auto-naming)
    #[arg(long = "out-file", value_name = "path")]
    pub out_file: Option<String>,

    /// Annotate Mermaid labels with the underlying NODE_KIND / SUBGRAPH_KIND
    #[arg(long = "debug", action = clap::ArgAction::SetTrue)]
    pub debug: bool,

    /// Enable plugin(s). Repeat the flag or comma-delimit for multiple. The 'unsnarl-plugin-' prefix may be omitted.
    #[arg(
        long = "plugin",
        value_name = "names",
        action = clap::ArgAction::Append
    )]
    pub plugins: Vec<String>,

    /// Show version
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    #[serde(skip)]
    pub version: (),
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
