//! Mirrors `ts/src/visual-graph/builder/if-test-node-id.ts`.

use super::sanitize::sanitize;

pub fn if_test_node_id(parent_scope_id: &str, offset: u32) -> String {
    format!("if_test_{}_{}", sanitize(parent_scope_id), offset)
}
