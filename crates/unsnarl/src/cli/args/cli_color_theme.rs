//! Color theme selection for `--color-theme`.

use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, ValueEnum, Serialize)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "lowercase")]
pub enum CliColorTheme {
    Dark,
    Light,
}
