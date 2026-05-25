//! Shared `VisualNode` / `VisualSubgraph` / `VisualGraph` builders
//! used by the sibling prune tests.

use unsnarl_ir::language::Language;

use crate::direction::Direction;
use crate::visual_edge::VisualEdge;
use crate::visual_element::VisualElement;
use crate::visual_graph::VisualGraph;
use crate::visual_node::{BindingVisualNode, SyntheticVisualNode, VisualNode};
use crate::visual_subgraph::{OwnedVisualSubgraph, VisualSubgraph};

pub fn const_binding_node(id: &str, name: &str, line: u32) -> VisualNode {
    BindingVisualNode::const_binding(id, name, line).into()
}

pub fn const_binding_node_with_end(
    id: &str,
    name: &str,
    line: u32,
    end_line: Option<u32>,
) -> VisualNode {
    let mut n = BindingVisualNode::const_binding(id, name, line);
    n.end_line = end_line;
    n.into()
}

pub fn write_op_node(id: &str, name: &str, line: u32) -> VisualNode {
    SyntheticVisualNode::write_reference(id, name, line).into()
}

pub fn return_use_node(id: &str, name: &str, line: u32, end_line: Option<u32>) -> VisualNode {
    let mut n = SyntheticVisualNode::return_argument_reference(id, name, line);
    n.end_line = end_line;
    n.into()
}

pub fn function_subgraph(id: &str, line: u32, elements: Vec<VisualElement>) -> VisualSubgraph {
    OwnedVisualSubgraph::function(
        id,
        line,
        Some("n_owner".to_string()),
        "owner",
        elements,
        Direction::TB,
    )
    .into()
}

pub fn return_subgraph(id: &str, line: u32, elements: Vec<VisualElement>) -> VisualSubgraph {
    OwnedVisualSubgraph::return_subgraph(id, line, elements, Direction::RL).into()
}

pub fn if_else_container_subgraph(
    id: &str,
    line: u32,
    elements: Vec<VisualElement>,
) -> VisualSubgraph {
    OwnedVisualSubgraph::if_else_container(id, line, false, elements, Direction::RL).into()
}

pub fn graph_of(elements: Vec<VisualElement>, edges: Vec<VisualEdge>) -> VisualGraph {
    VisualGraph::new(
        "x.ts",
        Language::Ts,
        Direction::RL,
        elements,
        edges,
        Vec::new(),
    )
}
