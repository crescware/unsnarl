//! Color theme selection for `--color-theme`.
//!
//! Mirrors `CLI_COLOR_THEME` in `ts/src/cli-color-theme.ts` (validated
//! via `COLOR_THEMES` in `ts/src/cli/args/cli-color-theme.ts`).

use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, ValueEnum, Serialize)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "lowercase")]
pub enum CliColorTheme {
    Dark,
    Light,
}
