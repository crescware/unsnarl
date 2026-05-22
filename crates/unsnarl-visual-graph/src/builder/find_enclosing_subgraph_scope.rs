use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

pub fn find_enclosing_subgraph_scope(
    scope_id: &str,
    scope_map: &HashMap<&str, &SerializedScope>,
    subgraph_owner_var: &HashMap<String, String>,
) -> Option<String> {
    let mut cur = scope_map.get(scope_id).copied();
    while let Some(scope) = cur {
        if subgraph_owner_var.contains_key(scope.id.value()) {
            return Some(scope.id.value().to_string());
        }
        let upper = scope.upper.as_ref()?;
        cur = scope_map.get(upper.value()).copied();
    }
    None
}

#[cfg(test)]
#[path = "find_enclosing_subgraph_scope_test.rs"]
mod find_enclosing_subgraph_scope_test;
