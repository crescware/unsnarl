//! Emitter format selection for `-f` / `--format`.
//!
//! The accepted values are the fixed set the emitter registry
//! produces.

use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, ValueEnum, Serialize)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "lowercase")]
pub enum CliFormat {
    Mermaid,
    Ir,
    Json,
    Markdown,
    Stats,
}
