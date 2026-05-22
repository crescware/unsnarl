//! Sibling tests for [`is_control_subgraph`].

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_oxc_parity::AstType;

use super::is_control_subgraph;
use crate::builder::testing::{base_serialized_scope, other_block_context};

fn scope_with_type(t: ScopeType) -> bool {
    let mut scope = base_serialized_scope("s");
    scope.r#type = t;
    is_control_subgraph(&scope)
}

#[test]
fn for_scope_is_control_subgraph() {
    assert!(scope_with_type(ScopeType::For));
}

#[test]
fn catch_scope_is_control_subgraph() {
    assert!(scope_with_type(ScopeType::Catch));
}

#[test]
fn switch_scope_is_control_subgraph() {
    assert!(scope_with_type(ScopeType::Switch));
}

#[test]
fn function_scope_is_not_control_subgraph() {
    assert!(!scope_with_type(ScopeType::Function));
}

#[test]
fn module_scope_is_not_control_subgraph() {
    assert!(!scope_with_type(ScopeType::Module));
}

#[test]
fn global_scope_is_not_control_subgraph() {
    assert!(!scope_with_type(ScopeType::Global));
}

#[test]
fn class_scope_is_not_control_subgraph() {
    assert!(!scope_with_type(ScopeType::Class));
}

fn block_scope_with_context(parent_type: AstType, key: &str) -> bool {
    let mut scope = base_serialized_scope("s");
    scope.r#type = ScopeType::Block;
    scope.block_context = Some(other_block_context(parent_type, key, 0, None));
    is_control_subgraph(&scope)
}

#[test]
fn block_in_if_consequent_is_control_subgraph() {
    assert!(block_scope_with_context(AstType::IfStatement, "consequent"));
}

#[test]
fn block_in_if_alternate_is_control_subgraph() {
    assert!(block_scope_with_context(AstType::IfStatement, "alternate"));
}

#[test]
fn block_in_try_block_is_control_subgraph() {
    assert!(block_scope_with_context(AstType::TryStatement, "block"));
}

#[test]
fn block_in_try_finalizer_is_control_subgraph() {
    assert!(block_scope_with_context(AstType::TryStatement, "finalizer"));
}

#[test]
fn block_in_switch_cases_is_control_subgraph() {
    assert!(block_scope_with_context(AstType::SwitchStatement, "cases"));
}

#[test]
fn block_in_while_body_is_control_subgraph() {
    assert!(block_scope_with_context(AstType::WhileStatement, "body"));
}

#[test]
fn block_in_do_while_body_is_control_subgraph() {
    assert!(block_scope_with_context(AstType::DoWhileStatement, "body"));
}

#[test]
fn plain_block_without_block_context_is_control_subgraph() {
    // A bare block scope (no `blockContext`) still renders as the
    // generic 'block' subgraph.
    let mut scope = base_serialized_scope("s");
    scope.r#type = ScopeType::Block;
    assert!(is_control_subgraph(&scope));
}
