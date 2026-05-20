//! Pipeline / CLI carrier for `-H` (no value) vs `-H <queries>` modes.
//!
//! Mirrors `ts/src/pipeline/highlight/highlight-run-options.ts`. The TS
//! type lives under `pipeline/highlight/`, but in the Rust workspace
//! the emitter crates (mermaid, markdown) need to reach it through
//! [`crate::EmitOptions`] equivalents, so the canonical home is here
//! alongside the other small data carriers
//! (`VisualGraphPruning`, `RootQueryResolution`). `unsnarl::pipeline::highlight`
//! re-exports it so the TS-path-driven imports remain spelled the same
//! way at the pipeline boundary.
//!
//! `Roots` reuses the queries from the `pruning.roots` set (so
//! `-H` alone follows whatever `-r` selects). `Queries` carries its
//! own query list, which is what `-H <value>` produces.

use unsnarl_root_query::ParsedRootQuery;

#[derive(Clone)]
pub enum HighlightRunOptions {
    Roots,
    Queries(Vec<ParsedRootQuery>),
}
