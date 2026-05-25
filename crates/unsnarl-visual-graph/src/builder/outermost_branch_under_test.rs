//! Sibling tests for [`outermost_branch_under`].

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;
use unsnarl_oxc_parity::AstType;

use super::outermost_branch_under;
use crate::builder::builder_fixtures::{base_serialized_scope, other_block_context, scope_id};

fn build_scopes() -> [SerializedScope; 4] {
    let root = base_serialized_scope("root");
    let mut outer = base_serialized_scope("outer");
    outer.upper = Some(scope_id("root"));
    let mut if_branch = base_serialized_scope("if");
    if_branch.upper = Some(scope_id("outer"));
    if_branch.block_context = Some(other_block_context(
        AstType::IfStatement,
        "consequent",
        0,
        None,
    ));
    let mut inner_branch = base_serialized_scope("inner");
    inner_branch.upper = Some(scope_id("if"));
    inner_branch.block_context = Some(other_block_context(
        AstType::IfStatement,
        "consequent",
        99,
        None,
    ));
    [root, outer, if_branch, inner_branch]
}

fn scope_map(scopes: &[SerializedScope]) -> HashMap<&str, &SerializedScope> {
    scopes.iter().map(|s| (s.id.value(), s)).collect()
}

#[test]
fn returns_none_when_scope_id_equals_branch_id() {
    let scopes = build_scopes();
    let map = scope_map(&scopes);
    assert_eq!(outermost_branch_under("outer", "outer", &map), None);
}

#[test]
fn returns_the_immediate_branch_child_when_scope_id_is_that_branch() {
    let scopes = build_scopes();
    let map = scope_map(&scopes);
    assert_eq!(
        outermost_branch_under("outer", "if", &map),
        Some("if".to_string())
    );
}

#[test]
fn walks_up_through_nested_branches_returns_outermost_under_branch() {
    let scopes = build_scopes();
    let map = scope_map(&scopes);
    assert_eq!(
        outermost_branch_under("outer", "inner", &map),
        Some("if".to_string())
    );
}

#[test]
fn returns_none_when_scope_id_is_not_under_branch_id() {
    let scopes = build_scopes();
    let map = scope_map(&scopes);
    assert_eq!(outermost_branch_under("if", "outer", &map), None);
}

#[test]
fn returns_none_when_traversal_hits_top_without_seeing_branch_id() {
    let scopes = build_scopes();
    let map = scope_map(&scopes);
    assert_eq!(outermost_branch_under("missing", "inner", &map), None);
}
