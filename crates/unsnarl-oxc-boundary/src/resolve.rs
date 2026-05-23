//! Scope-chain lookup utility used by `unsnarl-analyzer` to map a
//! `BindingPattern` identifier name back to the live `VariableId` it
//! resolves to.
//!
//! [`crate::oxc_semantic_adapter`] handles scope / reference resolution
//! at build time, but downstream consumers
//! (`unsnarl_analyzer::owner::all_binding_variables`) still need the
//! same `(arena, scope, name) → Option<VariableId>` projection at
//! analysis time. Keeping the helper in the boundary crate avoids
//! pulling the IR walk into `unsnarl-analyzer` and preserves the
//! single source of truth for the chain semantics.

use unsnarl_ir::ids::{ScopeId, VariableId};
use unsnarl_ir::IrArena;

/// Walk `scope`'s `upper` chain looking for a binding named `name`.
/// Returns the matching `VariableId` on the first scope whose
/// `ScopeData::set` contains the name, or `None` when the chain
/// terminates without a match (typically meaning the reference is an
/// implicit global that the adapter has already synthesised on the
/// root scope's `set`).
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
