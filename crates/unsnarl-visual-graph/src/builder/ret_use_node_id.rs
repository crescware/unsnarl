//! Mirrors `ts/src/visual-graph/builder/ret-use-node-id.ts`.

use super::sanitize::sanitize;

pub fn ret_use_node_id(ref_id: &str) -> String {
    format!("ret_use_{}", sanitize(ref_id))
}

#[cfg(test)]
#[path = "ret_use_node_id_test.rs"]
mod ret_use_node_id_test;
