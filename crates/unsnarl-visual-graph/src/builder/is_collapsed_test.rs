//! Drives the depth-gating predicate directly: only the recorded
//! `nesting_depths[kind]` is compared against the matching threshold,
//! and only when `depths` is provided. Module / function-expression
//! scopes never collapse because `nesting_kind_of` returns `None` for
//! them.

use super::is_collapsed;
use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths};
use unsnarl_ir::primitive::{SourceColumn, SourceLine, Span, Utf16CodeUnitOffset};
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::{SerializedBlock, SerializedScope, SerializedScopeId};
use unsnarl_oxc_parity::AstType;

fn scope_id(s: &str) -> SerializedScopeId {
    SerializedScopeId::new(s.to_string())
}

fn base_block() -> SerializedBlock {
    SerializedBlock {
        r#type: AstType::Program,
        span: Span {
            line: SourceLine(1),
            column: SourceColumn(0),
            offset: Utf16CodeUnitOffset(0),
        },
        end_span: Span {
            line: SourceLine(1),
            column: SourceColumn(0),
            offset: Utf16CodeUnitOffset(0),
        },
    }
}

fn base_scope() -> SerializedScope {
    SerializedScope {
        id: scope_id("scope_0"),
        r#type: ScopeType::Function,
        is_strict: false,
        upper: None,
        child_scopes: Vec::new(),
        variable_scope: scope_id("scope_0"),
        block: base_block(),
        variables: Vec::new(),
        references: Vec::new(),
        through: Vec::new(),
        function_expression_scope: false,
        block_context: None,
        falls_through: false,
        exits_function: false,
        nesting_depths: NestingDepths::uniform(NestingDepth(0)),
    }
}

fn uniform(n: u32) -> NestingDepths {
    NestingDepths::uniform(NestingDepth(n))
}

#[test]
fn returns_false_when_depths_option_is_absent() {
    let mut scope = base_scope();
    scope.nesting_depths.function = NestingDepth(99);
    assert!(!is_collapsed(&scope, None));
}

#[test]
fn returns_false_when_depth_equals_threshold() {
    let mut scope = base_scope();
    scope.nesting_depths.function = NestingDepth(1);
    let depths = uniform(1);
    assert!(!is_collapsed(&scope, Some(&depths)));
}

#[test]
fn returns_true_when_nesting_kind_depth_strictly_exceeds_threshold() {
    let mut scope = base_scope();
    scope.nesting_depths.function = NestingDepth(2);
    let depths = uniform(1);
    assert!(is_collapsed(&scope, Some(&depths)));
}

#[test]
fn each_nesting_kind_is_checked_independently() {
    let mut scope = base_scope();
    scope.r#type = ScopeType::For;
    scope.nesting_depths.function = NestingDepth(99);
    scope.nesting_depths.r#for = NestingDepth(1);

    let mut depths = uniform(10);
    depths.r#for = NestingDepth(1);
    assert!(!is_collapsed(&scope, Some(&depths)));

    let mut tighter = uniform(10);
    tighter.r#for = NestingDepth(0);
    assert!(is_collapsed(&scope, Some(&tighter)));
}

#[test]
fn non_counted_scopes_never_collapse() {
    let mut scope = base_scope();
    scope.function_expression_scope = true;
    scope.nesting_depths = uniform(99);
    let depths = uniform(0);
    assert!(!is_collapsed(&scope, Some(&depths)));
}
