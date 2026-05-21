//! Sibling tests for [`find_enclosing_subgraph_scope`]. Cases mirror
//! `ts/src/visual-graph/builder/find-enclosing-subgraph-scope.test.ts`.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

use super::find_enclosing_subgraph_scope;
use crate::builder::testing::{base_serialized_scope, scope_id};

fn ancestor_chain() -> [SerializedScope; 3] {
    let grand = base_serialized_scope("grand");
    let mut parent = base_serialized_scope("parent");
    parent.upper = Some(scope_id("grand"));
    let mut child = base_serialized_scope("child");
    child.upper = Some(scope_id("parent"));
    [grand, parent, child]
}

fn scope_map(scopes: &[SerializedScope]) -> HashMap<&str, &SerializedScope> {
    scopes.iter().map(|s| (s.id.value(), s)).collect()
}

fn owners(entries: &[(&str, &str)]) -> HashMap<String, String> {
    entries
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

#[test]
fn starting_scope_itself_is_owner_returns_start() {
    let scopes = ancestor_chain();
    let map = scope_map(&scopes);
    let owners = owners(&[("child", "v")]);
    assert_eq!(
        find_enclosing_subgraph_scope("child", &map, &owners),
        Some("child".to_string())
    );
}

#[test]
fn walks_up_multiple_levels_to_find_owner() {
    let scopes = ancestor_chain();
    let map = scope_map(&scopes);
    let owners = owners(&[("grand", "v")]);
    assert_eq!(
        find_enclosing_subgraph_scope("child", &map, &owners),
        Some("grand".to_string())
    );
}

#[test]
fn finds_owner_one_level_up() {
    let scopes = ancestor_chain();
    let map = scope_map(&scopes);
    let owners = owners(&[("parent", "v")]);
    assert_eq!(
        find_enclosing_subgraph_scope("child", &map, &owners),
        Some("parent".to_string())
    );
}

#[test]
fn no_owner_anywhere_returns_none() {
    let scopes = ancestor_chain();
    let map = scope_map(&scopes);
    let owners = HashMap::new();
    assert_eq!(find_enclosing_subgraph_scope("child", &map, &owners), None);
}

#[test]
fn starting_scope_id_not_in_map_returns_none() {
    let scopes: [SerializedScope; 0] = [];
    let map = scope_map(&scopes);
    let owners = owners(&[("x", "v")]);
    assert_eq!(
        find_enclosing_subgraph_scope("missing", &map, &owners),
        None
    );
}

#[test]
fn broken_upper_chain_returns_none() {
    let mut child = base_serialized_scope("child");
    child.upper = Some(scope_id("missing"));
    let scopes = [child];
    let map = scope_map(&scopes);
    let owners = HashMap::new();
    assert_eq!(find_enclosing_subgraph_scope("child", &map, &owners), None);
}
