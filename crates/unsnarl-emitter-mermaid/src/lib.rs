//! Mermaid emitter (strategy + theme).
//!
//! The strategy enum selects the renderer-specific lines (elk vs
//! dagre); the theme governs the `classDef` / `style` / `linkStyle`
//! colors. Both are picked at the CLI / pipeline boundary and handed
//! to [`MermaidEmitter`] at construction time.

pub mod collect_highlight_edge_indices;
pub mod collect_import_sources;
pub mod collect_nodes_into;
pub mod collect_wrapped_owner_ids;
pub mod emit_node;
pub mod emit_plain_subgraph;
pub mod emit_subgraph;
pub mod escape;
pub mod line_range_label;
pub mod mermaid;
pub mod node_head;
pub mod node_label;
pub mod node_syntax;
pub mod push_edge_lines;
pub mod record_nest_slot;
pub mod render_boundary_edges;
pub mod render_class_defs;
pub mod render_highlight;
pub mod render_pruning_comment;
pub mod render_state;
pub mod render_synthetic_node_block;
pub mod render_top_level_nodes;
pub mod render_top_level_subgraphs;
pub mod renders_in_synthetic_block;
pub mod split_edges;
pub mod strategy;
pub mod subgraph_label;
pub mod theme;

#[cfg(test)]
pub(crate) mod testing;

pub use mermaid::MermaidEmitter;
