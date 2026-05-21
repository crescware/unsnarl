//! Sibling tests for [`last_write_op_in_scope_before`]. Cases
//! mirror `ts/src/visual-graph/builder/last-write-op-in-scope-before.test.ts`.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

use super::last_write_op_in_scope_before;
use crate::builder::testing::{base_serialized_scope, base_write_op, scope_id};
use crate::builder::write_op::WriteOp;

fn scopes() -> [SerializedScope; 4] {
    let root = base_serialized_scope("root");
    let mut child = base_serialized_scope("child");
    child.upper = Some(scope_id("root"));
    let mut sibling = base_serialized_scope("sibling");
    sibling.upper = Some(scope_id("root"));
    let mut grandchild = base_serialized_scope("grandchild");
    grandchild.upper = Some(scope_id("child"));
    [root, child, sibling, grandchild]
}

fn scope_map(scopes: &[SerializedScope]) -> HashMap<&str, &SerializedScope> {
    scopes.iter().map(|s| (s.id.value(), s)).collect()
}

fn ops() -> Vec<WriteOp> {
    let mut out = Vec::new();
    for (ref_id, offset, scope_id) in [
        ("rRoot", 5u32, "root"),
        ("rChild", 10, "child"),
        ("rGrand", 15, "grandchild"),
        ("rSib", 20, "sibling"),
        ("rRoot2", 25, "root"),
    ] {
        let mut op = base_write_op();
        op.ref_id = ref_id.to_string();
        op.offset = offset;
        op.scope_id = scope_id.to_string();
        out.push(op);
    }
    out
}

fn by_var() -> HashMap<String, Vec<WriteOp>> {
    let mut map = HashMap::new();
    map.insert("v".to_string(), ops());
    map
}

#[test]
fn root_scope_sees_ops_from_itself_and_all_descendants() {
    let scopes = scopes();
    let map = scope_map(&scopes);
    let by_var = by_var();
    let result = last_write_op_in_scope_before("v", "root", 100, &by_var, &map);
    assert_eq!(result.map(|o| o.ref_id.as_str()), Some("rRoot2"));
}

#[test]
fn child_scope_sees_its_own_and_grandchild_ops_but_not_root_or_sibling() {
    let scopes = scopes();
    let map = scope_map(&scopes);
    let by_var = by_var();
    let result = last_write_op_in_scope_before("v", "child", 100, &by_var, &map);
    assert_eq!(result.map(|o| o.ref_id.as_str()), Some("rGrand"));
}

#[test]
fn child_scope_before_grandchild_write_picks_the_child_write() {
    let scopes = scopes();
    let map = scope_map(&scopes);
    let by_var = by_var();
    let result = last_write_op_in_scope_before("v", "child", 12, &by_var, &map);
    assert_eq!(result.map(|o| o.ref_id.as_str()), Some("rChild"));
}

#[test]
fn sibling_scope_sees_only_its_own_writes() {
    let scopes = scopes();
    let map = scope_map(&scopes);
    let by_var = by_var();
    let result = last_write_op_in_scope_before("v", "sibling", 100, &by_var, &map);
    assert_eq!(result.map(|o| o.ref_id.as_str()), Some("rSib"));
}

#[test]
fn ops_at_or_after_the_offset_are_excluded() {
    let scopes = scopes();
    let map = scope_map(&scopes);
    let by_var = by_var();
    let result = last_write_op_in_scope_before("v", "root", 5, &by_var, &map);
    assert!(result.is_none());
}

#[test]
fn no_candidates_returns_none() {
    let scopes = scopes();
    let map = scope_map(&scopes);
    let by_var = by_var();
    let result = last_write_op_in_scope_before("v", "root", 0, &by_var, &map);
    assert!(result.is_none());
}

#[test]
fn variable_with_no_recorded_writes_returns_none() {
    let scopes = scopes();
    let map = scope_map(&scopes);
    let by_var = by_var();
    let result = last_write_op_in_scope_before("missing", "root", 100, &by_var, &map);
    assert!(result.is_none());
}
