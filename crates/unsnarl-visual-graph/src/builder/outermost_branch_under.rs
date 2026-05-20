//! Mirrors `ts/src/visual-graph/builder/outermost-branch-under.ts`.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

use super::is_branch_scope::is_branch_scope;

pub fn outermost_branch_under(
    branch_id: &str,
    scope_id: &str,
    scope_map: &HashMap<&str, &SerializedScope>,
) -> Option<String> {
    if scope_id == branch_id {
        return None;
    }
    let mut result: Option<String> = None;
    let mut cur = scope_map.get(scope_id).copied();
    while let Some(scope) = cur {
        if scope.id.value() == branch_id {
            break;
        }
        if is_branch_scope(scope.id.value(), scope_map) {
            result = Some(scope.id.value().to_string());
        }
        let upper = scope.upper.as_ref()?;
        cur = scope_map.get(upper.value()).copied();
    }
    let cur = cur?;
    if cur.id.value() != branch_id {
        return None;
    }
    result
}
