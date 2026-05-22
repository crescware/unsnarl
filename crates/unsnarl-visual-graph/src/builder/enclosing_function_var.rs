use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

use super::find_enclosing_subgraph_scope::find_enclosing_subgraph_scope;

pub fn enclosing_function_var(
    scope_id: &str,
    scope_map: &HashMap<&str, &SerializedScope>,
    subgraph_owner_var: &HashMap<String, String>,
) -> Option<String> {
    let fn_scope_id = find_enclosing_subgraph_scope(scope_id, scope_map, subgraph_owner_var)?;
    subgraph_owner_var.get(&fn_scope_id).cloned()
}

#[cfg(test)]
#[path = "enclosing_function_var_test.rs"]
mod enclosing_function_var_test;
