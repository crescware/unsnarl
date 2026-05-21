//! Sibling tests for [`should_subgraph`]. Cases mirror
//! `ts/src/visual-graph/builder/should-subgraph.test.ts`.

use unsnarl_ir::scope::block_context::BlockContext;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_oxc_parity::AstType;

use super::should_subgraph;
use crate::builder::testing::{base_serialized_scope, other_block_context};

fn check(t: ScopeType, ctx: Option<BlockContext>) -> bool {
    let mut scope = base_serialized_scope("s");
    scope.r#type = t;
    scope.block_context = ctx;
    should_subgraph(&scope)
}

#[test]
fn function_scope_is_subgraph() {
    assert!(check(ScopeType::Function, None));
}

#[test]
fn class_scope_is_subgraph() {
    assert!(check(ScopeType::Class, None));
}

#[test]
fn control_kind_for_is_subgraph() {
    assert!(check(ScopeType::For, None));
}

#[test]
fn branch_block_if_consequent_is_subgraph() {
    let ctx = other_block_context(AstType::IfStatement, "consequent", 0, None);
    assert!(check(ScopeType::Block, Some(ctx)));
}

#[test]
fn bare_block_is_subgraph() {
    // Mirrors the TS fallthrough: a `Block` scope without a
    // recognised parent context still renders as the generic
    // 'block' subgraph.
    assert!(check(ScopeType::Block, None));
}

#[test]
fn module_scope_is_not_subgraph() {
    assert!(!check(ScopeType::Module, None));
}

#[test]
fn global_scope_is_not_subgraph() {
    assert!(!check(ScopeType::Global, None));
}
