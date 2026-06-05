//! Highlight reuses the `-r/--roots` query grammar to flag a subset of
//! nodes for renderer-specific paint.
//!
//! Also hosts the `HighlightRunOptions` data carrier here so the
//! emitter crates can reach it through [`crate::EmitOptions`]
//! equivalents.
//!
//! Public entry points:
//!
//! - [`collect_highlight_path_ids::collect_highlight_path_ids`] — given
//!   a [`crate::VisualGraph`] and a list of `-H` queries, return the ids
//!   to highlight. Handles the point / path (`a..b`) / direction
//!   (`a..+a` / `+b` / `+c`) grammar (issue #90): it resolves each
//!   endpoint's ambiguity, computes reachability over the drawn graph's
//!   edges, and reports which ids are point hits (vs reachability hits).
//! - [`node_matches_highlight_query::node_matches_highlight_query`] —
//!   the per-node matcher. Diverges from `prune::node_matches_query`
//!   on purpose by dropping the `NAME_QUERY_EXCLUDED` filter so bare
//!   name queries paint every appearance of the identifier.
//! - [`highlight_run_options::HighlightRunOptions`] — the pipeline /
//!   CLI carrier for `-H` (no value) vs `-H <queries>` modes.

pub mod collect_highlight_path_ids;
pub mod highlight_run_options;
pub mod node_matches_highlight_query;

pub use collect_highlight_path_ids::{
    collect_highlight_path_ids, HighlightSelection, HighlightWarning,
};
pub use highlight_run_options::HighlightRunOptions;
pub use node_matches_highlight_query::node_matches_highlight_query;
