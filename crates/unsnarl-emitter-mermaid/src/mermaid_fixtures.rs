//! Test-only fixture helpers shared across the mermaid emitter's
//! sibling `_test.rs` files.
//!
//! Helpers are grouped into a single module rather than one file
//! per helper because they are consumed only by sibling unit tests
//! and the inline grouping keeps the file count low.
//!
//! Each `base_*` constructor returns the concrete struct (rather than
//! the wrapping enum) so callers can pin individual fields with
//! struct-update syntax and then wrap with the appropriate variant.

use std::collections::{HashMap, HashSet};

use unsnarl_ir::language::Language;
use unsnarl_visual_graph::direction::Direction;
use unsnarl_visual_graph::visual_graph::VisualGraph;
use unsnarl_visual_graph::visual_node::{
    BindingNodeKind, BindingVisualNode, SyntheticNodeKind, SyntheticVisualNode,
};
use unsnarl_visual_graph::visual_subgraph::{
    ControlSubgraphKind, ControlVisualSubgraph, OwnedVisualSubgraph,
};

use crate::render_state::RenderState;
use crate::strategy::MermaidStrategy;
use crate::theme::DARK_THEME;

// ---- Nodes ----------------------------------------------------------------

pub fn base_const_binding() -> BindingVisualNode {
    BindingVisualNode::const_binding("n_v", "x", 1)
}

pub fn base_var_binding() -> BindingVisualNode {
    BindingVisualNode::var_binding("n_v", "x", 1)
}

pub fn base_let_binding() -> BindingVisualNode {
    BindingVisualNode::let_binding("n_v", "x", 1)
}

pub fn base_using_binding() -> BindingVisualNode {
    BindingVisualNode::using_binding("n_v", "x", 1)
}

pub fn base_await_using_binding() -> BindingVisualNode {
    BindingVisualNode::await_using_binding("n_v", "x", 1)
}

pub fn base_write_op() -> SyntheticVisualNode {
    SyntheticVisualNode::write_reference("n_v", "x", 1)
}

pub fn base_simple_synthetic(kind: SyntheticNodeKind) -> SyntheticVisualNode {
    SyntheticVisualNode {
        kind,
        ..SyntheticVisualNode::return_argument_reference("n_v", "x", 1)
    }
}

pub fn base_simple_binding(kind: BindingNodeKind) -> BindingVisualNode {
    BindingVisualNode {
        kind,
        ..BindingVisualNode::formal_parameter("n_v", "x", 1)
    }
}

pub fn base_import_binding_named(imported_name: &str) -> BindingVisualNode {
    BindingVisualNode::named_import_binding("n_v", "x", imported_name, 1)
}

pub fn base_import_binding_default() -> BindingVisualNode {
    BindingVisualNode::default_import_binding("n_v", "x", 1)
}

pub fn base_import_binding_namespace() -> BindingVisualNode {
    BindingVisualNode::namespace_import_binding("n_v", "x", 1)
}

// ---- Subgraphs ------------------------------------------------------------

pub fn base_function_subgraph() -> OwnedVisualSubgraph {
    OwnedVisualSubgraph::function(
        "s_x",
        1,
        Some("n_owner".to_string()),
        "owner",
        Vec::new(),
        Direction::RL,
    )
}

pub fn base_case_subgraph() -> ControlVisualSubgraph {
    ControlVisualSubgraph::case("s_x", 1, None, Vec::new(), Direction::RL)
}

pub fn base_class_subgraph() -> OwnedVisualSubgraph {
    OwnedVisualSubgraph::class("s_x", 1, None, Vec::new(), Direction::RL)
}

pub fn base_if_else_container_subgraph() -> OwnedVisualSubgraph {
    OwnedVisualSubgraph::if_else_container("s_x", 1, false, Vec::new(), Direction::RL)
}

pub fn base_plain_subgraph(kind: ControlSubgraphKind) -> ControlVisualSubgraph {
    ControlVisualSubgraph {
        kind,
        ..ControlVisualSubgraph::block("s_x", 1, Vec::new(), Direction::RL)
    }
}

// ---- Edges / Graphs ------------------------------------------------------

pub fn base_graph() -> VisualGraph {
    VisualGraph::new(
        "input.ts",
        Language::Ts,
        Direction::RL,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
}

// ---- RenderState ---------------------------------------------------------

/// Dark theme, dagre strategy, every collection empty.
///
/// `'a` stays free here so the caller can populate `node_map` with
/// borrows that live in the test scope; the empty `HashMap` does not
/// pin the lifetime.
pub fn base_render_state<'a>() -> RenderState<'a> {
    RenderState {
        lines: Vec::new(),
        node_map: HashMap::new(),
        wrapped_owner_ids: HashSet::new(),
        placeholder_ids: Vec::new(),
        nest_class_map: HashMap::new(),
        strategy: MermaidStrategy::Dagre,
        theme: &DARK_THEME,
        debug: false,
    }
}
