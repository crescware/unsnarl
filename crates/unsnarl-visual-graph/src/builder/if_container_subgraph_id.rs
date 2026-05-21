//! Mirrors `ts/src/visual-graph/builder/if-container-subgraph-id.ts`.

use super::sanitize::sanitize;

pub fn if_container_subgraph_id(parent_scope_id: &str, offset: u32) -> String {
    format!("cont_if_{}_{}", sanitize(parent_scope_id), offset)
}

#[cfg(test)]
#[path = "if_container_subgraph_id_test.rs"]
mod if_container_subgraph_id_test;
