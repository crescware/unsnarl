//! Mirrors `ts/src/visual-graph/builder/throw-use-node-id.ts`.

use super::sanitize::sanitize;

pub fn throw_use_node_id(ref_id: &str) -> String {
    format!("throw_use_{}", sanitize(ref_id))
}

#[cfg(test)]
#[path = "throw_use_node_id_test.rs"]
mod throw_use_node_id_test;
