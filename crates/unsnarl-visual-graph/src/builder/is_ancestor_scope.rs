//! Mirrors `ts/src/visual-graph/builder/is-ancestor-scope.ts`.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedScope;

pub fn is_ancestor_scope(
    ancestor_id: &str,
    descendant_id: &str,
    scope_map: &HashMap<&str, &SerializedScope>,
) -> bool {
    let mut cur = scope_map.get(descendant_id).copied();
    while let Some(scope) = cur {
        if scope.id.value() == ancestor_id {
            return true;
        }
        let Some(upper) = scope.upper.as_ref() else {
            return false;
        };
        cur = scope_map.get(upper.value()).copied();
    }
    false
}
