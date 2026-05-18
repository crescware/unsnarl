//! Layout engine selection for `--mermaid-renderer`.
//!
//! Mirrors `CLI_MERMAID_RENDERER` in `ts/src/cli-mermaid-renderer.ts`
//! (validated via `MERMAID_RENDERERS` in
//! `ts/src/cli/args/cli-mermaid-renderer.ts`).

use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, ValueEnum, Serialize)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "lowercase")]
pub enum CliMermaidRenderer {
    Dagre,
    Elk,
}
