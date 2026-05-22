//! Pruning core: turn a base [`VisualGraph`](crate::visual_graph::VisualGraph)
//! and a `Vec<ParsedRootQuery>` (plus descendants / ancestors radii)
//! into a narrowed graph.
//!
//! The module is split one function per file. Public entry points:
//!
//! - [`prune_visual_graph::prune_visual_graph`] — main orchestrator.
//! - [`resolve_ambiguous_queries::resolve_ambiguous_queries`] —
//!   `LineOrName` disambiguator that runs before pruning.
//! - [`format_resolution_notice::format_resolution_notice`] —
//!   shared three-line warning formatter used by the CLI's stderr
//!   emitter and the markdown emitter's Notice section.

pub mod bfs;
pub mod build_adjacency;
pub mod build_parent_map;
pub mod collect_ids;
pub mod collect_node_ids;
pub mod format_resolution_notice;
pub mod iterate_visual_nodes;
pub mod iterate_visual_subgraphs;
pub mod name_query_excluded;
pub mod node_matches_query;
pub mod prune_options;
pub mod prune_visual_graph;
pub mod push_to;
pub mod rebuild_elements;
pub mod resolve_ambiguous_queries;
pub mod root_candidate_kinds;
pub mod root_query_resolution;

#[cfg(test)]
mod test_helpers;

pub use format_resolution_notice::format_resolution_notice;
pub use prune_options::{PerQueryMatch, PruneOptions, PruneResult};
pub use prune_visual_graph::prune_visual_graph;
pub use resolve_ambiguous_queries::{resolve_ambiguous_queries, ResolveResult};
pub use root_query_resolution::{ResolvedAs, RootQueryResolution};
