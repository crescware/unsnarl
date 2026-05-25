//! Sibling tests for [`branch_scope_of`].

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;
use unsnarl_oxc_parity::AstType;

use super::branch_scope_of;
use crate::builder::builder_fixtures::{base_serialized_scope, other_block_context, scope_id};

fn build_scopes() -> [SerializedScope; 4] {
    let outer = base_serialized_scope("outer");
    let mut if_branch = base_serialized_scope("if");
    if_branch.upper = Some(scope_id("outer"));
    if_branch.block_context = Some(other_block_context(
        AstType::IfStatement,
        "consequent",
        0,
        None,
    ));
    let mut inner = base_serialized_scope("inner");
    inner.upper = Some(scope_id("if"));
    let mut deeper = base_serialized_scope("deeper");
    deeper.upper = Some(scope_id("inner"));
    [outer, if_branch, inner, deeper]
}

fn ancestor_map(scopes: &[SerializedScope]) -> HashMap<&str, &SerializedScope> {
    scopes.iter().map(|s| (s.id.value(), s)).collect()
}

#[test]
fn branch_scope_itself_returns_its_own_id() {
    let scopes = build_scopes();
    let map = ancestor_map(&scopes);
    assert_eq!(branch_scope_of("if", &map), Some("if".to_string()));
}

#[test]
fn child_of_branch_returns_the_branch_id() {
    let scopes = build_scopes();
    let map = ancestor_map(&scopes);
    assert_eq!(branch_scope_of("inner", &map), Some("if".to_string()));
}

#[test]
fn deeper_descendant_still_resolves_to_branch() {
    let scopes = build_scopes();
    let map = ancestor_map(&scopes);
    assert_eq!(branch_scope_of("deeper", &map), Some("if".to_string()));
}

#[test]
fn non_branch_ancestor_chain_returns_none() {
    let scopes = build_scopes();
    let map = ancestor_map(&scopes);
    assert_eq!(branch_scope_of("outer", &map), None);
}

#[test]
fn missing_scope_returns_none() {
    let scopes = build_scopes();
    let map = ancestor_map(&scopes);
    assert_eq!(branch_scope_of("missing", &map), None);
}
