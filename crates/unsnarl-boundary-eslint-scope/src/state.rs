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
use unsnarl_oxc_parity::VariableDeclarationKind;

use crate::diagnostic_collector::DiagnosticCollector;

pub struct ScopeBuilderState {
    pub(crate) arena: IrArena,
    pub(crate) global_scope: ScopeId,
    pub(crate) stack: Vec<ScopeId>,
    pub(crate) diagnostics: DiagnosticCollector,
}

impl ScopeBuilderState {
    /// Build a fresh state seeded with the root scope.
    ///
    /// Mirrors `ScopeManager`'s constructor in `manager.ts`: a single
    /// `ScopeImpl` is created as the global / module scope with no
    /// `upper`, and `stack` / `allScopes` are seeded with that scope.
    /// The Rust port collapses `allScopes` into the arena
    /// (`IndexVec<ScopeId, ScopeData>` already enumerates every
    /// allocated scope, so a second container would duplicate the
    /// information) and keeps a `Vec<ScopeId>` for `stack`.
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
            stack: vec![global_scope],
            diagnostics: DiagnosticCollector::new(),
        }
    }
}

/// The currently active scope on the build stack.
///
/// Mirrors `ScopeManager#current` in `manager.ts`. Returns the top of
/// `stack`; panics if the stack is empty, which would mean the root
/// scope was somehow popped — a builder bug, not user input.
pub(crate) fn current_scope(state: &ScopeBuilderState) -> ScopeId {
    *state
        .stack
        .last()
        .expect("ScopeBuilderState.stack must not be empty")
}

/// Allocate a new scope, attach it to the current scope as a child,
/// and push it onto the stack.
///
/// Mirrors `ScopeManager#push` in `manager.ts` together with the side
/// effects of `ScopeImpl`'s constructor: the parent gains an entry in
/// its `child_scopes`, the new scope inherits the parent's
/// `is_strict`, and `variable_scope` is fixed to either the new scope
/// (for `Function` / `Module` / `Global`) or the parent's
/// `variable_scope`.
///
/// Returns the freshly allocated `ScopeId`, which is also now the
/// stack top (so a follow-up [`current_scope`] returns it).
pub(crate) fn push_scope(state: &mut ScopeBuilderState, ty: ScopeType, block: AstNode) -> ScopeId {
    let parent = current_scope(state);
    let parent_is_strict = state.arena.scopes[parent].is_strict;
    let parent_variable_scope = state.arena.scopes[parent].variable_scope;
    let is_self_variable_scope = matches!(
        ty,
        ScopeType::Function | ScopeType::Module | ScopeType::Global
    );
    let placeholder = ScopeId::from_usize(0);
    let new_id = state.arena.scopes.push(ScopeData::new(
        ty,
        parent_is_strict,
        Some(parent),
        Vec::new(),
        placeholder,
        block,
        Vec::new(),
        Vec::new(),
        Vec::new(),
        false,
    ));
    let variable_scope = if is_self_variable_scope {
        new_id
    } else {
        parent_variable_scope
    };
    state.arena.scopes[new_id].variable_scope = variable_scope;
    state.arena.scopes[parent].child_scopes.push(new_id);
    state.stack.push(new_id);
    new_id
}

/// Pop the current scope off the build stack.
///
/// Mirrors `ScopeManager#pop` in `manager.ts`. Panics if popping
/// would leave the stack empty — the root scope must remain
/// throughout the build.
pub(crate) fn pop_scope(state: &mut ScopeBuilderState) {
    assert!(
        state.stack.len() > 1,
        "cannot pop the root scope from the build stack"
    );
    state.stack.pop();
}

/// Declare a variable inside a scope.
///
/// Looks up the binding by name in the target scope's `set`; if no
/// variable with that name exists yet, allocates a fresh
/// `VariableData` in the arena, registers it on the scope (`set` +
/// `variables`), and proceeds. Then records `identifier` on the
/// variable's `identifiers` list and pushes a `DefinitionData`
/// carrying the same identifier plus `def_type` / `def_node` /
/// `parent`.
///
/// `identifier` is consumed twice (once in `variable.identifiers`,
/// once in `def.name`); the explicit clone is the derive-policy
/// justification for `AstIdentifier: Clone`.
/// Per-definition extras the serializer reads but the
/// `(node, parent)` materialisation drops. Defaults to all-`None` so
/// callsites that build simple defs (function names, class names,
/// parameters, catch-clause bindings, implicit globals) need not
/// mention these fields explicitly.
///
/// The fields mirror the same-named `DefinitionData` slots; see
/// `unsnarl_ir::scope::definition` for the variant-by-variant
/// invariants.
#[derive(Default)]
pub(crate) struct DefinitionExtras {
    pub init: Option<AstNode>,
    pub declaration_kind: Option<VariableDeclarationKind>,
    pub import_source: Option<String>,
    pub imported_name: Option<String>,
}

pub(crate) fn declare_variable(
    state: &mut ScopeBuilderState,
    scope: ScopeId,
    identifier: AstIdentifier,
    def_type: DefinitionType,
    def_node: AstNode,
    parent: Option<AstNode>,
) -> VariableId {
    declare_variable_with_extras(
        state,
        scope,
        identifier,
        def_type,
        def_node,
        parent,
        DefinitionExtras::default(),
    )
}

pub(crate) fn declare_variable_with_extras(
    state: &mut ScopeBuilderState,
    scope: ScopeId,
    identifier: AstIdentifier,
    def_type: DefinitionType,
    def_node: AstNode,
    parent: Option<AstNode>,
    extras: DefinitionExtras,
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
        init: extras.init,
        declaration_kind: extras.declaration_kind,
        import_source: extras.import_source,
        imported_name: extras.imported_name,
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

#[cfg(test)]
#[path = "state_test.rs"]
mod state_test;
