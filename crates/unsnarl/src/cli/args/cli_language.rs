//! Stdin language selection for `--stdin-lang`.

use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, ValueEnum, Serialize)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "lowercase")]
pub enum CliLanguage {
    Ts,
    Tsx,
    Js,
    Jsx,
}
