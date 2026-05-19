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

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope::ScopeData;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::IrArena;

pub struct ScopeBuilderState {
    pub(crate) arena: IrArena,
    pub(crate) global_scope: ScopeId,
}

impl ScopeBuilderState {
    /// Build a fresh state seeded with the root scope.
    ///
    /// Mirrors `ScopeManager`'s constructor in `manager.ts`: a single
    /// `ScopeImpl` is created as the global / module scope with no
    /// `upper`, and the stack / `allScopes` are seeded with that scope.
    /// The Rust port differs only in that the stack of live scope IDs
    /// is kept as `Vec<ScopeId>` (added in Step 9.4 alongside
    /// `push_scope`); the seed scope itself is recorded as
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
        }
    }
}

/// Drain the build state into a `(IrArena, ScopeId)` pair.
///
/// The TS side has no separate `finish` step: `manager.globalScope`
/// is returned directly inside `analyze`. The Rust side keeps the
/// build-only state behind `ScopeBuilderState` and unwraps it here
/// once the walker is done, so the immutable `IrArena` is the only
/// thing that leaves the boundary crate.
pub(crate) fn finish(state: ScopeBuilderState) -> (IrArena, ScopeId) {
    (state.arena, state.global_scope)
}
