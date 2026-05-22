//! Side-by-side probe of `oxc_semantic::SemanticBuilder` against the
//! hand-rolled eslint-scope-compatible scope-builder.
//!
//! Phase 1 of the migration documented in
//! <https://github.com/crescware/unsnarl/issues/190>. The probe runs
//! after the hand-rolled build inside [`crate::analyze::analyze`] and
//! does **not** affect the returned [`crate::analysis_result::EslintScopeAnalysisResult`];
//! it only emits a single `tracing::debug!` event with the four facts
//! Phase 2 needs as a baseline:
//!
//! - number of scopes
//! - number of symbols
//! - number of resolved references (sum across all symbols)
//! - number of root-unresolved references
//! - number of `oxc_semantic` checker errors
//!
//! Removed in Phase 4.
//!
//! The probe is fully gated behind `tracing::enabled!(Level::DEBUG)`
//! against this module's target, so a release build with `RUST_LOG`
//! unset pays only a single atomic load per [`analyze`] call. Enable
//! with e.g.
//!
//! ```text
//! RUST_LOG=unsnarl_boundary_eslint_scope::oxc_semantic_probe=debug
//! ```
//!
//! [`analyze`]: crate::analyze::analyze

use oxc_ast::ast::Program;
use oxc_semantic::SemanticBuilder;

pub(crate) fn probe(program: &Program<'_>) {
    if !tracing::enabled!(target: "unsnarl_boundary_eslint_scope::oxc_semantic_probe", tracing::Level::DEBUG)
    {
        return;
    }
    let ret = SemanticBuilder::new().build(program);
    let scoping = ret.semantic.scoping();
    let resolved_count: usize = scoping.resolved_references().map(|v| v.len()).sum();
    let unresolved_count: usize = scoping
        .root_unresolved_references()
        .values()
        .map(|v| v.len())
        .sum();
    tracing::debug!(
        target: "unsnarl_boundary_eslint_scope::oxc_semantic_probe",
        scopes = scoping.scopes_len(),
        symbols = scoping.symbols_len(),
        resolved_references = resolved_count,
        root_unresolved_references = unresolved_count,
        errors = ret.errors.len(),
        "oxc_semantic probe",
    );
}

#[cfg(test)]
#[path = "oxc_semantic_probe_test.rs"]
mod oxc_semantic_probe_test;
