//! `unsnarl-visual-graph` builder family.
//!
//! Mirrors `ts/src/visual-graph/builder/`. The leaf modules host
//! single pure functions; composite modules (`build_scope`,
//! `build_children`, `build_visual_graph`) drive them. The entry
//! point [`build_visual_graph::build_visual_graph`] lands once the
//! supporting leaves and composites are populated.

pub mod branch_container_key;
pub mod edge_label_of_ref;
pub mod expression_statement_node_id;
pub mod find_node_by_id;
pub mod if_container_subgraph_id;
pub mod if_test_node_id;
pub mod intermediate_key;
pub mod is_ancestor_scope;
pub mod is_class_subgraph;
pub mod is_collapsed;
pub mod is_function_subgraph;
pub mod line_for_offset;
pub mod loop_test_node_id;
pub mod module_root_id;
pub mod nesting_kind_of;
pub mod node_id;
pub mod ret_use_node_id;
pub mod return_subgraph_id;
pub mod sanitize;
pub mod subgraph_scope_id;
pub mod throw_subgraph_id;
pub mod throw_use_node_id;
pub mod write_op_node_id;
