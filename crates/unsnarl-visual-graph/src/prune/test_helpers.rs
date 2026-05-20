//! Shared `VisualNode` / `VisualSubgraph` / `VisualGraph` builders
//! used by the sibling prune tests.

use unsnarl_ir::language::Language;
use unsnarl_ir::serialized::serialized_ir::SERIALIZED_IR_VERSION;

use crate::direction::Direction;
use crate::visual_boundary_edge::VisualBoundaryEdge;
use crate::visual_edge::VisualEdge;
use crate::visual_element::VisualElement;
use crate::visual_element_type::{NodeTypeTag, SubgraphTypeTag};
use crate::visual_graph::{VisualGraph, VisualGraphSource};
use crate::visual_node::{
    BindingExtras, BindingNodeKind, BindingVisualNode, SyntheticExtras, SyntheticNodeKind,
    SyntheticVisualNode, VisualNode,
};
use crate::visual_subgraph::{OwnedExtras, OwnedSubgraphKind, OwnedVisualSubgraph, VisualSubgraph};

pub fn const_binding_node(id: &str, name: &str, line: u32) -> VisualNode {
    VisualNode::Binding(BindingVisualNode {
        r#type: NodeTypeTag::Node,
        id: id.to_string(),
        name: name.to_string(),
        line,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        kind: BindingNodeKind::ConstBinding,
        extras: BindingExtras::Variable {
            init_is_function: false,
        },
    })
}

pub fn const_binding_node_with_end(
    id: &str,
    name: &str,
    line: u32,
    end_line: Option<u32>,
) -> VisualNode {
    VisualNode::Binding(BindingVisualNode {
        r#type: NodeTypeTag::Node,
        id: id.to_string(),
        name: name.to_string(),
        line,
        end_line,
        is_jsx_element: false,
        unused: false,
        kind: BindingNodeKind::ConstBinding,
        extras: BindingExtras::Variable {
            init_is_function: false,
        },
    })
}

pub fn write_op_node(id: &str, name: &str, line: u32) -> VisualNode {
    VisualNode::Synthetic(SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id: id.to_string(),
        kind: SyntheticNodeKind::WriteReference,
        name: name.to_string(),
        line,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::WriteOp {
            declaration_kind: None,
        },
    })
}

pub fn return_use_node(id: &str, name: &str, line: u32, end_line: Option<u32>) -> VisualNode {
    VisualNode::Synthetic(SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id: id.to_string(),
        kind: SyntheticNodeKind::ReturnArgumentReference,
        name: name.to_string(),
        line,
        end_line,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::None {},
    })
}

pub fn function_subgraph(id: &str, line: u32, elements: Vec<VisualElement>) -> VisualSubgraph {
    VisualSubgraph::Owned(OwnedVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: id.to_string(),
        kind: OwnedSubgraphKind::Function,
        line,
        end_line: None,
        direction: Direction::TB,
        extras: OwnedExtras::Function {
            owner_node_id: Some("n_owner".to_string()),
            owner_name: "owner".to_string(),
        },
        elements,
    })
}

pub fn return_subgraph(id: &str, line: u32, elements: Vec<VisualElement>) -> VisualSubgraph {
    VisualSubgraph::Owned(OwnedVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: id.to_string(),
        kind: OwnedSubgraphKind::Return,
        line,
        end_line: None,
        direction: Direction::RL,
        extras: OwnedExtras::None {},
        elements,
    })
}

pub fn if_else_container_subgraph(
    id: &str,
    line: u32,
    elements: Vec<VisualElement>,
) -> VisualSubgraph {
    VisualSubgraph::Owned(OwnedVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: id.to_string(),
        kind: OwnedSubgraphKind::IfElseContainer,
        line,
        end_line: None,
        direction: Direction::RL,
        extras: OwnedExtras::IfElseContainer { has_else: false },
        elements,
    })
}

pub fn graph_of(elements: Vec<VisualElement>, edges: Vec<VisualEdge>) -> VisualGraph {
    VisualGraph {
        version: SERIALIZED_IR_VERSION,
        source: VisualGraphSource {
            path: "x.ts".to_string(),
            language: Language::Ts,
        },
        direction: Direction::RL,
        elements,
        edges,
        boundary_edges: Vec::<VisualBoundaryEdge>::new(),
        pruning: None,
    }
}
