//! Sibling tests for [`is_class_subgraph`]. Cases mirror
//! `ts/src/visual-graph/builder/is-class-subgraph.test.ts`.

use unsnarl_ir::scope_type::ScopeType;

use super::is_class_subgraph;
use crate::builder::testing::base_serialized_scope;

#[test]
fn class_scope_is_subgraph() {
    let mut scope = base_serialized_scope("s");
    scope.r#type = ScopeType::Class;
    assert!(is_class_subgraph(&scope));
}

#[test]
fn function_scope_is_not_class_subgraph() {
    let mut scope = base_serialized_scope("s");
    scope.r#type = ScopeType::Function;
    assert!(!is_class_subgraph(&scope));
}

#[test]
fn block_scope_is_not_class_subgraph() {
    let mut scope = base_serialized_scope("s");
    scope.r#type = ScopeType::Block;
    assert!(!is_class_subgraph(&scope));
}
