//! `unsnarl-visual-graph` builder family.
//!
//! The leaf modules host single pure functions; composite modules
//! (`build_scope`, `build_children`, `build_visual_graph`) drive
//! them. The entry point is
//! [`build_visual_graph::build_visual_graph`].

pub mod arena;
pub mod branch_container_key;
pub mod branch_merged_origins;
pub mod branch_scope_of;
pub mod build_children;
pub mod build_scope;
pub mod build_visual_graph;
pub mod context;
pub mod control_subgraph_kind_of;
pub mod describe_subgraph;
pub mod edge_label_of_ref;
pub mod enclosing_function_var;
pub mod ensure_beyond_depth_stub;
pub mod ensure_expression_statement_node;
pub mod ensure_return_use_node;
pub mod ensure_throw_use_node;
pub mod expression_statement_node_id;
pub mod find_enclosing_subgraph_scope;
pub mod find_host_subgraph;
pub mod find_node_by_id;
pub mod if_container_subgraph_id;
pub mod if_test_node_id;
pub mod intermediate_key;
pub mod is_ancestor_scope;
pub mod is_branch_scope;
pub mod is_class_subgraph;
pub mod is_collapsed;
pub mod is_control_subgraph;
pub mod is_function_subgraph;
pub mod last_write_op_in_scope_before;
pub mod line_for_offset;
pub mod loop_test_anchor;
pub mod loop_test_node_id;
pub mod make_variable_node;
pub mod module_root_id;
pub mod nesting_kind_of;
pub mod node_id;
pub mod outermost_branch_under;
pub mod owner_target_id;
pub mod predicate_target_id;
pub mod previous_fallthrough_case;
pub mod push_edge;
pub mod read_origins;
pub mod render_head_expression;
pub mod resolve_read_target_id;
pub mod ret_use_node_id;
pub mod return_subgraph_id;
pub mod sanitize;
pub mod set_predecessor_of;
pub mod should_subgraph;
pub mod state;
pub mod state_at;
pub mod state_ref_id;
pub mod subgraph_scope_id;
pub mod switch_discriminant_anchor;
pub mod throw_subgraph_id;
pub mod throw_use_node_id;
pub mod timing;
pub mod visible_ancestor_subgraph;
pub mod write_op;
pub mod write_op_node_id;

#[cfg(test)]
pub(crate) mod testing;
