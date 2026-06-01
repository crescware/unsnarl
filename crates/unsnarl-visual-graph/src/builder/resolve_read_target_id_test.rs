//! Sibling tests for [`resolve_read_target_id`].

use unsnarl_ir::serialized::SerializedReference;

use super::resolve_read_target_id;
use crate::builder::arena::{BuildArena, ElementHandle, SubgraphIdx};
use crate::builder::builder_fixtures::{
    base_builder_context, base_serialized_reference, base_serialized_scope, empty_serialized_ir,
    normal_completion, reference_id, return_completion, scope_id, throw_completion,
};
use crate::builder::module_root_id::MODULE_ROOT_ID;
use crate::builder::state::BuildState;
use crate::direction::Direction;
use crate::visual_subgraph::OwnedVisualSubgraph;

fn push_host(arena: &mut BuildArena) -> SubgraphIdx {
    arena.push_subgraph(
        OwnedVisualSubgraph::function(
            "s_fn",
            1,
            Some("n_owner".to_string()),
            "owner",
            Vec::new(),
            Direction::RL,
        )
        .into(),
    )
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
fn return_completion_prefers_return_use_over_expr_stmt_id() {
    // A returned value resolves to its return-use node even when the read
    // also sits inside an ExpressionStatement: an arrow's implicit return
    // in a statement-level method chain belongs to its own call, not the
    // enclosing statement. The `Normal` case still short-circuits to the
    // expr-stmt id (see `returns_expr_stmt_id_verbatim_even_when_enclosing_fn_is_none`).
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
        None,
        &r,
    );
    assert_eq!(result, "ret_use_r1");
}

#[test]
fn returns_expr_stmt_id_verbatim_even_when_enclosing_fn_is_none() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let r = read_ref_with(normal_completion());
    let result = resolve_read_target_id(
        &mut arena,
        &mut state,
        &ctx,
        Some("expr_42"),
        None,
        None,
        &r,
    );
    assert_eq!(result, "expr_42");
}

#[test]
fn return_completion_creates_return_use_despite_expr_stmt_id() {
    // The return-use side effect (a Return subgraph + node in the host)
    // fires for a Return completion even when an expr-stmt id is present,
    // because the return-use target now takes precedence over the claim.
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
        None,
        &r,
    );
    assert_eq!(
        state.return_subgraphs_by_fn.get("fnVar").map(|m| m.len()),
        Some(1)
    );
    assert_eq!(arena.subgraph(host).children.len(), 1);
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
        None,
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
    let result = resolve_read_target_id(&mut arena, &mut state, &ctx, None, None, None, &r);
    assert_eq!(result, MODULE_ROOT_ID);
}

#[test]
fn return_completion_with_host_returns_return_use_id() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_host(&mut arena);
    let r = read_ref_with(return_completion(0, 50, 3, 5));
    let result =
        resolve_read_target_id(&mut arena, &mut state, &ctx, None, Some("fnVar"), None, &r);
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
    let result =
        resolve_read_target_id(&mut arena, &mut state, &ctx, None, Some("fnVar"), None, &r);
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
    let result =
        resolve_read_target_id(&mut arena, &mut state, &ctx, None, Some("fnVar"), None, &r);
    assert_eq!(result, MODULE_ROOT_ID);
}

#[test]
fn collapsed_body_with_no_host_falls_back_to_module_root_for_return() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let r = read_ref_with(return_completion(0, 10, 1, 1));
    let result =
        resolve_read_target_id(&mut arena, &mut state, &ctx, None, Some("fnVar"), None, &r);
    assert_eq!(result, MODULE_ROOT_ID);
}

#[test]
fn collapsed_body_with_no_host_falls_back_to_module_root_for_throw() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let r = read_ref_with(throw_completion(0, 10, 1, 1));
    let result =
        resolve_read_target_id(&mut arena, &mut state, &ctx, None, Some("fnVar"), None, &r);
    assert_eq!(result, MODULE_ROOT_ID);
}

#[test]
fn top_level_return_completion_falls_back_to_module_root() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let r = read_ref_with(return_completion(0, 10, 1, 1));
    let result = resolve_read_target_id(&mut arena, &mut state, &ctx, None, None, None, &r);
    assert_eq!(result, MODULE_ROOT_ID);
}

#[test]
fn top_level_throw_completion_falls_back_to_module_root() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let r = read_ref_with(throw_completion(0, 10, 1, 1));
    let result = resolve_read_target_id(&mut arena, &mut state, &ctx, None, None, None, &r);
    assert_eq!(result, MODULE_ROOT_ID);
}

#[test]
fn idempotent_on_repeated_calls_with_same_reference() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_host(&mut arena);
    let r = read_ref_with(return_completion(0, 50, 3, 5));
    let first = resolve_read_target_id(&mut arena, &mut state, &ctx, None, Some("fnVar"), None, &r);
    let second =
        resolve_read_target_id(&mut arena, &mut state, &ctx, None, Some("fnVar"), None, &r);
    assert_eq!(first, second);
    // Host still has a single Return subgraph.
    assert_eq!(arena.subgraph(host).children.len(), 1);
    let ElementHandle::Subgraph(ret_idx) = arena.subgraph(host).children[0] else {
        panic!("expected subgraph");
    };
    // The Return subgraph still holds a single ReturnUse node.
    assert_eq!(arena.subgraph(ret_idx).children.len(), 1);
}

/// Sets up a host found via the scope-direct walk (no owner var):
/// `scope_map` carries scope `scope`, and `subgraph_by_scope` maps it
/// to a freshly pushed host subgraph. Mirrors the owner-var-less
/// callback case (`const Panel = wrap(arrow)`).
fn state_with_scope_host(arena: &mut BuildArena) -> (BuildState, SubgraphIdx) {
    let host = push_host(arena);
    let mut state = BuildState::new();
    state.subgraph_by_scope.insert("scope".to_string(), host);
    (state, host)
}

#[test]
fn return_without_owner_var_hosts_via_scope_id() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("scope"));
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_scope_host(&mut arena);
    let r = read_ref_with(return_completion(0, 50, 3, 5));
    let result =
        resolve_read_target_id(&mut arena, &mut state, &ctx, None, None, Some("scope"), &r);
    assert_eq!(result, "ret_use_r1");
    // The wrapping Return subgraph is keyed by the host scope id, not
    // by any owner variable.
    assert_eq!(
        state.return_subgraphs_by_fn.get("scope").map(|m| m.len()),
        Some(1)
    );
    assert_eq!(arena.subgraph(host).children.len(), 1);
}

#[test]
fn throw_without_owner_var_hosts_via_scope_id() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("scope"));
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let (mut state, host) = state_with_scope_host(&mut arena);
    let r = read_ref_with(throw_completion(0, 50, 3, 5));
    let result =
        resolve_read_target_id(&mut arena, &mut state, &ctx, None, None, Some("scope"), &r);
    assert_eq!(result, "throw_use_r1");
    assert_eq!(
        state.throw_subgraphs_by_fn.get("scope").map(|m| m.len()),
        Some(1)
    );
    assert_eq!(arena.subgraph(host).children.len(), 1);
}

#[test]
fn return_without_owner_var_and_without_host_falls_back_to_module_root() {
    // No `subgraph_by_scope` entry and no owner var: `find_host_subgraph`
    // yields nothing, so even a `return` completion lands on module_root.
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("scope"));
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let mut state = BuildState::new();
    let r = read_ref_with(return_completion(0, 50, 3, 5));
    let result =
        resolve_read_target_id(&mut arena, &mut state, &ctx, None, None, Some("scope"), &r);
    assert_eq!(result, MODULE_ROOT_ID);
    assert!(state.return_subgraphs_by_fn.is_empty());
}
