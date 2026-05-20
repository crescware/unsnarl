//! `unsnarl-visual-graph` builder family.
//!
//! Mirrors `ts/src/visual-graph/builder/`. The leaf modules host
//! single pure functions; composite modules (`build_scope`,
//! `build_children`, `build_visual_graph`) drive them. The entry
//! point [`build_visual_graph::build_visual_graph`] lands once the
//! supporting leaves and composites are populated.

pub mod expression_statement_node_id;
pub mod if_container_subgraph_id;
pub mod if_test_node_id;
pub mod intermediate_key;
pub mod line_for_offset;
pub mod loop_test_node_id;
pub mod module_root_id;
pub mod node_id;
pub mod ret_use_node_id;
pub mod return_subgraph_id;
pub mod sanitize;
pub mod subgraph_scope_id;
pub mod throw_subgraph_id;
pub mod throw_use_node_id;
pub mod write_op_node_id;
