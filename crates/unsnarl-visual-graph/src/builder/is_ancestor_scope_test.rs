//! Sibling tests for [`is_ancestor_scope`].

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

use super::is_ancestor_scope;
use crate::builder::testing::{base_serialized_scope, scope_id};

fn build_chain() -> [SerializedScope; 4] {
    let root = base_serialized_scope("root");
    let mut mid = base_serialized_scope("mid");
    mid.upper = Some(scope_id("root"));
    let mut leaf = base_serialized_scope("leaf");
    leaf.upper = Some(scope_id("mid"));
    let mut sibling = base_serialized_scope("sibling");
    sibling.upper = Some(scope_id("root"));
    [root, mid, leaf, sibling]
}

fn ancestor_map(scopes: &[SerializedScope]) -> HashMap<&str, &SerializedScope> {
    scopes.iter().map(|s| (s.id.value(), s)).collect()
}

#[test]
fn self_is_its_own_ancestor() {
    let scopes = build_chain();
    let map = ancestor_map(&scopes);
    assert!(is_ancestor_scope("leaf", "leaf", &map));
}

#[test]
fn direct_parent_is_ancestor() {
    let scopes = build_chain();
    let map = ancestor_map(&scopes);
    assert!(is_ancestor_scope("mid", "leaf", &map));
}

#[test]
fn grandparent_is_ancestor() {
    let scopes = build_chain();
    let map = ancestor_map(&scopes);
    assert!(is_ancestor_scope("root", "leaf", &map));
}

#[test]
fn child_is_not_ancestor_of_its_parent() {
    let scopes = build_chain();
    let map = ancestor_map(&scopes);
    assert!(!is_ancestor_scope("leaf", "mid", &map));
}

#[test]
fn sibling_is_not_ancestor() {
    let scopes = build_chain();
    let map = ancestor_map(&scopes);
    assert!(!is_ancestor_scope("sibling", "leaf", &map));
}

#[test]
fn missing_descendant_returns_false() {
    let scopes = build_chain();
    let map = ancestor_map(&scopes);
    assert!(!is_ancestor_scope("root", "missing", &map));
}

#[test]
fn broken_upper_chain_returns_false_at_the_break() {
    let mut orphan = base_serialized_scope("orphan");
    orphan.upper = Some(scope_id("missing"));
    let scopes = [orphan];
    let map = ancestor_map(&scopes);
    assert!(!is_ancestor_scope("any", "orphan", &map));
}
