use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

use super::branch_container_key::branch_container_key;

pub fn is_branch_scope(scope_id: &str, scope_map: &HashMap<&str, &SerializedScope>) -> bool {
    scope_map
        .get(scope_id)
        .copied()
        .is_some_and(|s| branch_container_key(s).is_some())
}

#[cfg(test)]
#[path = "is_branch_scope_test.rs"]
mod is_branch_scope_test;
