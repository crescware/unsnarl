use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

pub fn find_enclosing_subgraph_scope(
    scope_id: &str,
    scope_map: &HashMap<&str, &SerializedScope>,
    subgraph_owner_var: &HashMap<String, String>,
) -> Option<String> {
    find_enclosing_subgraph_scope_borrowed(scope_id, scope_map, subgraph_owner_var)
        .map(str::to_string)
}

/// Borrow-returning twin of [`find_enclosing_subgraph_scope`]. The
/// emitted scope id lives inside `scope_map`, so hot callers can
/// re-look-up the matching `subgraph_owner_var` entry without
/// allocating a fresh `String` per reference.
pub fn find_enclosing_subgraph_scope_borrowed<'a>(
    scope_id: &str,
    scope_map: &'a HashMap<&'a str, &'a SerializedScope>,
    subgraph_owner_var: &HashMap<String, String>,
) -> Option<&'a str> {
    let mut cur = scope_map.get(scope_id).copied();
    while let Some(scope) = cur {
        // The scope ids stored in `scope_map`'s values come from the
        // same IR arena the map's keys do, so the borrow that
        // `scope.id.value()` hands back is tied to `'a` -- the
        // function can return it without re-allocating.
        if subgraph_owner_var.contains_key(scope.id.value()) {
            return scope_map.get_key_value(scope.id.value()).map(|(k, _)| *k);
        }
        let upper = scope.upper.as_ref()?;
        cur = scope_map.get(upper.value()).copied();
    }
    None
}

#[cfg(test)]
#[path = "find_enclosing_subgraph_scope_test.rs"]
mod find_enclosing_subgraph_scope_test;
