//! Synthesise the implicit per-function `arguments` binding.

use oxc_index::IndexVec;

use unsnarl_ir::ids::{ScopeId, VariableId};
use unsnarl_ir::scope::{ScopeData, VariableData};

pub(super) fn push_implicit_arguments(
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    scope: ScopeId,
) {
    let name = "arguments";
    if scopes[scope].set().contains_key(name) {
        return;
    }
    let var_id = variables.push(VariableData::new(
        name.to_string(),
        scope,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ));
    scopes[scope].insert_into_set(name.to_string(), var_id);
    scopes[scope].variables.push(var_id);
}
