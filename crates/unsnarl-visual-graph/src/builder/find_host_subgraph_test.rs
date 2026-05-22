//! Sibling tests for [`find_host_subgraph`].
//! The
//! Rust signature returns a [`SubgraphIdx`] handle (vs the TS
//! `VisualSubgraph` value) so each test asserts on the returned
//! handle's identity against the arena slots it populated.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

use crate::builder::arena::{BuildArena, SubgraphIdx};
use crate::builder::find_host_subgraph::find_host_subgraph;
use crate::builder::state::BuildState;
use crate::builder::testing::{base_serialized_reference, base_serialized_scope, scope_id};
use crate::direction::Direction;
use crate::visual_element_type::SubgraphTypeTag;
use crate::visual_subgraph::{OwnedExtras, OwnedSubgraphKind, OwnedVisualSubgraph, VisualSubgraph};

fn push_function_subgraph(arena: &mut BuildArena, id: &str) -> SubgraphIdx {
    arena.push_subgraph(VisualSubgraph::Owned(OwnedVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: id.to_string(),
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

fn build_scope_chain() -> [SerializedScope; 3] {
    let root = base_serialized_scope("root");
    let mut inner = base_serialized_scope("inner");
    inner.upper = Some(scope_id("root"));
    let mut leaf = base_serialized_scope("leaf");
    leaf.upper = Some(scope_id("inner"));
    [root, inner, leaf]
}

fn scope_map(scopes: &[SerializedScope]) -> HashMap<&str, &SerializedScope> {
    scopes.iter().map(|s| (s.id.value(), s)).collect()
}

#[test]
fn returns_subgraph_mapped_to_refs_own_scope_when_present() {
    let scopes = build_scope_chain();
    let map = scope_map(&scopes);
    let mut arena = BuildArena::new();
    let sg = push_function_subgraph(&mut arena, "s_leaf");
    let mut state = BuildState::new();
    state.subgraph_by_scope.insert("leaf".to_string(), sg);

    let mut r = base_serialized_reference();
    r.from = scope_id("leaf");

    assert_eq!(
        find_host_subgraph(&r, Some("fnVar"), &map, &state),
        Some(sg)
    );
}

#[test]
fn walks_up_via_upper_to_find_closest_enclosing_subgraph() {
    let scopes = build_scope_chain();
    let map = scope_map(&scopes);
    let mut arena = BuildArena::new();
    let sg = push_function_subgraph(&mut arena, "s_root");
    let mut state = BuildState::new();
    state.subgraph_by_scope.insert("root".to_string(), sg);

    let mut r = base_serialized_reference();
    r.from = scope_id("leaf");

    assert_eq!(
        find_host_subgraph(&r, Some("fnVar"), &map, &state),
        Some(sg)
    );
}

#[test]
fn falls_back_to_function_subgraph_for_enclosing_fn_variable_id() {
    let scopes = build_scope_chain();
    let map = scope_map(&scopes);
    let mut arena = BuildArena::new();
    let fn_sg = push_function_subgraph(&mut arena, "s_fn");
    let mut state = BuildState::new();
    state
        .function_subgraph_by_fn
        .insert("fnVar".to_string(), fn_sg);

    let mut r = base_serialized_reference();
    r.from = scope_id("leaf");

    assert_eq!(
        find_host_subgraph(&r, Some("fnVar"), &map, &state),
        Some(fn_sg)
    );
}

#[test]
fn returns_none_when_neither_chain_nor_fn_fallback_yields_subgraph() {
    let scopes = build_scope_chain();
    let map = scope_map(&scopes);
    let state = BuildState::new();

    let mut r = base_serialized_reference();
    r.from = scope_id("leaf");

    assert_eq!(find_host_subgraph(&r, Some("nope"), &map, &state), None);
}

#[test]
fn returns_none_when_ref_from_is_not_in_scope_map_and_fn_fallback_missing() {
    let scopes = build_scope_chain();
    let map = scope_map(&scopes);
    let state = BuildState::new();

    let mut r = base_serialized_reference();
    r.from = scope_id("missing");

    assert_eq!(
        find_host_subgraph(&r, Some("missingFn"), &map, &state),
        None
    );
}

#[test]
fn returns_scope_chain_subgraph_even_when_enclosing_fn_var_id_is_none() {
    let scopes = build_scope_chain();
    let map = scope_map(&scopes);
    let mut arena = BuildArena::new();
    let sg = push_function_subgraph(&mut arena, "s_root");
    let mut state = BuildState::new();
    state.subgraph_by_scope.insert("root".to_string(), sg);

    let mut r = base_serialized_reference();
    r.from = scope_id("leaf");

    assert_eq!(find_host_subgraph(&r, None, &map, &state), Some(sg));
}

#[test]
fn returns_none_when_enclosing_fn_var_id_is_none_and_scope_chain_empty() {
    let scopes = build_scope_chain();
    let map = scope_map(&scopes);
    let mut arena = BuildArena::new();
    let fn_sg = push_function_subgraph(&mut arena, "s_fn");
    let mut state = BuildState::new();
    state
        .function_subgraph_by_fn
        .insert("fnVar".to_string(), fn_sg);

    let mut r = base_serialized_reference();
    r.from = scope_id("leaf");

    assert_eq!(find_host_subgraph(&r, None, &map, &state), None);
}
