//! Implicit-global bookkeeping: allocate (or reuse) the root-scope
//! `ImplicitGlobalVariable` row for an unresolved name, and push a
//! reference through its scope chain up to the global scope.

use std::collections::HashMap;

use oxc_index::IndexVec;

use unsnarl_ir::ids::{DefinitionId, ReferenceId, ScopeId, VariableId};
use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::scope::{DefinitionData, ScopeData, VariableData};
use unsnarl_ir::DefinitionType;

/// Outcome of [`ensure_implicit_global`]: the resolved `VariableId`
/// plus whether this call freshly created the implicit-global row.
/// Used by the caller to decide whether to push the reference through
/// the `scope.through` chain — the parity baseline only records a
/// through entry on the first unresolved encounter of a name (because
/// every subsequent occurrence resolves against the freshly-created
/// implicit-global row in the root scope and takes the resolved
/// short-circuit path instead).
pub(super) struct ImplicitGlobalLookup {
    pub(super) var_id: VariableId,
    pub(super) newly_created: bool,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn ensure_implicit_global(
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    definitions: &mut IndexVec<DefinitionId, DefinitionData>,
    implicit_globals: &mut HashMap<String, VariableId>,
    root: ScopeId,
    name: &str,
    first_occurrence: &AstIdentifier,
) -> ImplicitGlobalLookup {
    if let Some(&id) = implicit_globals.get(name) {
        return ImplicitGlobalLookup {
            var_id: id,
            newly_created: false,
        };
    }
    let var_id = variables.push(VariableData::new(
        name.to_string(),
        root,
        vec![first_occurrence.clone()],
        Vec::new(),
        Vec::new(),
    ));
    scopes[root].insert_into_set(name.to_string(), var_id);
    scopes[root].variables.push(var_id);
    let node = AstNode::new(first_occurrence.r#type.clone(), first_occurrence.span);
    let def_id = definitions.push(DefinitionData {
        r#type: DefinitionType::ImplicitGlobalVariable,
        name: first_occurrence.clone(),
        node,
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    });
    variables[var_id].defs.push(def_id);
    implicit_globals.insert(name.to_string(), var_id);
    ImplicitGlobalLookup {
        var_id,
        newly_created: true,
    }
}

pub(super) fn push_through_chain(
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    from: ScopeId,
    root: ScopeId,
    ref_id: ReferenceId,
) {
    let mut cur = Some(from);
    while let Some(s) = cur {
        if s == root {
            break;
        }
        scopes[s].through.push(ref_id);
        cur = scopes[s].upper;
    }
    scopes[root].through.push(ref_id);
}
