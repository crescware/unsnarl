//! Layout engine selection for `--mermaid-renderer`.

use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, ValueEnum, Serialize)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "lowercase")]
pub enum CliMermaidRenderer {
    Dagre,
    Elk,
}
