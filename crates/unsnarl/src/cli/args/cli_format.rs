//! Emitter format selection for `-f` / `--format`.
//!
//! The accepted values are the fixed set the emitter registry
//! produces. The lowercase forms below are the spelling exposed on
//! the CLI surface and serialised through serde for TS parity.

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CliFormat {
    Mermaid,
    Ir,
    Json,
    Markdown,
    Stats,
}

impl CliFormat {
    pub const ACCEPTED: &'static [&'static str] = &["mermaid", "ir", "json", "markdown", "stats"];

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "mermaid" => Some(Self::Mermaid),
            "ir" => Some(Self::Ir),
            "json" => Some(Self::Json),
            "markdown" => Some(Self::Markdown),
            "stats" => Some(Self::Stats),
            _ => None,
        }
    }
}
