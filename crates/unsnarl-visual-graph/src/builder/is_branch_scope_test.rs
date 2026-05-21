//! Sibling tests for [`is_branch_scope`]. Cases mirror
//! `ts/src/visual-graph/builder/is-branch-scope.test.ts`.

use std::collections::HashMap;

use unsnarl_ir::scope::block_context::BlockContext;
use unsnarl_ir::serialized::SerializedScope;
use unsnarl_oxc_parity::AstType;

use super::is_branch_scope;
use crate::builder::testing::{base_serialized_scope, other_block_context};

fn scope_with_context(ctx: Option<BlockContext>) -> SerializedScope {
    let mut scope = base_serialized_scope("s");
    scope.block_context = ctx;
    scope
}

fn check(ctx: Option<BlockContext>) -> bool {
    let scope = scope_with_context(ctx);
    let mut map: HashMap<&str, &SerializedScope> = HashMap::new();
    map.insert(scope.id.value(), &scope);
    is_branch_scope("s", &map)
}

#[test]
fn if_consequent_block_scope_is_branch() {
    let ctx = other_block_context(AstType::IfStatement, "consequent", 0, None);
    assert!(check(Some(ctx)));
}

#[test]
fn if_alternate_block_scope_is_branch() {
    let ctx = other_block_context(AstType::IfStatement, "alternate", 0, None);
    assert!(check(Some(ctx)));
}

#[test]
fn switch_case_scope_is_branch() {
    // is_branch_scope routes through `branch_container_key`, which
    // for the `switch:` case key requires the `CaseClause` variant.
    // The TS test uses a generic block context, but the Rust impl
    // separates the variants — exercise the `CaseClause` variant
    // here so the test mirrors the underlying merge-key contract.
    use unsnarl_ir::primitive::SourceOffset;
    use unsnarl_ir::scope::block_context::CaseClauseBlockContext;
    let ctx = BlockContext::CaseClause(CaseClauseBlockContext::new(
        AstType::SwitchStatement,
        "cases".to_string(),
        SourceOffset(0),
        None,
    ));
    assert!(check(Some(ctx)));
}

#[test]
fn try_block_scope_is_branch() {
    // `try` and `catch` are sibling branches.
    let ctx = other_block_context(AstType::TryStatement, "block", 0, None);
    assert!(check(Some(ctx)));
}

#[test]
fn try_handler_scope_is_branch() {
    // `try` and `catch` are sibling branches.
    let ctx = other_block_context(AstType::TryStatement, "handler", 0, None);
    assert!(check(Some(ctx)));
}

#[test]
fn try_finalizer_scope_is_not_branch() {
    // `finally` is post-merge, not a branch.
    let ctx = other_block_context(AstType::TryStatement, "finalizer", 0, None);
    assert!(!check(Some(ctx)));
}

#[test]
fn scope_without_block_context_is_not_branch() {
    assert!(!check(None));
}

#[test]
fn scope_id_missing_from_map_is_not_branch() {
    let map: HashMap<&str, &SerializedScope> = HashMap::new();
    assert!(!is_branch_scope("missing", &map));
}
