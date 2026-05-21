//! Mirrors `ts/src/visual-graph/builder/node-id.ts`.

use super::sanitize::sanitize;

pub fn node_id(var_id: &str) -> String {
    format!("n_{}", sanitize(var_id))
}

#[cfg(test)]
#[path = "node_id_test.rs"]
mod node_id_test;
