//! Reference resolution against the live scope chain plus implicit
//! global fallback.
//!
//! The `resolved` field is updated through the arena
//! (`arena.references[id].resolved = ...`).
//!
//! When the identifier doesn't resolve in any reachable scope, we
//! create an `ImplicitGlobalVariable` on the global scope so the
//! downstream IR has a binding to point at. This matches
//! eslint-scope's `__defineImplicitVariable` shape.

use unsnarl_ir::ids::{ReferenceId, ScopeId, VariableId};
use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::reference::reference_flags::ReferenceFlagBits;
use unsnarl_ir::reference::ReferenceData;
use unsnarl_ir::scope::{DefinitionData, VariableData};
use unsnarl_ir::{DefinitionType, IrArena};

use crate::state::ScopeBuilderState;

pub(crate) fn bind_reference(
    state: &mut ScopeBuilderState,
    scope: ScopeId,
    identifier: AstIdentifier,
    flags: ReferenceFlagBits,
    init: bool,
) -> ReferenceId {
    let name = identifier.name().to_string();
    let ref_id = state.arena.references.push(ReferenceData {
        identifier,
        from: scope,
        resolved: None,
        init,
        flags,
    });
    state.arena.scopes[scope].references.push(ref_id);

    let resolved = resolve_in_scope_chain(&state.arena, scope, &name);
    if let Some(target) = resolved {
        state.arena.references[ref_id].resolved = Some(target);
        state.arena.variables[target].references.push(ref_id);
        return ref_id;
    }
    let target = declare_implicit_global(state, &name, ref_id);
    state.arena.references[ref_id].resolved = Some(target);
    state.arena.variables[target].references.push(ref_id);

    // Implicit-global path only: the reference did not resolve in any
    // reachable scope, so every scope between `scope` and the global
    // scope (exclusive) "passes the reference through" up to the
    // global where the implicit binding lives. The global scope is
    // appended last so the order in `globalScope.through` matches
    // the eslint-scope contract.
    let global = state.global_scope;
    let mut cur = Some(scope);
    while let Some(s) = cur {
        if s == global {
            break;
        }
        state.arena.scopes[s].through.push(ref_id);
        cur = state.arena.scopes[s].upper;
    }
    state.arena.scopes[global].through.push(ref_id);
    ref_id
}

pub fn resolve_in_scope_chain(arena: &IrArena, scope: ScopeId, name: &str) -> Option<VariableId> {
    let mut cur = Some(scope);
    while let Some(s) = cur {
        if let Some(&id) = arena.scopes[s].set().get(name) {
            return Some(id);
        }
        cur = arena.scopes[s].upper;
    }
    None
}

fn declare_implicit_global(
    state: &mut ScopeBuilderState,
    name: &str,
    ref_id: ReferenceId,
) -> VariableId {
    // Invariant: `bind_reference` calls this only when
    // `resolve_in_scope_chain` returned `None`, and that walk always
    // reaches the global scope (every scope's `upper` chain terminates
    // at `global_scope`). So `global.set()` is guaranteed not to
    // contain `name` here; no early-return lookup is needed.
    let global = state.global_scope;
    let ident_copy = state.arena.references[ref_id].identifier.clone();
    let var_id = state.arena.variables.push(VariableData::new(
        name.to_string(),
        global,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ));
    state.arena.scopes[global].insert_into_set(name.to_string(), var_id);
    state.arena.scopes[global].variables.push(var_id);
    let ident_node = AstNode {
        r#type: ident_copy.r#type.clone(),
        span: ident_copy.span,
    };
    state.arena.variables[var_id]
        .identifiers
        .push(ident_copy.clone());
    let def_id = state.arena.definitions.push(DefinitionData {
        r#type: DefinitionType::ImplicitGlobalVariable,
        name: ident_copy,
        node: ident_node,
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    });
    state.arena.variables[var_id].defs.push(def_id);
    var_id
}
