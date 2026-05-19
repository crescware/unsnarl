//! Register the synthetic `arguments` binding inside a function scope.
//!
//! Mirrors `declareImplicitArguments` in
//! `ts/src/boundary/eslint-scope/declare/declare-implicit-arguments.ts`.
//! The TS version pushes a fresh `VariableImpl` with no identifiers
//! and no defs — the ES spec's `CreateUnmappedArgumentsObject` /
//! `CreateMappedArgumentsObject` shape — and the Rust port does the
//! same through the arena. Arrow functions inherit `arguments` from
//! the enclosing function scope and must not call this helper.

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::scope::VariableData;

use crate::state::ScopeBuilderState;

pub(crate) fn declare_implicit_arguments(state: &mut ScopeBuilderState, scope: ScopeId) {
    let name = "arguments";
    if state.arena.scopes[scope].set().contains_key(name) {
        return;
    }
    let id = state.arena.variables.push(VariableData::new(
        name.to_string(),
        scope,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ));
    state.arena.scopes[scope].insert_into_set(name.to_string(), id);
    state.arena.scopes[scope].variables.push(id);
}
