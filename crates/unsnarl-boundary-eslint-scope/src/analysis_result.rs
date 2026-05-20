//! Result type returned by [`crate::analyze::analyze`].
//!
//! Mirrors `EslintScopeAnalysisResult` in
//! `ts/src/boundary/eslint-scope/analysis-result.ts`. The TS port
//! exposes only `globalScope: Scope` because TS scopes carry their
//! arena identity through `Scope` references. The Rust port exposes
//! the arena explicitly so downstream crates can walk `ScopeData` /
//! `VariableData` / `ReferenceData` / `DefinitionData` via the
//! `IndexVec<*Id, _>` rows.

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::IrArena;

pub struct EslintScopeAnalysisResult {
    pub arena: IrArena,
    pub global_scope: ScopeId,
}
