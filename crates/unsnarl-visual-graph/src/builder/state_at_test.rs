//! Sibling tests for [`state_at`].

use std::collections::HashMap;

use super::state_at;
use crate::builder::builder_fixtures::base_write_op;
use crate::builder::write_op::WriteOp;

fn ops_by_v() -> HashMap<String, Vec<WriteOp>> {
    let mut ops = Vec::new();
    for (ref_id, offset) in [("r1", 10), ("r2", 20), ("r3", 30)] {
        let mut op = base_write_op();
        op.ref_id = ref_id.to_string();
        op.var_id = "v".to_string();
        op.offset = offset;
        ops.push(op);
    }
    let mut map = HashMap::new();
    map.insert("v".to_string(), ops);
    map
}

#[test]
fn before_any_write_returns_the_variable_id() {
    let map = ops_by_v();
    assert_eq!(state_at("v", 5, &map), "v");
}

#[test]
fn exactly_at_first_write_offset_still_pre_write() {
    // The `state_at` predicate uses strict `<`, so offset == first
    // write offset still reports the pre-write state (variable id).
    let map = ops_by_v();
    assert_eq!(state_at("v", 10, &map), "v");
}

#[test]
fn between_first_and_second_write_returns_first_ref_id() {
    let map = ops_by_v();
    assert_eq!(state_at("v", 15, &map), "r1");
}

#[test]
fn between_second_and_third_returns_second_ref_id() {
    let map = ops_by_v();
    assert_eq!(state_at("v", 25, &map), "r2");
}

#[test]
fn after_last_write_returns_last_ref_id() {
    let map = ops_by_v();
    assert_eq!(state_at("v", 999, &map), "r3");
}

#[test]
fn variable_with_no_recorded_writes_returns_the_variable_id() {
    let map = ops_by_v();
    assert_eq!(state_at("missing", 100, &map), "missing");
}
