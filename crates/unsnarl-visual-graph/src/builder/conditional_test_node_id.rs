use super::sanitize::sanitize;

pub fn conditional_test_node_id(parent_scope_id: &str, offset: u32) -> String {
    format!("ternary_test_{}_{}", sanitize(parent_scope_id), offset)
}

#[cfg(test)]
#[path = "conditional_test_node_id_test.rs"]
mod conditional_test_node_id_test;
