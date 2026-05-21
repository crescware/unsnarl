//! Sibling tests for [`set_predecessor_of`]. Cases mirror
//! `ts/src/visual-graph/builder/set-predecessor-of.test.ts`.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

use super::set_predecessor_of;
use crate::builder::testing::{base_serialized_scope, base_write_op, scope_id};
use crate::builder::write_op::WriteOp;

fn write_op_with(ref_id: &str, scope_id: &str) -> WriteOp {
    let mut op = base_write_op();
    op.ref_id = ref_id.to_string();
    op.scope_id = scope_id.to_string();
    op
}

fn scope_map(scopes: &[SerializedScope]) -> HashMap<&str, &SerializedScope> {
    scopes.iter().map(|s| (s.id.value(), s)).collect()
}

#[test]
fn returns_variable_node_id_when_there_are_no_prior_ops() {
    let scopes = [base_serialized_scope("s")];
    let map = scope_map(&scopes);
    let op = write_op_with("r1", "s");
    assert_eq!(
        set_predecessor_of(&op, std::slice::from_ref(&op), &map),
        "n_v"
    );
}

#[test]
fn returns_prior_write_node_id_when_prior_in_the_same_scope() {
    let scopes = [base_serialized_scope("s")];
    let map = scope_map(&scopes);
    let prev = write_op_with("rPrev", "s");
    let op = write_op_with("rCur", "s");
    assert_eq!(
        set_predecessor_of(&op, &[prev, op.clone()], &map),
        "wr_rPrev"
    );
}

#[test]
fn returns_prior_write_node_id_when_prior_in_an_ancestor_scope() {
    let root = base_serialized_scope("root");
    let mut child = base_serialized_scope("child");
    child.upper = Some(scope_id("root"));
    let scopes = [root, child];
    let map = scope_map(&scopes);
    let prev = write_op_with("rPrev", "root");
    let op = write_op_with("rCur", "child");
    assert_eq!(
        set_predecessor_of(&op, &[prev, op.clone()], &map),
        "wr_rPrev"
    );
}

#[test]
fn returns_variable_node_id_when_prior_in_a_sibling_scope() {
    let root = base_serialized_scope("root");
    let mut a = base_serialized_scope("a");
    a.upper = Some(scope_id("root"));
    let mut b = base_serialized_scope("b");
    b.upper = Some(scope_id("root"));
    let scopes = [root, a, b];
    let map = scope_map(&scopes);
    let prev = write_op_with("rPrev", "a");
    let op = write_op_with("rCur", "b");
    assert_eq!(set_predecessor_of(&op, &[prev, op.clone()], &map), "n_v");
}

#[test]
fn returns_variable_node_id_when_op_is_missing_from_ops_list() {
    let scopes = [base_serialized_scope("s")];
    let map = scope_map(&scopes);
    let op = write_op_with("rCur", "s");
    assert_eq!(set_predecessor_of(&op, &[], &map), "n_v");
}
