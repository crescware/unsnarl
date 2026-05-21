//! Sibling tests for [`resolve_read_target_id`]. Cases mirror
//! `ts/src/visual-graph/builder/resolve-read-target-id.test.ts`.

use unsnarl_ir::serialized::SerializedReference;

use super::resolve_read_target_id;
use crate::builder::arena::{BuildArena, ElementHandle, SubgraphIdx};
use crate::builder::module_root_id::MODULE_ROOT_ID;
use crate::builder::state::BuildState;
use crate::builder::testing::{
    base_builder_context, base_serialized_reference, empty_serialized_ir, normal_completion,
    reference_id, return_completion, scope_id, throw_completion,
};
use crate::direction::Direction;
use crate::visual_element_type::SubgraphTypeTag;
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

fn read_ref_with(completion: unsnarl_ir::serialized::SerializedCompletion) -> SerializedReference {
    let mut r = base_serialized_reference();
    r.id = reference_id("r1");
    r.from = scope_id("scope");
    r.completion = completion;
    r
}

#[test]
fn returns_expr_stmt_id_verbatim() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, _host) = state_with_host(&mut arena);
    let r = read_ref_with(return_completion(0, 10, 1, 1));
    let result = resolve_read_target_id(
        &mut arena,
        &mut state,
        &ctx,
        Some("expr_42"),
        Some("fnVar"),
        &r,
    );
    assert_eq!(result, "expr_42");
}

#[test]
fn returns_expr_stmt_id_verbatim_even_when_enclosing_fn_is_none() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let r = read_ref_with(normal_completion());
    let result = resolve_read_target_id(&mut arena, &mut state, &ctx, Some("expr_42"), None, &r);
    assert_eq!(result, "expr_42");
}

#[test]
fn expr_stmt_short_circuit_produces_no_return_side_effects() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_host(&mut arena);
    let r = read_ref_with(return_completion(0, 10, 1, 1));
    resolve_read_target_id(
        &mut arena,
        &mut state,
        &ctx,
        Some("expr_42"),
        Some("fnVar"),
        &r,
    );
    assert!(state.return_subgraphs_by_fn.is_empty());
    assert!(arena.subgraph(host).children.is_empty());
}

#[test]
fn expr_stmt_short_circuit_produces_no_throw_side_effects() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_host(&mut arena);
    let r = read_ref_with(throw_completion(0, 10, 1, 1));
    resolve_read_target_id(
        &mut arena,
        &mut state,
        &ctx,
        Some("expr_42"),
        Some("fnVar"),
        &r,
    );
    assert!(state.throw_subgraphs_by_fn.is_empty());
    assert!(arena.subgraph(host).children.is_empty());
}

#[test]
fn null_expr_and_null_fn_falls_back_to_module_root() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let r = read_ref_with(normal_completion());
    let result = resolve_read_target_id(&mut arena, &mut state, &ctx, None, None, &r);
    assert_eq!(result, MODULE_ROOT_ID);
}

#[test]
fn return_completion_with_host_returns_return_use_id() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_host(&mut arena);
    let r = read_ref_with(return_completion(0, 50, 3, 5));
    let result = resolve_read_target_id(&mut arena, &mut state, &ctx, None, Some("fnVar"), &r);
    assert_eq!(result, "ret_use_r1");
    assert_eq!(
        state.return_subgraphs_by_fn.get("fnVar").map(|m| m.len()),
        Some(1)
    );
    assert_eq!(arena.subgraph(host).children.len(), 1);
}

#[test]
fn throw_completion_with_host_returns_throw_use_id() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_host(&mut arena);
    let r = read_ref_with(throw_completion(0, 50, 3, 5));
    let result = resolve_read_target_id(&mut arena, &mut state, &ctx, None, Some("fnVar"), &r);
    assert_eq!(result, "throw_use_r1");
    assert_eq!(
        state.throw_subgraphs_by_fn.get("fnVar").map(|m| m.len()),
        Some(1)
    );
    assert_eq!(arena.subgraph(host).children.len(), 1);
}

#[test]
fn normal_completion_inside_function_falls_back_to_module_root() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, _host) = state_with_host(&mut arena);
    let r = read_ref_with(normal_completion());
    let result = resolve_read_target_id(&mut arena, &mut state, &ctx, None, Some("fnVar"), &r);
    assert_eq!(result, MODULE_ROOT_ID);
}

#[test]
fn collapsed_body_with_no_host_falls_back_to_module_root_for_return() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let r = read_ref_with(return_completion(0, 10, 1, 1));
    let result = resolve_read_target_id(&mut arena, &mut state, &ctx, None, Some("fnVar"), &r);
    assert_eq!(result, MODULE_ROOT_ID);
}

#[test]
fn collapsed_body_with_no_host_falls_back_to_module_root_for_throw() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let r = read_ref_with(throw_completion(0, 10, 1, 1));
    let result = resolve_read_target_id(&mut arena, &mut state, &ctx, None, Some("fnVar"), &r);
    assert_eq!(result, MODULE_ROOT_ID);
}

#[test]
fn top_level_return_completion_falls_back_to_module_root() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let r = read_ref_with(return_completion(0, 10, 1, 1));
    let result = resolve_read_target_id(&mut arena, &mut state, &ctx, None, None, &r);
    assert_eq!(result, MODULE_ROOT_ID);
}

#[test]
fn top_level_throw_completion_falls_back_to_module_root() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let r = read_ref_with(throw_completion(0, 10, 1, 1));
    let result = resolve_read_target_id(&mut arena, &mut state, &ctx, None, None, &r);
    assert_eq!(result, MODULE_ROOT_ID);
}

#[test]
fn idempotent_on_repeated_calls_with_same_reference() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_host(&mut arena);
    let r = read_ref_with(return_completion(0, 50, 3, 5));
    let first = resolve_read_target_id(&mut arena, &mut state, &ctx, None, Some("fnVar"), &r);
    let second = resolve_read_target_id(&mut arena, &mut state, &ctx, None, Some("fnVar"), &r);
    assert_eq!(first, second);
    // Host still has a single Return subgraph.
    assert_eq!(arena.subgraph(host).children.len(), 1);
    let ElementHandle::Subgraph(ret_idx) = arena.subgraph(host).children[0] else {
        panic!("expected subgraph");
    };
    // The Return subgraph still holds a single ReturnUse node.
    assert_eq!(arena.subgraph(ret_idx).children.len(), 1);
}
