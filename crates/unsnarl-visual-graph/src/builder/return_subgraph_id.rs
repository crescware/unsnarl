use super::sanitize::sanitize;

pub fn return_subgraph_id(var_id: &str, container_key: &str) -> String {
    format!("s_return_{}_{}", sanitize(var_id), sanitize(container_key))
}

#[cfg(test)]
#[path = "return_subgraph_id_test.rs"]
mod return_subgraph_id_test;
