use super::sanitize::sanitize;

pub fn if_test_node_id(parent_scope_id: &str, offset: u32) -> String {
    format!("if_test_{}_{}", sanitize(parent_scope_id), offset)
}

#[cfg(test)]
#[path = "if_test_node_id_test.rs"]
mod if_test_node_id_test;
