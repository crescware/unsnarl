//! Resolve an `arguments` reference to the synthetic `arguments`
//! binding `variable_mapping` inserted into the nearest enclosing
//! non-arrow function scope.

use oxc_index::IndexVec;

use unsnarl_ir::ids::{ScopeId, VariableId};
use unsnarl_ir::scope::ScopeData;

pub(super) fn resolve_synthetic_arguments(
    scopes: &IndexVec<ScopeId, ScopeData>,
    from: ScopeId,
) -> Option<VariableId> {
    let mut cur = Some(from);
    while let Some(s) = cur {
        if let Some(&id) = scopes[s].set().get("arguments") {
            return Some(id);
        }
        cur = scopes[s].upper;
    }
    None
}
