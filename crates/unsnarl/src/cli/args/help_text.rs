//! `--help` / `-h` output for the `uns` CLI.
//!
//! The clap derive used to assemble this from per-field doc comments
//! and `#[arg(...)]` attributes at runtime. Owning it as a static
//! string here removes the clap-builder code from the binary (~150KB
//! of `__text`) at the cost of having to hand-edit this constant
//! whenever a flag is added, removed, or renamed.
//!
//! Editing checklist: every change to [`super::Args`] field set,
//! short / long names, accepted enum values, or default values must
//! land here too, and the matching `args_test.rs` cases should be
//! verified.

/// Plain ASCII so the output is grep-friendly and renders the same in
/// every terminal. Each option entry uses the same two-line shape:
/// "  -x, --long <value>  description" / continuation lines indented
/// to align with the description column. Final newline is included so
/// callers can `print!`/`write!` it as-is.
pub const HELP_TEXT: &str = "\
Generate visual graphs from JavaScript / TypeScript source

Usage: uns [OPTIONS] [FILE]

Arguments:
  [FILE]  Input file

Options:
  -f, --format <id>              Emitter format
                                 [default: mermaid]
                                 [possible values: mermaid, ir, json, markdown, stats]
      --no-pretty-json           Disable pretty-printed JSON output
      --mermaid-renderer <renderer>
                                 Layout engine for Mermaid output
                                 [possible values: dagre, elk]
      --color-theme <theme>      Color theme for Mermaid output
                                 [default: dark]
                                 [possible values: dark, light]
      --stdin                    Read from stdin
      --stdin-lang <lang>        Language for stdin input
                                 [default: ts]
                                 [possible values: ts, tsx, js, jsx]
  -r, --roots <queries>          Comma-separated root queries (repeatable)
  -H, --highlight [<queries>]    Highlight matching nodes and adjacent edges
                                 (defaults to the -r/--roots queries)
  -A, --descendants <N>          Descendants generations
  -B, --ancestors <N>            Ancestors generations
  -C, --context <N>              Context generations (-A and -B shorthand)
      --depth <N>                Sugar: set both --depth-function and --depth-block to <N>
      --depth-function <N>       Max function-scope nesting depth before scopes
                                 collapse to a single node
      --depth-block <N>          Max block-scope nesting depth (if/for/while/switch/
                                 try-catch-finally/block) before scopes collapse
  -o, --out-dir <dir>            Write output to <dir>/<auto-name>.<ext>
      --out-file <path>          Write output to <path> (full file path, no auto-naming)
      --debug                    Annotate Mermaid labels with NODE_KIND / SUBGRAPH_KIND
      --verbose                  Stream diagnostic and timing logs to stderr
      --plugin <names>           Enable plugin(s). Repeat the flag or comma-delimit
                                 for multiple. The 'unsnarl-plugin-' prefix may be omitted.
  -v, --version                  Show version
  -h, --help                     Print help
";

pub fn version_text() -> String {
    format!("unsnarl {}\n", env!("CARGO_PKG_VERSION"))
}
