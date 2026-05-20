use super::*;

use unsnarl_ir::language::Language;
use unsnarl_ir::serialized::serialized_ir::SERIALIZED_IR_VERSION;
use unsnarl_ir::SourceLine;

use crate::direction::Direction;
use crate::visual_boundary_edge::VisualBoundaryEdge;
use crate::visual_edge::VisualEdge;
use crate::visual_element::VisualElement;
use crate::visual_element_type::NodeTypeTag;
use crate::visual_graph::{VisualGraph, VisualGraphSource};
use crate::visual_node::{
    BindingExtras, BindingNodeKind, BindingVisualNode, SyntheticExtras, SyntheticNodeKind,
    SyntheticVisualNode, VisualNode,
};

fn variable_node(id: &str, name: &str, line: u32) -> VisualNode {
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

fn return_use_node(id: &str, name: &str, line: u32) -> VisualNode {
    VisualNode::Synthetic(SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id: id.to_string(),
        kind: SyntheticNodeKind::ReturnArgumentReference,
        name: name.to_string(),
        line,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::None {},
    })
}

fn write_op_node(id: &str, name: &str, line: u32) -> VisualNode {
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

fn graph_of(nodes: Vec<VisualNode>) -> VisualGraph {
    VisualGraph {
        version: SERIALIZED_IR_VERSION,
        source: VisualGraphSource {
            path: "x.ts".to_string(),
            language: Language::Ts,
        },
        direction: Direction::RL,
        elements: nodes.into_iter().map(VisualElement::Node).collect(),
        edges: Vec::<VisualEdge>::new(),
        boundary_edges: Vec::<VisualBoundaryEdge>::new(),
        pruning: None,
    }
}

#[test]
fn returns_empty_when_no_queries_are_supplied() {
    let g = graph_of(vec![variable_node("n_a", "a", 1)]);
    assert!(collect_highlight_ids(&g, &[]).is_empty());
}

#[test]
fn name_query_collects_every_node_carrying_that_source_name() {
    let g = graph_of(vec![
        variable_node("n_a_decl", "a", 1),
        variable_node("n_a_use", "a", 2),
        variable_node("n_b", "b", 3),
    ]);
    let ids = collect_highlight_ids(
        &g,
        &[ParsedRootQuery::Name {
            name: "a".to_string(),
            raw: "a".to_string(),
        }],
    );
    // Walk order: n_a_decl is visited first, then n_a_use.
    assert_eq!(ids, vec!["n_a_decl", "n_a_use"]);
}

#[test]
fn line_query_collects_every_node_on_that_line() {
    let g = graph_of(vec![
        variable_node("n_a", "a", 1),
        variable_node("n_b", "b", 2),
        variable_node("n_c", "c", 2),
    ]);
    let ids = collect_highlight_ids(
        &g,
        &[ParsedRootQuery::Line {
            line: SourceLine(2),
            raw: "2".to_string(),
        }],
    );
    assert_eq!(ids, vec!["n_b", "n_c"]);
}

#[test]
fn query_that_matches_nothing_yields_empty_set() {
    let g = graph_of(vec![variable_node("n_a", "a", 1)]);
    let ids = collect_highlight_ids(
        &g,
        &[ParsedRootQuery::Name {
            name: "nope".to_string(),
            raw: "nope".to_string(),
        }],
    );
    assert!(ids.is_empty());
}

// Highlight diverges from `-r/--roots` here on purpose: pruning's
// `node_matches_query` would skip `WriteOp` / `ReturnUse` on a bare
// name query, but for highlight the user benefit is "paint every
// place this identifier appears". This test pins the divergence.
#[test]
fn name_query_matches_write_op_and_return_use_unlike_minus_r() {
    let g = graph_of(vec![
        variable_node("n_decl", "counter", 1),
        write_op_node("n_write", "counter", 2),
        return_use_node("n_return", "counter", 3),
        variable_node("n_other", "other", 4),
    ]);
    let ids = collect_highlight_ids(
        &g,
        &[ParsedRootQuery::Name {
            name: "counter".to_string(),
            raw: "counter".to_string(),
        }],
    );
    assert_eq!(ids, vec!["n_decl", "n_write", "n_return"]);
}
