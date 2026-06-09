//! Drives the depth-gating predicate directly: the supplied `rendered`
//! depths are compared kind-by-kind against the matching threshold,
//! and only when `ceiling` is provided. Module / function-expression
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
        callback_argument: None,
        falls_through: false,
        exits_function: false,
        nesting_depths: NestingDepths::uniform(NestingDepth(0)),
        abrupt_statements: Vec::new(),
    }
}

fn uniform(n: u32) -> NestingDepths {
    NestingDepths::uniform(NestingDepth(n))
}

#[test]
fn returns_false_when_ceiling_is_absent() {
    let scope = base_scope();
    let rendered = uniform(99);
    assert!(!is_collapsed(&scope, &rendered, None));
}

#[test]
fn returns_false_when_rendered_equals_ceiling() {
    let scope = base_scope();
    let mut rendered = uniform(0);
    rendered.function = NestingDepth(1);
    let ceiling = uniform(1);
    assert!(!is_collapsed(&scope, &rendered, Some(&ceiling)));
}

#[test]
fn returns_true_when_rendered_strictly_exceeds_ceiling() {
    let scope = base_scope();
    let mut rendered = uniform(0);
    rendered.function = NestingDepth(2);
    let ceiling = uniform(1);
    assert!(is_collapsed(&scope, &rendered, Some(&ceiling)));
}

#[test]
fn each_nesting_kind_is_checked_independently() {
    let mut scope = base_scope();
    scope.r#type = ScopeType::For;
    let mut rendered = uniform(0);
    rendered.function = NestingDepth(99);
    rendered.r#for = NestingDepth(1);

    let mut ceiling = uniform(10);
    ceiling.r#for = NestingDepth(1);
    assert!(!is_collapsed(&scope, &rendered, Some(&ceiling)));

    let mut tighter = uniform(10);
    tighter.r#for = NestingDepth(0);
    assert!(is_collapsed(&scope, &rendered, Some(&tighter)));
}

#[test]
fn non_counted_scopes_never_collapse() {
    let mut scope = base_scope();
    scope.function_expression_scope = true;
    let rendered = uniform(99);
    let ceiling = uniform(0);
    assert!(!is_collapsed(&scope, &rendered, Some(&ceiling)));
}

#[test]
fn ternary_arm_block_scope_collapses_under_block_ceiling() {
    use unsnarl_ir::scope::block_context::{BlockContext, OtherBlockContext};

    let mut scope = base_scope();
    scope.r#type = ScopeType::Block;
    scope.block_context = Some(BlockContext::Other(OtherBlockContext::new(
        AstType::ConditionalExpression,
        "consequent".to_string(),
        Utf16CodeUnitOffset(0),
        None,
    )));

    let mut rendered = uniform(0);
    rendered.block = NestingDepth(2);
    let mut ceiling = uniform(10);
    ceiling.block = NestingDepth(1);
    assert!(is_collapsed(&scope, &rendered, Some(&ceiling)));

    ceiling.block = NestingDepth(2);
    assert!(!is_collapsed(&scope, &rendered, Some(&ceiling)));
}
