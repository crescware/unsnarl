//! Emitter format selection for `-f` / `--format`.
//!
//! TS truth lives in the emitter `format` constants under
//! `ts/src/emitter/{ir,json,mermaid,markdown,stats}/*.ts` and is composed
//! at runtime via `ts/src/pipeline/create-default-emitter-registry.ts`.
//! The Rust emitter registry is filled in over Steps 12-16; until then
//! the accepted values are the fixed set the TS registry produces.

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
