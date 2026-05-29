//! Sibling tests for [`find_host_scope_id`].

use super::find_host_scope_id;
use crate::builder::arena::{BuildArena, SubgraphIdx};
use crate::builder::builder_fixtures::{
    base_builder_context, base_serialized_reference, base_serialized_scope, empty_serialized_ir,
    scope_id,
};
use crate::builder::state::BuildState;
use crate::direction::Direction;
use crate::visual_subgraph::OwnedVisualSubgraph;
use unsnarl_ir::serialized::{SerializedReference, SerializedScope};

/// Push any subgraph so `subgraph_by_scope` can map a scope id to a
/// real handle. `find_host_scope_id` only consults the *keys*, so the
/// descriptor's contents are irrelevant.
fn dummy_subgraph(arena: &mut BuildArena) -> SubgraphIdx {
    arena.push_subgraph(
        OwnedVisualSubgraph::function("s_x", 1, None, "x", Vec::new(), Direction::RL).into(),
    )
}

fn scope_with_upper(id: &str, upper: Option<&str>) -> SerializedScope {
    let mut s = base_serialized_scope(id);
    s.upper = upper.map(scope_id);
    s
}

fn ref_from(scope: &str) -> SerializedReference {
    let mut r = base_serialized_reference();
    r.from = scope_id(scope);
    r
}

#[test]
fn returns_the_from_scope_when_it_materialised_a_subgraph() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(scope_with_upper("inner", Some("outer")));
    ir.scopes.push(scope_with_upper("outer", None));
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let idx = dummy_subgraph(&mut arena);
    let mut state = BuildState::new();
    state.subgraph_by_scope.insert("inner".to_string(), idx);
    let r = ref_from("inner");
    assert_eq!(
        find_host_scope_id(&r, &ctx.scope_map, &state),
        Some("inner")
    );
}

#[test]
fn walks_up_to_the_closest_materialised_ancestor() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(scope_with_upper("inner", Some("outer")));
    ir.scopes.push(scope_with_upper("outer", None));
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let idx = dummy_subgraph(&mut arena);
    let mut state = BuildState::new();
    // Only the ancestor materialised; the walk skips `inner`.
    state.subgraph_by_scope.insert("outer".to_string(), idx);
    let r = ref_from("inner");
    assert_eq!(
        find_host_scope_id(&r, &ctx.scope_map, &state),
        Some("outer")
    );
}

#[test]
fn returns_none_when_no_scope_in_the_chain_materialised() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(scope_with_upper("inner", Some("outer")));
    ir.scopes.push(scope_with_upper("outer", None));
    let ctx = base_builder_context(&ir);
    let state = BuildState::new();
    let r = ref_from("inner");
    assert_eq!(find_host_scope_id(&r, &ctx.scope_map, &state), None);
}

#[test]
fn returns_none_when_from_scope_is_absent_from_the_map() {
    let ir = empty_serialized_ir();
    let ctx = base_builder_context(&ir);
    let mut arena = BuildArena::new();
    let idx = dummy_subgraph(&mut arena);
    let mut state = BuildState::new();
    state.subgraph_by_scope.insert("ghost".to_string(), idx);
    let r = ref_from("ghost");
    // `ghost` is keyed in `subgraph_by_scope` but absent from
    // `scope_map`, so the walk never starts.
    assert_eq!(find_host_scope_id(&r, &ctx.scope_map, &state), None);
}
