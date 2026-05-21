//! Sibling tests for [`owner_target_id`]. Cases mirror
//! `ts/src/visual-graph/builder/owner-target-id.test.ts`.

use std::collections::HashMap;

use super::owner_target_id;
use crate::builder::testing::base_write_op;

fn ops_by_owner() -> HashMap<String, Vec<crate::builder::write_op::WriteOp>> {
    let mut op1 = base_write_op();
    op1.ref_id = "r1".to_string();
    op1.var_id = "owner".to_string();
    op1.offset = 10;
    let mut op2 = base_write_op();
    op2.ref_id = "r2".to_string();
    op2.var_id = "owner".to_string();
    op2.offset = 20;
    let mut map = HashMap::new();
    map.insert("owner".to_string(), vec![op1, op2]);
    map
}

#[test]
fn before_any_write_returns_the_owner_node_id() {
    let map = ops_by_owner();
    assert_eq!(owner_target_id("owner", 5, &map), "n_owner");
}

#[test]
fn at_first_write_offset_returns_its_write_op_node() {
    // Inclusive: offset == first write offset still picks that write.
    let map = ops_by_owner();
    assert_eq!(owner_target_id("owner", 10, &map), "wr_r1");
}

#[test]
fn between_writes_returns_first_write_op_node() {
    let map = ops_by_owner();
    assert_eq!(owner_target_id("owner", 15, &map), "wr_r1");
}

#[test]
fn at_second_write_offset_returns_its_write_op_node() {
    let map = ops_by_owner();
    assert_eq!(owner_target_id("owner", 20, &map), "wr_r2");
}

#[test]
fn after_last_write_returns_last_write_op_node() {
    let map = ops_by_owner();
    assert_eq!(owner_target_id("owner", 999, &map), "wr_r2");
}

#[test]
fn owner_without_recorded_writes_falls_back_to_node_id() {
    let map = ops_by_owner();
    assert_eq!(owner_target_id("missing", 100, &map), "n_missing");
}
