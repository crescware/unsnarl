//! Mirrors `ts/src/visual-graph/builder/loop-test-node-id.ts`.

use super::sanitize::sanitize;

pub fn while_test_node_id(parent_scope_id: &str, offset: u32) -> String {
    format!("while_test_{}_{}", sanitize(parent_scope_id), offset)
}

pub fn do_while_test_node_id(parent_scope_id: &str, offset: u32) -> String {
    format!("do_while_test_{}_{}", sanitize(parent_scope_id), offset)
}

pub fn for_test_node_id(parent_scope_id: &str, offset: u32) -> String {
    format!("for_test_{}_{}", sanitize(parent_scope_id), offset)
}
