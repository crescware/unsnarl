//! Highlight reuses the `-r/--roots` query grammar to flag a subset of
//! nodes for renderer-specific paint.
//!
//! Mirrors `ts/src/visual-graph/highlight/` plus the
//! `pipeline/highlight/highlight-run-options.ts` data carrier (hosted
//! here because the emitter crates need to reach it through
//! [`crate::EmitOptions`] equivalents). Public entry points:
//!
//! - [`collect_highlight_ids::collect_highlight_ids`] — given a
//!   [`crate::VisualGraph`] and a list of already-resolved queries,
//!   return the ids of every visible node that satisfies at least one
//!   query under the highlight-specific matcher.
//! - [`node_matches_highlight_query::node_matches_highlight_query`] —
//!   the per-node matcher. Diverges from `prune::node_matches_query`
//!   on purpose by dropping the `NAME_QUERY_EXCLUDED` filter so bare
//!   name queries paint every appearance of the identifier.
//! - [`highlight_run_options::HighlightRunOptions`] — the pipeline /
//!   CLI carrier for `-H` (no value) vs `-H <queries>` modes.

pub mod collect_highlight_ids;
pub mod highlight_run_options;
pub mod node_matches_highlight_query;

pub use collect_highlight_ids::collect_highlight_ids;
pub use highlight_run_options::HighlightRunOptions;
pub use node_matches_highlight_query::node_matches_highlight_query;
