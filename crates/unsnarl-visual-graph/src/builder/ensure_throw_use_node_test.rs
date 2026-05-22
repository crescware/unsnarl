//! Sibling tests for [`ensure_throw_use_node`].

use unsnarl_ir::serialized::serialized_reference::SerializedReferenceIdentifier;
use unsnarl_ir::serialized::SerializedReference;
use unsnarl_ir::serialized::SerializedVariable;

use super::ensure_throw_use_node;
use crate::builder::arena::{BuildArena, ElementHandle, NodeIdx, SubgraphIdx};
use crate::builder::state::BuildState;
use crate::builder::testing::{
    base_builder_context, base_serialized_reference, empty_serialized_ir, jsx_container,
    reference_id, scope_id, span_offset_line, throw_completion, variable_id,
};
use crate::direction::Direction;
use crate::visual_element_type::SubgraphTypeTag;
use crate::visual_node::{SyntheticNodeKind, VisualNode};
use crate::visual_subgraph::{OwnedExtras, OwnedSubgraphKind, OwnedVisualSubgraph, VisualSubgraph};

fn push_host(arena: &mut BuildArena) -> SubgraphIdx {
    arena.push_subgraph(VisualSubgraph::Owned(OwnedVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: "s_fn".to_string(),
        kind: OwnedSubgraphKind::Function,
        line: 1,
        end_line: None,
        direction: Direction::RL,
        extras: OwnedExtras::Function {
            owner_node_id: Some("n_owner".to_string()),
            owner_name: "owner".to_string(),
        },
        elements: Vec::new(),
    }))
}

fn state_with_host(arena: &mut BuildArena) -> (BuildState, SubgraphIdx) {
    let host = push_host(arena);
    let mut state = BuildState::new();
    state
        .function_subgraph_by_fn
        .insert("fnVar".to_string(), host);
    (state, host)
}

fn throw_ref(
    ref_id_str: &str,
    start: u32,
    end: u32,
    start_line: u32,
    end_line: u32,
) -> SerializedReference {
    let mut r = base_serialized_reference();
    r.id = reference_id(ref_id_str);
    r.from = scope_id("scope");
    r.completion = throw_completion(start, end, start_line, end_line);
    r
}

fn first_subgraph_child(arena: &BuildArena, host: SubgraphIdx) -> SubgraphIdx {
    match arena.subgraph(host).children[0] {
        ElementHandle::Subgraph(idx) => idx,
        _ => panic!("expected subgraph child"),
    }
}

fn first_node_child(arena: &BuildArena, sg: SubgraphIdx) -> NodeIdx {
    match arena.subgraph(sg).children[0] {
        ElementHandle::Node(idx) => idx,
        _ => panic!("expected node child"),
    }
}

#[test]
fn returns_none_when_no_host_subgraph_exists() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut state = BuildState::new();
    let mut arena = BuildArena::new();
    let r = throw_ref("r", 0, 10, 1, 1);
    assert_eq!(
        ensure_throw_use_node(&mut arena, &mut state, &ctx, "fnVar", &r),
        None
    );
}

#[test]
fn returns_none_when_completion_is_not_throw() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_host(&mut arena);
    let mut r = base_serialized_reference();
    r.id = reference_id("r1");
    assert_eq!(
        ensure_throw_use_node(&mut arena, &mut state, &ctx, "fnVar", &r),
        None
    );
    assert!(arena.subgraph(host).children.is_empty());
}

#[test]
fn creates_throw_subgraph_and_throw_use_node() {
    let mut ir = empty_serialized_ir();
    ir.variables.push(SerializedVariable::new(
        variable_id("v"),
        "x".to_string(),
        scope_id("s"),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ));
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_host(&mut arena);
    let mut r = base_serialized_reference();
    r.id = reference_id("r1");
    r.identifier = SerializedReferenceIdentifier::new("x".to_string(), span_offset_line(0, 3));
    r.resolved = Some(variable_id("v"));
    r.completion = throw_completion(0, 50, 3, 5);
    let id = ensure_throw_use_node(&mut arena, &mut state, &ctx, "fnVar", &r);
    assert_eq!(id.as_deref(), Some("throw_use_r1"));
    assert_eq!(arena.subgraph(host).children.len(), 1);
    let throw_idx = first_subgraph_child(&arena, host);
    let VisualSubgraph::Owned(sg) = arena.subgraph(throw_idx).descriptor.clone() else {
        panic!("expected owned");
    };
    assert!(matches!(sg.kind, OwnedSubgraphKind::Throw));
    assert_eq!(sg.line, 3);
    assert_eq!(sg.end_line, Some(5));
    let node_idx = first_node_child(&arena, throw_idx);
    let VisualNode::Synthetic(node) = arena.node(node_idx).clone() else {
        panic!("expected synthetic");
    };
    assert!(matches!(
        node.kind,
        SyntheticNodeKind::ThrowArgumentReference
    ));
    assert_eq!(node.name, "x");
    assert_eq!(node.line, 3);
}

#[test]
fn uses_identifier_name_when_variable_is_not_resolved() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_host(&mut arena);
    let mut r = base_serialized_reference();
    r.id = reference_id("r1");
    r.identifier =
        SerializedReferenceIdentifier::new("literal".to_string(), span_offset_line(0, 1));
    r.resolved = None;
    r.completion = throw_completion(0, 10, 1, 1);
    ensure_throw_use_node(&mut arena, &mut state, &ctx, "fnVar", &r);
    let throw_idx = first_subgraph_child(&arena, host);
    let node_idx = first_node_child(&arena, throw_idx);
    let VisualNode::Synthetic(n) = arena.node(node_idx).clone() else {
        panic!("expected synthetic");
    };
    assert_eq!(n.name, "literal");
}

#[test]
fn groups_refs_with_same_throw_completion_offsets_into_one_subgraph() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_host(&mut arena);
    let r1 = throw_ref("r1", 0, 50, 3, 5);
    let r2 = throw_ref("r2", 0, 50, 3, 5);
    ensure_throw_use_node(&mut arena, &mut state, &ctx, "fnVar", &r1);
    ensure_throw_use_node(&mut arena, &mut state, &ctx, "fnVar", &r2);
    assert_eq!(arena.subgraph(host).children.len(), 1);
    let throw_idx = first_subgraph_child(&arena, host);
    let ids: Vec<String> = arena
        .subgraph(throw_idx)
        .children
        .iter()
        .map(|h| match h {
            ElementHandle::Node(idx) => arena.node(*idx).id().to_string(),
            _ => panic!("expected node"),
        })
        .collect();
    assert_eq!(ids, vec!["throw_use_r1", "throw_use_r2"]);
}

#[test]
fn different_throw_completion_offsets_create_separate_subgraphs() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_host(&mut arena);
    let r1 = throw_ref("r1", 0, 10, 1, 1);
    let r2 = throw_ref("r2", 20, 30, 1, 1);
    ensure_throw_use_node(&mut arena, &mut state, &ctx, "fnVar", &r1);
    ensure_throw_use_node(&mut arena, &mut state, &ctx, "fnVar", &r2);
    assert_eq!(arena.subgraph(host).children.len(), 2);
}

#[test]
fn does_not_duplicate_throw_use_node_when_called_twice_with_same_ref_id() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_host(&mut arena);
    let r = throw_ref("r1", 0, 10, 1, 1);
    ensure_throw_use_node(&mut arena, &mut state, &ctx, "fnVar", &r);
    ensure_throw_use_node(&mut arena, &mut state, &ctx, "fnVar", &r);
    let throw_idx = first_subgraph_child(&arena, host);
    assert_eq!(arena.subgraph(throw_idx).children.len(), 1);
}

#[test]
fn sets_is_jsx_element_and_end_line_when_reference_has_jsx_element() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_host(&mut arena);
    let mut r = base_serialized_reference();
    r.id = reference_id("r1");
    r.identifier = SerializedReferenceIdentifier::new("Foo".to_string(), span_offset_line(0, 2));
    r.jsx_element = Some(jsx_container(0, 30, 2, 5));
    r.completion = throw_completion(0, 30, 2, 5);
    ensure_throw_use_node(&mut arena, &mut state, &ctx, "fnVar", &r);
    let throw_idx = first_subgraph_child(&arena, host);
    let node_idx = first_node_child(&arena, throw_idx);
    let VisualNode::Synthetic(n) = arena.node(node_idx).clone() else {
        panic!("expected synthetic");
    };
    assert!(n.is_jsx_element);
    assert_eq!(n.end_line, Some(5));
}
