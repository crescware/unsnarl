//! Build-time state owned by the scope-builder.
//!
//! `ScopeBuilderState` is the Rust-side analogue of the TS
//! `ScopeManager` (`manager.ts`) plus the build-only mutable state that
//! TS keeps split across `ScopeImpl` / `VariableImpl` /
//! `ReferenceImpl` constructors. The contract types in `unsnarl-ir`
//! (`ScopeData`, `VariableData`, `ReferenceData`, `DefinitionData`) are
//! immutable from the analyzer's perspective once `finish` runs; while
//! the build is in progress we own them through `IrArena` so the
//! borrow checker permits the per-step mutations.
//!
//! `ScopeBuilderState` is reachable from outside the crate only via
//! callback arguments to `AnalysisVisitor`; the fields are therefore
//! `pub(crate)` to keep mutation pathways localised to this crate.

use oxc_index::IndexVec;

use unsnarl_ir::diagnostic::Diagnostic;
use unsnarl_ir::ids::{ScopeId, VariableId};
use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::scope::{DefinitionData, ScopeData, VariableData};
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::{DefinitionType, IrArena};

use crate::diagnostic_collector::DiagnosticCollector;

pub struct ScopeBuilderState {
    pub(crate) arena: IrArena,
    pub(crate) global_scope: ScopeId,
    pub(crate) diagnostics: DiagnosticCollector,
}

impl ScopeBuilderState {
    /// Build a fresh state seeded with the root scope.
    ///
    /// Mirrors `ScopeManager`'s constructor in `manager.ts`: a single
    /// `ScopeImpl` is created as the global / module scope with no
    /// `upper`, and the stack / `allScopes` are seeded with that scope.
    /// The Rust port differs only in that the stack of live scope IDs
    /// is kept as `Vec<ScopeId>` (added alongside `push_scope` in a
    /// later commit); the seed scope itself is recorded as
    /// `global_scope` here.
    pub(crate) fn new(root_kind: ScopeType, block: AstNode) -> Self {
        let mut arena = IrArena {
            scopes: IndexVec::new(),
            variables: IndexVec::new(),
            references: IndexVec::new(),
            definitions: IndexVec::new(),
        };
        let is_strict = matches!(root_kind, ScopeType::Module);
        let self_id_placeholder = ScopeId::from_usize(0);
        let global_scope = arena.scopes.push(ScopeData::new(
            root_kind,
            is_strict,
            None,
            Vec::new(),
            self_id_placeholder,
            block,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            false,
        ));
        arena.scopes[global_scope].variable_scope = global_scope;
        Self {
            arena,
            global_scope,
            diagnostics: DiagnosticCollector::new(),
        }
    }
}

/// Declare a variable inside a scope, mirroring `declareVariable` in
/// `ts/src/boundary/eslint-scope/declare/declare-variable.ts`.
///
/// Looks up the binding by name in the target scope's `set`; if no
/// variable with that name exists yet, allocates a fresh
/// `VariableData` in the arena, registers it on the scope (`set` +
/// `variables`), and proceeds. Then records `identifier` on the
/// variable's `identifiers` list (the TS `variable.identifiers.push`)
/// and pushes a `DefinitionData` carrying the same identifier plus
/// `def_type` / `def_node` / `parent` (the TS object literal pushed to
/// `variable.defs`).
///
/// `identifier` is consumed twice on the TS side (`variable.identifiers`
/// and `def.name`); Rust requires an explicit clone, which is the
/// derive-policy justification for `AstIdentifier: Clone`.
pub(crate) fn declare_variable(
    state: &mut ScopeBuilderState,
    scope: ScopeId,
    identifier: AstIdentifier,
    def_type: DefinitionType,
    def_node: AstNode,
    parent: Option<AstNode>,
) -> VariableId {
    let name = identifier.name().to_string();
    let variable_id = match state.arena.scopes[scope].set().get(&name).copied() {
        Some(id) => id,
        None => {
            let id = state.arena.variables.push(VariableData::new(
                name.clone(),
                scope,
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ));
            state.arena.scopes[scope].insert_into_set(name, id);
            state.arena.scopes[scope].variables.push(id);
            id
        }
    };
    state.arena.variables[variable_id]
        .identifiers
        .push(identifier.clone());
    let def_id = state.arena.definitions.push(DefinitionData {
        r#type: def_type,
        name: identifier,
        node: def_node,
        parent,
    });
    state.arena.variables[variable_id].defs.push(def_id);
    variable_id
}

/// Drain the build state into a `(IrArena, ScopeId, Vec<Diagnostic>)`
/// triple.
///
/// The TS side has no separate `finish` step (`manager.globalScope`
/// is returned directly inside `analyze`, and `visitor.onDiagnostic`
/// is fed from `diagnostics.list()` at the very end). The Rust side
/// keeps the build-only state behind `ScopeBuilderState` and unwraps
/// it here so the immutable arena + the collected diagnostics are the
/// only things that leave the boundary crate.
pub(crate) fn finish(state: ScopeBuilderState) -> (IrArena, ScopeId, Vec<Diagnostic>) {
    (
        state.arena,
        state.global_scope,
        state.diagnostics.into_list(),
    )
}
