//! Pipeline / CLI carrier for `-H` (no value) vs `-H <queries>` modes.
//!
//! The canonical home is here alongside the other small data
//! carriers (`VisualGraphPruning`, `RootQueryResolution`) so the
//! emitter crates (mermaid, markdown) can reach it through
//! [`crate::EmitOptions`] equivalents.
//! `unsnarl::pipeline::highlight` re-exports it for callers that
//! want a pipeline-side import path.
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
