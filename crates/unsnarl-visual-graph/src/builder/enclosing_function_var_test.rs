//! Sibling tests for [`enclosing_function_var`]. Cases mirror
//! `ts/src/visual-graph/builder/enclosing-function-var.test.ts`.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

use super::enclosing_function_var;
use crate::builder::testing::{base_serialized_scope, scope_id};

fn build_chain() -> [SerializedScope; 3] {
    let grand = base_serialized_scope("grand");
    let mut parent = base_serialized_scope("parent");
    parent.upper = Some(scope_id("grand"));
    let mut child = base_serialized_scope("child");
    child.upper = Some(scope_id("parent"));
    [grand, parent, child]
}

fn ancestor_map(scopes: &[SerializedScope]) -> HashMap<&str, &SerializedScope> {
    scopes.iter().map(|s| (s.id.value(), s)).collect()
}

fn owners(entries: &[(&str, &str)]) -> HashMap<String, String> {
    entries
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

#[test]
fn owner_at_start_scope_returns_its_variable_id() {
    let scopes = build_chain();
    let map = ancestor_map(&scopes);
    let owners = owners(&[("child", "varChild")]);
    assert_eq!(
        enclosing_function_var("child", &map, &owners),
        Some("varChild".to_string())
    );
}

#[test]
fn owner_higher_up_returns_that_ancestors_variable_id() {
    let scopes = build_chain();
    let map = ancestor_map(&scopes);
    let owners = owners(&[("grand", "varGrand")]);
    assert_eq!(
        enclosing_function_var("child", &map, &owners),
        Some("varGrand".to_string())
    );
}

#[test]
fn no_owner_anywhere_returns_none() {
    let scopes = build_chain();
    let map = ancestor_map(&scopes);
    let owners = HashMap::new();
    assert_eq!(enclosing_function_var("child", &map, &owners), None);
}

#[test]
fn starting_scope_missing_from_map_returns_none() {
    let scopes = build_chain();
    let map = ancestor_map(&scopes);
    let owners = owners(&[("x", "v")]);
    assert_eq!(enclosing_function_var("missing", &map, &owners), None);
}
