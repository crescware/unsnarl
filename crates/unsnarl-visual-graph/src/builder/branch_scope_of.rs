use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

use super::is_branch_scope::is_branch_scope;

pub fn branch_scope_of(
    scope_id: &str,
    scope_map: &HashMap<&str, &SerializedScope>,
) -> Option<String> {
    let mut cur = scope_map.get(scope_id).copied();
    while let Some(scope) = cur {
        if is_branch_scope(scope.id.value(), scope_map) {
            return Some(scope.id.value().to_string());
        }
        let upper = scope.upper.as_ref()?;
        cur = scope_map.get(upper.value()).copied();
    }
    None
}

#[cfg(test)]
#[path = "branch_scope_of_test.rs"]
mod branch_scope_of_test;
