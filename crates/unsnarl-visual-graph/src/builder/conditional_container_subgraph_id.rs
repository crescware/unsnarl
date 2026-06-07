use super::sanitize::sanitize;

pub fn conditional_container_subgraph_id(parent_scope_id: &str, offset: u32) -> String {
    format!("cont_ternary_{}_{}", sanitize(parent_scope_id), offset)
}

#[cfg(test)]
#[path = "conditional_container_subgraph_id_test.rs"]
mod conditional_container_subgraph_id_test;
