//! Sibling tests for [`nesting_kind_of`].

use unsnarl_ir::nesting_kind::NestingKind;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_oxc_parity::AstType;

use super::nesting_kind_of;
use crate::builder::testing::{base_serialized_scope, other_block_context};

#[test]
fn function_scope_returns_function() {
    let mut s = base_serialized_scope("s");
    s.r#type = ScopeType::Function;
    assert!(matches!(nesting_kind_of(&s), Some(NestingKind::Function)));
}

#[test]
fn function_expression_name_scope_returns_none() {
    // Not counted as a nesting scope.
    let mut s = base_serialized_scope("s");
    s.r#type = ScopeType::Function;
    s.function_expression_scope = true;
    assert!(nesting_kind_of(&s).is_none());
}

#[test]
fn for_scope_returns_for() {
    let mut s = base_serialized_scope("s");
    s.r#type = ScopeType::For;
    assert!(matches!(nesting_kind_of(&s), Some(NestingKind::For)));
}

#[test]
fn switch_scope_returns_switch() {
    let mut s = base_serialized_scope("s");
    s.r#type = ScopeType::Switch;
    assert!(matches!(nesting_kind_of(&s), Some(NestingKind::Switch)));
}

#[test]
fn catch_scope_returns_try_catch_finally() {
    let mut s = base_serialized_scope("s");
    s.r#type = ScopeType::Catch;
    assert!(matches!(
        nesting_kind_of(&s),
        Some(NestingKind::TryCatchFinally)
    ));
}

#[test]
fn block_scope_inside_if_statement_returns_if() {
    let mut s = base_serialized_scope("s");
    s.r#type = ScopeType::Block;
    s.block_context = Some(other_block_context(
        AstType::IfStatement,
        "consequent",
        0,
        None,
    ));
    assert!(matches!(nesting_kind_of(&s), Some(NestingKind::If)));
}

#[test]
fn block_scope_inside_for_statement_body_returns_for() {
    let mut s = base_serialized_scope("s");
    s.r#type = ScopeType::Block;
    s.block_context = Some(other_block_context(AstType::ForStatement, "body", 0, None));
    assert!(matches!(nesting_kind_of(&s), Some(NestingKind::For)));
}

#[test]
fn block_scope_inside_while_statement_body_returns_while() {
    let mut s = base_serialized_scope("s");
    s.r#type = ScopeType::Block;
    s.block_context = Some(other_block_context(
        AstType::WhileStatement,
        "body",
        0,
        None,
    ));
    assert!(matches!(nesting_kind_of(&s), Some(NestingKind::While)));
}

#[test]
fn block_scope_inside_try_statement_returns_try_catch_finally() {
    let mut s = base_serialized_scope("s");
    s.r#type = ScopeType::Block;
    s.block_context = Some(other_block_context(AstType::TryStatement, "block", 0, None));
    assert!(matches!(
        nesting_kind_of(&s),
        Some(NestingKind::TryCatchFinally)
    ));
}

#[test]
fn bare_block_scope_returns_block() {
    let mut s = base_serialized_scope("s");
    s.r#type = ScopeType::Block;
    s.block_context = None;
    assert!(matches!(nesting_kind_of(&s), Some(NestingKind::Block)));
}

#[test]
fn module_scope_returns_none() {
    let mut s = base_serialized_scope("s");
    s.r#type = ScopeType::Module;
    assert!(nesting_kind_of(&s).is_none());
}
