//! Mirrors `ts/src/visual-graph/builder/write-op-node-id.ts`.

use super::sanitize::sanitize;

pub fn write_op_node_id(ref_id: &str) -> String {
    format!("wr_{}", sanitize(ref_id))
}

#[cfg(test)]
#[path = "write_op_node_id_test.rs"]
mod write_op_node_id_test;
