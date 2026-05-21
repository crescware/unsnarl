//! Sibling tests for [`read_origins`]. Cases mirror
//! `ts/src/visual-graph/builder/read-origins.test.ts`.

use std::collections::HashSet;

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::SerializedScope;
use unsnarl_oxc_parity::AstType;

use super::read_origins;
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

fn scope_with_other_ctx(
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
fn no_prior_writes_returns_node_id_of_variable() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("s"));
    let ctx = base_builder_context(&ir);
    assert_eq!(read_origins("v", 100, "s", &ctx), vec!["n_v"]);
}

#[test]
fn prior_write_in_ancestor_scope_returns_its_write_op_node_id() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_upper("child", "root"));
    let mut ctx = base_builder_context(&ir);
    let op = write_op_at("rRoot", 5, "root");
    ctx.write_ops_by_variable.insert("v".to_string(), vec![op]);
    assert_eq!(read_origins("v", 50, "child", &ctx), vec!["wr_rRoot"]);
}

#[test]
fn prior_write_in_non_ancestor_non_branch_scope_returns_its_write_op_node_id() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_upper("a", "root"));
    ir.scopes.push(scope_with_upper("b", "root"));
    let mut ctx = base_builder_context(&ir);
    let op = write_op_at("rA", 5, "a");
    ctx.write_ops_by_variable.insert("v".to_string(), vec![op]);
    assert_eq!(read_origins("v", 50, "b", &ctx), vec!["wr_rA"]);
}

#[test]
fn if_without_alternate_adds_pre_if_origin_as_variable_id_when_no_prior_write() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_other_ctx(
        "cons",
        "root",
        AstType::IfStatement,
        "consequent",
        50,
    ));
    let mut ctx = base_builder_context(&ir);
    let op = write_op_at("rCons", 60, "cons");
    ctx.write_ops_by_variable.insert("v".to_string(), vec![op]);
    let got: HashSet<String> = read_origins("v", 100, "root", &ctx).into_iter().collect();
    let expected: HashSet<String> = ["wr_rCons", "n_v"].iter().map(|s| s.to_string()).collect();
    assert_eq!(got, expected);
}

#[test]
fn if_without_alternate_uses_last_pre_if_write_as_second_origin() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_other_ctx(
        "cons",
        "root",
        AstType::IfStatement,
        "consequent",
        50,
    ));
    let mut ctx = base_builder_context(&ir);
    let pre_if = write_op_at("rPre", 10, "root");
    let in_if = write_op_at("rCons", 60, "cons");
    ctx.write_ops_by_variable
        .insert("v".to_string(), vec![pre_if, in_if]);
    let got: HashSet<String> = read_origins("v", 100, "root", &ctx).into_iter().collect();
    let expected: HashSet<String> = ["wr_rCons", "wr_rPre"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(got, expected);
}

#[test]
fn if_else_with_writes_in_both_branches_yields_one_origin_per_branch() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_other_ctx(
        "cons",
        "root",
        AstType::IfStatement,
        "consequent",
        50,
    ));
    ir.scopes.push(scope_with_other_ctx(
        "alt",
        "root",
        AstType::IfStatement,
        "alternate",
        50,
    ));
    let mut ctx = base_builder_context(&ir);
    let op_cons = write_op_at("rCons", 60, "cons");
    let op_alt = write_op_at("rAlt", 70, "alt");
    ctx.write_ops_by_variable
        .insert("v".to_string(), vec![op_cons, op_alt]);
    let got: HashSet<String> = read_origins("v", 100, "root", &ctx).into_iter().collect();
    let expected: HashSet<String> = ["wr_rCons", "wr_rAlt"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(got, expected);
}

#[test]
fn switch_case_with_exits_function_is_excluded() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    let mut switch_scope = scope_with_upper("switch", "root");
    switch_scope.r#type = ScopeType::Switch;
    ir.scopes.push(switch_scope);
    let mut c1 = case_scope("c1", "switch", 100);
    c1.exits_function = true;
    ir.scopes.push(c1);
    ir.scopes.push(case_scope("c2", "switch", 100));
    let mut ctx = base_builder_context(&ir);
    let op_c1 = write_op_at("rC1", 110, "c1");
    let op_c2 = write_op_at("rC2", 120, "c2");
    let cases: Vec<&SerializedScope> = ir
        .scopes
        .iter()
        .filter(|s| s.id.value() == "c1" || s.id.value() == "c2")
        .collect();
    ctx.sorted_cases_by_container
        .insert("switch:switch:100".to_string(), cases);
    ctx.write_ops_by_variable
        .insert("v".to_string(), vec![op_c1, op_c2]);
    assert_eq!(read_origins("v", 200, "root", &ctx), vec!["wr_rC2"]);
}

#[test]
fn switch_case_fallsthrough_to_later_case_is_excluded() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    let mut switch_scope = scope_with_upper("switch", "root");
    switch_scope.r#type = ScopeType::Switch;
    ir.scopes.push(switch_scope);
    let mut c1 = case_scope("c1", "switch", 100);
    c1.falls_through = true;
    ir.scopes.push(c1);
    ir.scopes.push(case_scope("c2", "switch", 100));
    let mut ctx = base_builder_context(&ir);
    let cases: Vec<&SerializedScope> = ir
        .scopes
        .iter()
        .filter(|s| s.id.value() == "c1" || s.id.value() == "c2")
        .collect();
    ctx.sorted_cases_by_container
        .insert("switch:switch:100".to_string(), cases);
    let op_c1 = write_op_at("rC1", 110, "c1");
    let op_c2 = write_op_at("rC2", 120, "c2");
    ctx.write_ops_by_variable
        .insert("v".to_string(), vec![op_c1, op_c2]);
    assert_eq!(read_origins("v", 200, "root", &ctx), vec!["wr_rC2"]);
}

#[test]
fn try_catch_with_writes_in_both_branches_yields_one_origin_per_branch() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_other_ctx(
        "tryBlock",
        "root",
        AstType::TryStatement,
        "block",
        50,
    ));
    ir.scopes.push(scope_with_other_ctx(
        "catchBlock",
        "root",
        AstType::TryStatement,
        "handler",
        50,
    ));
    let mut ctx = base_builder_context(&ir);
    let op_try = write_op_at("rTry", 60, "tryBlock");
    let op_catch = write_op_at("rCatch", 70, "catchBlock");
    ctx.write_ops_by_variable
        .insert("v".to_string(), vec![op_try, op_catch]);
    let got: HashSet<String> = read_origins("v", 100, "root", &ctx).into_iter().collect();
    let expected: HashSet<String> = ["wr_rTry", "wr_rCatch"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(got, expected);
}

#[test]
fn try_without_catch_handler_adds_pre_try_origin_as_variable_id() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_other_ctx(
        "tryBlock",
        "root",
        AstType::TryStatement,
        "block",
        50,
    ));
    let mut ctx = base_builder_context(&ir);
    let op_try = write_op_at("rTry", 60, "tryBlock");
    ctx.write_ops_by_variable
        .insert("v".to_string(), vec![op_try]);
    let got: HashSet<String> = read_origins("v", 100, "root", &ctx).into_iter().collect();
    let expected: HashSet<String> = ["wr_rTry", "n_v"].iter().map(|s| s.to_string()).collect();
    assert_eq!(got, expected);
}

#[test]
fn read_inside_finally_sees_writes_from_try_and_catch() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_other_ctx(
        "tryBlock",
        "root",
        AstType::TryStatement,
        "block",
        50,
    ));
    ir.scopes.push(scope_with_other_ctx(
        "catchBlock",
        "root",
        AstType::TryStatement,
        "handler",
        50,
    ));
    ir.scopes.push(scope_with_other_ctx(
        "finallyBlock",
        "root",
        AstType::TryStatement,
        "finalizer",
        50,
    ));
    let mut ctx = base_builder_context(&ir);
    let op_try = write_op_at("rTry", 60, "tryBlock");
    let op_catch = write_op_at("rCatch", 70, "catchBlock");
    ctx.write_ops_by_variable
        .insert("v".to_string(), vec![op_try, op_catch]);
    let got: HashSet<String> = read_origins("v", 90, "finallyBlock", &ctx)
        .into_iter()
        .collect();
    let expected: HashSet<String> = ["wr_rTry", "wr_rCatch"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(got, expected);
}

#[test]
fn duplicate_origins_are_deduplicated() {
    let mut ir = empty_serialized_ir();
    ir.scopes.push(base_serialized_scope("root"));
    ir.scopes.push(scope_with_other_ctx(
        "cons",
        "root",
        AstType::IfStatement,
        "consequent",
        50,
    ));
    ir.scopes.push(scope_with_other_ctx(
        "alt",
        "root",
        AstType::IfStatement,
        "alternate",
        50,
    ));
    let mut ctx = base_builder_context(&ir);
    let op_cons = write_op_at("shared", 60, "cons");
    let op_alt = write_op_at("shared", 70, "alt");
    ctx.write_ops_by_variable
        .insert("v".to_string(), vec![op_cons, op_alt]);
    assert_eq!(read_origins("v", 100, "root", &ctx), vec!["wr_shared"]);
}
