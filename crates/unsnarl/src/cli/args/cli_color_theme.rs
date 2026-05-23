//! Color theme selection for `--color-theme`.

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CliColorTheme {
    Dark,
    Light,
}

impl CliColorTheme {
    pub const ACCEPTED: &'static [&'static str] = &["dark", "light"];

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "dark" => Some(Self::Dark),
            "light" => Some(Self::Light),
            _ => None,
        }
    }
}
