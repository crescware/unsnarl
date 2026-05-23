//! Layout engine selection for `--mermaid-renderer`.

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CliMermaidRenderer {
    Dagre,
    Elk,
}

impl CliMermaidRenderer {
    pub const ACCEPTED: &'static [&'static str] = &["dagre", "elk"];

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "dagre" => Some(Self::Dagre),
            "elk" => Some(Self::Elk),
            _ => None,
        }
    }
}
