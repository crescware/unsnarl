//! Stdin language selection for `--stdin-lang`.

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CliLanguage {
    Ts,
    Tsx,
    Js,
    Jsx,
}

impl CliLanguage {
    pub const ACCEPTED: &'static [&'static str] = &["ts", "tsx", "js", "jsx"];

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "ts" => Some(Self::Ts),
            "tsx" => Some(Self::Tsx),
            "js" => Some(Self::Js),
            "jsx" => Some(Self::Jsx),
            _ => None,
        }
    }
}
