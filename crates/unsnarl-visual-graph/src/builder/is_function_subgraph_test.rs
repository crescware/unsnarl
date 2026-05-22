//! Sibling tests for [`is_function_subgraph`].

use unsnarl_ir::scope_type::ScopeType;

use super::is_function_subgraph;
use crate::builder::testing::base_serialized_scope;

#[test]
fn function_scope_is_subgraph() {
    let mut scope = base_serialized_scope("s");
    scope.r#type = ScopeType::Function;
    assert!(is_function_subgraph(&scope));
}

#[test]
fn function_expression_name_scope_is_not_subgraph() {
    // The named-function-expression name binding never renders as a
    // subgraph even though its `type` is `Function`.
    let mut scope = base_serialized_scope("s");
    scope.r#type = ScopeType::Function;
    scope.function_expression_scope = true;
    assert!(!is_function_subgraph(&scope));
}

#[test]
fn non_function_scope_is_not_subgraph() {
    let mut scope = base_serialized_scope("s");
    scope.r#type = ScopeType::Block;
    assert!(!is_function_subgraph(&scope));
}
