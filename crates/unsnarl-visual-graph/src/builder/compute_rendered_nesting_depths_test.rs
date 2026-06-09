//! Sibling tests for `compute_rendered_nesting_depths`.
//!
//! Builds toy `SerializedIR` trees by hand and asserts that each
//! scope's recorded depth matches the subgraph hierarchy actually
//! rendered — specifically that ternary-arm Block scopes increment
//! the `Block` counter just like AST-anchored Block subgraphs would,
//! and that scopes which do not produce a subgraph (e.g. Module /
//! function-expression) leave the counters untouched.

use super::compute_rendered_nesting_depths;
use unsnarl_ir::diagnostic::Diagnostic;
use unsnarl_ir::language::Language;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::serialized_ir::SERIALIZED_IR_VERSION;
use unsnarl_ir::serialized::{SerializedIR, SerializedScope, SerializedSource};
use unsnarl_oxc_parity::AstType;

use crate::builder::builder_fixtures::{
    base_serialized_scope, case_clause_block_context, other_block_context, scope_id,
};

fn ir(scopes: Vec<SerializedScope>) -> SerializedIR {
    SerializedIR {
        version: SERIALIZED_IR_VERSION,
        source: SerializedSource {
            path: "test.ts".to_string(),
            language: Language::Ts,
        },
        raw: String::new(),
        scopes,
        variables: Vec::new(),
        references: Vec::new(),
        unused_variable_ids: Vec::new(),
        diagnostics: Vec::<Diagnostic>::new(),
    }
}

fn child_of(s: &mut SerializedScope, child_ids: &[&str]) {
    s.child_scopes = child_ids.iter().map(|c| scope_id(c)).collect();
}

#[test]
fn module_root_records_all_zero() {
    let mut m = base_serialized_scope("m");
    m.r#type = ScopeType::Module;
    let ir = ir(vec![m]);
    let depths = compute_rendered_nesting_depths(&ir);
    let d = depths.get("m").expect("module scope recorded");
    assert_eq!(d.function.0, 0);
    assert_eq!(d.r#if.0, 0);
    assert_eq!(d.block.0, 0);
}

#[test]
fn function_scope_inside_module_increments_function_counter() {
    let mut m = base_serialized_scope("m");
    m.r#type = ScopeType::Module;
    child_of(&mut m, &["f"]);

    let mut f = base_serialized_scope("f");
    f.r#type = ScopeType::Function;
    f.upper = Some(scope_id("m"));

    let depths = compute_rendered_nesting_depths(&ir(vec![m, f]));
    assert_eq!(depths.get("m").expect("scope present").function.0, 0);
    assert_eq!(depths.get("f").expect("scope present").function.0, 1);
}

#[test]
fn function_expression_scope_does_not_increment() {
    let mut m = base_serialized_scope("m");
    m.r#type = ScopeType::Module;
    child_of(&mut m, &["fexpr"]);

    let mut fexpr = base_serialized_scope("fexpr");
    fexpr.r#type = ScopeType::Function;
    fexpr.upper = Some(scope_id("m"));
    fexpr.function_expression_scope = true;

    let depths = compute_rendered_nesting_depths(&ir(vec![m, fexpr]));
    assert_eq!(depths.get("fexpr").expect("scope present").function.0, 0);
}

#[test]
fn nested_if_consequent_blocks_increment_if_counter_per_level() {
    let mut m = base_serialized_scope("m");
    m.r#type = ScopeType::Module;
    child_of(&mut m, &["c1"]);

    let mut c1 = base_serialized_scope("c1");
    c1.upper = Some(scope_id("m"));
    c1.block_context = Some(other_block_context(
        AstType::IfStatement,
        "consequent",
        0,
        None,
    ));
    child_of(&mut c1, &["c2"]);

    let mut c2 = base_serialized_scope("c2");
    c2.upper = Some(scope_id("c1"));
    c2.block_context = Some(other_block_context(
        AstType::IfStatement,
        "consequent",
        0,
        None,
    ));

    let depths = compute_rendered_nesting_depths(&ir(vec![m, c1, c2]));
    assert_eq!(depths.get("c1").expect("scope present").r#if.0, 1);
    assert_eq!(depths.get("c2").expect("scope present").r#if.0, 2);
}

#[test]
fn nested_ternary_arms_increment_block_counter() {
    let mut m = base_serialized_scope("m");
    m.r#type = ScopeType::Module;
    child_of(&mut m, &["a1"]);

    let mut a1 = base_serialized_scope("a1");
    a1.upper = Some(scope_id("m"));
    a1.block_context = Some(other_block_context(
        AstType::ConditionalExpression,
        "consequent",
        0,
        None,
    ));
    child_of(&mut a1, &["a2"]);

    let mut a2 = base_serialized_scope("a2");
    a2.upper = Some(scope_id("a1"));
    a2.block_context = Some(other_block_context(
        AstType::ConditionalExpression,
        "alternate",
        0,
        None,
    ));
    child_of(&mut a2, &["a3"]);

    let mut a3 = base_serialized_scope("a3");
    a3.upper = Some(scope_id("a2"));
    a3.block_context = Some(other_block_context(
        AstType::ConditionalExpression,
        "consequent",
        0,
        None,
    ));

    let depths = compute_rendered_nesting_depths(&ir(vec![m, a1, a2, a3]));
    assert_eq!(depths.get("a1").expect("scope present").block.0, 1);
    assert_eq!(depths.get("a2").expect("scope present").block.0, 2);
    assert_eq!(depths.get("a3").expect("scope present").block.0, 3);
    assert_eq!(depths.get("a3").expect("scope present").r#if.0, 0);
}

#[test]
fn for_wrapper_is_skipped_so_body_block_counts_one_step_per_for() {
    let mut m = base_serialized_scope("m");
    m.r#type = ScopeType::Module;
    child_of(&mut m, &["f1"]);

    let mut f1 = base_serialized_scope("f1");
    f1.r#type = ScopeType::For;
    f1.upper = Some(scope_id("m"));
    child_of(&mut f1, &["b1"]);

    let mut b1 = base_serialized_scope("b1");
    b1.upper = Some(scope_id("f1"));
    b1.block_context = Some(other_block_context(AstType::ForStatement, "body", 0, None));

    let depths = compute_rendered_nesting_depths(&ir(vec![m, f1, b1]));
    assert_eq!(depths.get("f1").expect("scope present").r#for.0, 0);
    assert_eq!(depths.get("b1").expect("scope present").r#for.0, 1);
}

#[test]
fn catch_wrapper_is_skipped_so_body_block_counts_one_step_per_catch() {
    let mut m = base_serialized_scope("m");
    m.r#type = ScopeType::Module;
    child_of(&mut m, &["c1"]);

    let mut c1 = base_serialized_scope("c1");
    c1.r#type = ScopeType::Catch;
    c1.upper = Some(scope_id("m"));
    child_of(&mut c1, &["b1"]);

    let mut b1 = base_serialized_scope("b1");
    b1.upper = Some(scope_id("c1"));
    b1.block_context = Some(other_block_context(AstType::CatchClause, "body", 0, None));

    let depths = compute_rendered_nesting_depths(&ir(vec![m, c1, b1]));
    assert_eq!(
        depths.get("c1").expect("scope present").try_catch_finally.0,
        0
    );
    assert_eq!(
        depths.get("b1").expect("scope present").try_catch_finally.0,
        1
    );
}

#[test]
fn switch_wrapper_counts_once_even_with_multiple_cases() {
    let mut m = base_serialized_scope("m");
    m.r#type = ScopeType::Module;
    child_of(&mut m, &["sw"]);

    let mut sw = base_serialized_scope("sw");
    sw.r#type = ScopeType::Switch;
    sw.upper = Some(scope_id("m"));
    child_of(&mut sw, &["case1", "case2"]);

    let mut case1 = base_serialized_scope("case1");
    case1.upper = Some(scope_id("sw"));
    case1.block.r#type = AstType::SwitchCase;
    case1.block_context = Some(case_clause_block_context(
        AstType::SwitchStatement,
        "cases",
        0,
        Some("1"),
    ));

    let mut case2 = base_serialized_scope("case2");
    case2.upper = Some(scope_id("sw"));
    case2.block.r#type = AstType::SwitchCase;
    case2.block_context = Some(case_clause_block_context(
        AstType::SwitchStatement,
        "cases",
        0,
        Some("2"),
    ));

    let depths = compute_rendered_nesting_depths(&ir(vec![m, sw, case1, case2]));
    assert_eq!(depths.get("sw").expect("scope present").switch.0, 1);
    assert_eq!(depths.get("case1").expect("scope present").switch.0, 1);
    assert_eq!(depths.get("case2").expect("scope present").switch.0, 1);
}
