//! Mirrors `ts/src/visual-graph/builder/throw-subgraph-id.ts`.

use super::sanitize::sanitize;

pub fn throw_subgraph_id(var_id: &str, container_key: &str) -> String {
    format!("s_throw_{}_{}", sanitize(var_id), sanitize(container_key))
}

#[cfg(test)]
#[path = "throw_subgraph_id_test.rs"]
mod throw_subgraph_id_test;
