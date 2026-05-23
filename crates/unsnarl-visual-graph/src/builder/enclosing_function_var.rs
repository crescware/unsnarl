use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

use super::find_enclosing_subgraph_scope::{
    find_enclosing_subgraph_scope, find_enclosing_subgraph_scope_borrowed,
};
use unsnarl_instrumentation::TimingScope;

pub fn enclosing_function_var(
    scope_id: &str,
    scope_map: &HashMap<&str, &SerializedScope>,
    subgraph_owner_var: &HashMap<String, String>,
) -> Option<String> {
    let _t = TimingScope::start("enclosing_function_var");
    let fn_scope_id = find_enclosing_subgraph_scope(scope_id, scope_map, subgraph_owner_var)?;
    subgraph_owner_var.get(&fn_scope_id).cloned()
}

/// Borrow-returning twin of [`enclosing_function_var`]. The returned
/// `&str` lives inside `subgraph_owner_var`, which `emit_reference_edges`
/// keeps alive for the entire reference loop, so hot callers can pass
/// it straight to `find_host_subgraph` without allocating a new
/// `String` per iteration (~52k calls on `mermaid.js`).
pub fn enclosing_function_var_borrowed<'a>(
    scope_id: &str,
    scope_map: &'a HashMap<&'a str, &'a SerializedScope>,
    subgraph_owner_var: &'a HashMap<String, String>,
) -> Option<&'a str> {
    let _t = TimingScope::start("enclosing_function_var");
    let fn_scope_id =
        find_enclosing_subgraph_scope_borrowed(scope_id, scope_map, subgraph_owner_var)?;
    subgraph_owner_var.get(fn_scope_id).map(String::as_str)
}

#[cfg(test)]
#[path = "enclosing_function_var_test.rs"]
mod enclosing_function_var_test;
