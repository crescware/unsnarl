//! Stdin language selection for `--stdin-lang`.
//!
//! Mirrors `LANGUAGE` in `ts/src/language.ts` (validated via
//! `LANGUAGES` in `ts/src/cli/args/cli-language.ts`).

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
