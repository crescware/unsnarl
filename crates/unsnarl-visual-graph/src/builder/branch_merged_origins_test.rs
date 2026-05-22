//! Sibling tests for [`branch_merged_origins`].

use std::collections::HashSet;

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::SerializedScope;
use unsnarl_oxc_parity::AstType;

use super::branch_merged_origins;
use crate::builder::testing::{
    base_builder_context, base_serialized_scope, base_write_op, case_clause_block_context,
    empty_serialized_ir, other_block_context, scope_id,
};
use crate::builder::write_op::WriteOp;

fn write_op_at(ref_id: &str, offset: u32, scope_id_str: &str) -> WriteOp {
    WriteOp {
        ref_id: ref_id.to_string(),
        offset,
        scope_id: scope_id_str.to_string(),
        ..base_write_op()
    }
}

fn scope_with_upper(id: &str, upper: &str) -> SerializedScope {
    let mut s = base_serialized_scope(id);
    s.upper = Some(scope_id(upper));
    s
}

fn scope_with_block_context(
    id: &str,
    upper: &str,
    parent_type: AstType,
    key: &str,
    parent_span_offset: u32,
) -> SerializedScope {
    let mut s = scope_with_upper(id, upper);
    s.block_context = Some(other_block_context(
        parent_type,
        key,
        parent_span_offset,
        None,
    ));
    s
}

fn case_scope(id: &str, upper: &str, parent_span_offset: u32) -> SerializedScope {
    let mut s = scope_with_upper(id, upper);
    s.block_context = Some(case_clause_block_context(
        AstType::SwitchStatement,
        "cases",
        parent_span_offset,
        None,
    ));
    s
}

#[test]
fn returns_empty_when_the_branch_has_no_writes() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_upper("br", "root"));
    let ctx = base_builder_context(&ir);
    assert!(branch_merged_origins("br", &[], &ctx).is_empty());
}

#[test]
fn returns_linearly_last_write_when_it_sits_directly_in_branch_scope() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_upper("br", "root"));
    let ctx = base_builder_context(&ir);
    let op = write_op_at("rA", 10, "br");
    assert_eq!(branch_merged_origins("br", &[op], &ctx), vec!["wr_rA"]);
}

#[test]
fn recurses_into_nested_if_else_and_collects_both_arms() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_upper("outer", "root"));
    ir.scopes.push(scope_with_block_context(
        "cons",
        "outer",
        AstType::IfStatement,
        "consequent",
        50,
    ));
    ir.scopes.push(scope_with_block_context(
        "alt",
        "outer",
        AstType::IfStatement,
        "alternate",
        50,
    ));
    let ctx = base_builder_context(&ir);
    let op_cons = write_op_at("rCons", 60, "cons");
    let op_alt = write_op_at("rAlt", 70, "alt");
    let got: HashSet<String> = branch_merged_origins("outer", &[op_cons, op_alt], &ctx)
        .into_iter()
        .collect();
    let expected: HashSet<String> = ["wr_rCons", "wr_rAlt"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(got, expected);
}

#[test]
fn recurses_into_nested_switch_and_collects_all_reachable_cases() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_upper("outer", "root"));
    let mut switch_scope = scope_with_upper("switch", "outer");
    switch_scope.r#type = ScopeType::Switch;
    ir.scopes.push(switch_scope);
    ir.scopes.push(case_scope("c1", "switch", 100));
    ir.scopes.push(case_scope("c2", "switch", 100));
    let mut ctx = base_builder_context(&ir);
    let container_key = "switch:switch:100".to_string();
    let cases: Vec<&SerializedScope> = ir
        .scopes
        .iter()
        .filter(|s| s.id.value() == "c1" || s.id.value() == "c2")
        .collect();
    ctx.sorted_cases_by_container.insert(container_key, cases);
    let op_c1 = write_op_at("rC1", 110, "c1");
    let op_c2 = write_op_at("rC2", 120, "c2");
    let got: HashSet<String> = branch_merged_origins("outer", &[op_c1, op_c2], &ctx)
        .into_iter()
        .collect();
    let expected: HashSet<String> = ["wr_rC1", "wr_rC2"].iter().map(|s| s.to_string()).collect();
    assert_eq!(got, expected);
}

#[test]
fn excludes_switch_case_that_exits_function() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_upper("outer", "root"));
    let mut switch_scope = scope_with_upper("switch", "outer");
    switch_scope.r#type = ScopeType::Switch;
    ir.scopes.push(switch_scope);
    let mut c1 = case_scope("c1", "switch", 100);
    c1.exits_function = true;
    ir.scopes.push(c1);
    ir.scopes.push(case_scope("c2", "switch", 100));
    let mut ctx = base_builder_context(&ir);
    let container_key = "switch:switch:100".to_string();
    let cases: Vec<&SerializedScope> = ir
        .scopes
        .iter()
        .filter(|s| s.id.value() == "c1" || s.id.value() == "c2")
        .collect();
    ctx.sorted_cases_by_container.insert(container_key, cases);
    let op_c1 = write_op_at("rC1", 110, "c1");
    let op_c2 = write_op_at("rC2", 120, "c2");
    assert_eq!(
        branch_merged_origins("outer", &[op_c1, op_c2], &ctx),
        vec!["wr_rC2"]
    );
}

#[test]
fn descends_through_three_level_nesting() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_upper("outer", "root"));
    ir.scopes.push(scope_with_block_context(
        "midCons",
        "outer",
        AstType::IfStatement,
        "consequent",
        50,
    ));
    ir.scopes.push(scope_with_block_context(
        "midAlt",
        "outer",
        AstType::IfStatement,
        "alternate",
        50,
    ));
    ir.scopes.push(scope_with_block_context(
        "leafCons",
        "midAlt",
        AstType::IfStatement,
        "consequent",
        70,
    ));
    ir.scopes.push(scope_with_block_context(
        "leafAlt",
        "midAlt",
        AstType::IfStatement,
        "alternate",
        70,
    ));
    let ctx = base_builder_context(&ir);
    let op_mid = write_op_at("rMid", 60, "midCons");
    let op_leaf_c = write_op_at("rLeafC", 80, "leafCons");
    let op_leaf_a = write_op_at("rLeafA", 90, "leafAlt");
    let got: HashSet<String> =
        branch_merged_origins("outer", &[op_mid, op_leaf_c, op_leaf_a], &ctx)
            .into_iter()
            .collect();
    let expected: HashSet<String> = ["wr_rMid", "wr_rLeafC", "wr_rLeafA"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(got, expected);
}

#[test]
fn inner_if_without_alternate_keeps_pre_inner_write_reachable() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_upper("outer", "root"));
    ir.scopes.push(scope_with_block_context(
        "innerCons",
        "outer",
        AstType::IfStatement,
        "consequent",
        100,
    ));
    let ctx = base_builder_context(&ir);
    let op_pre = write_op_at("rPre", 60, "outer");
    let op_inner = write_op_at("rInner", 110, "innerCons");
    let got: HashSet<String> = branch_merged_origins("outer", &[op_pre, op_inner], &ctx)
        .into_iter()
        .collect();
    let expected: HashSet<String> = ["wr_rPre", "wr_rInner"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(got, expected);
}
