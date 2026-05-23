//! Result type returned by [`crate::analyze::analyze`].
//!
//! Exposes the arena explicitly so downstream crates can walk
//! `ScopeData` / `VariableData` / `ReferenceData` / `DefinitionData`
//! via the `IndexVec<*Id, _>` rows.

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::IrArena;

pub struct ScopeAnalysisResult {
    pub arena: IrArena,
    pub global_scope: ScopeId,
}
