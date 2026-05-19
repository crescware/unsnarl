//! Result type returned by [`crate::analyze::analyze`].
//!
//! Mirrors `EslintScopeAnalysisResult` in
//! `ts/src/boundary/eslint-scope/analysis-result.ts`. The body is left
//! intentionally empty in Step 8.5; Step 9 will populate it once the
//! scope-builder body is implemented and the `IrArena` / `ScopeId`
//! exposure shape is decided.

pub struct EslintScopeAnalysisResult {
    // Step 9: e.g. `pub global_scope: ScopeId,` (and/or the arena itself).
}
