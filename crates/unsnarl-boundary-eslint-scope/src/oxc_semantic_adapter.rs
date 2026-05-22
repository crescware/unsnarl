//! Adapter from `oxc_semantic::SemanticBuilder` output to the unsnarl
//! IR shape returned by [`crate::analyze::analyze`].
//!
//! Phase 2 of the migration documented in
//! <https://github.com/crescware/unsnarl/issues/190>. The adapter
//! replaces the hand-rolled eslint-scope-compatible scope-builder; its
//! output must remain shape-compatible with the existing
//! [`EslintScopeAnalysisResult`] so downstream crates
//! (`unsnarl-analyzer`, `unsnarl-emitter-ir`, …) continue to work
//! unchanged.
//!
//! Sub-modules split the mapping by entity (scopes, variables,
//! references, definitions). The entry point is [`build_from_program`].
//!
//! [`EslintScopeAnalysisResult`]: crate::analysis_result::EslintScopeAnalysisResult

use oxc_ast::ast::Program;

use unsnarl_ir::Language;

use crate::analysis_result::EslintScopeAnalysisResult;

pub(crate) mod build;
pub(crate) mod definition_mapping;
pub(crate) mod reference_mapping;
pub(crate) mod scope_mapping;
pub(crate) mod variable_mapping;

/// Run `oxc_semantic` against `program` and adapt the result into the
/// unsnarl boundary's [`EslintScopeAnalysisResult`].
///
/// `raw` is the original source string; some downstream consumers need
/// it for span resolution. `language` selects JS / JSX / TS / TSX —
/// the underlying [`oxc_semantic::SemanticBuilder`] reads only
/// `program.source_type`, but a few normalisations the boundary applies
/// (e.g. the TypeScript hashbang/directive offset normalisation in
/// `analyze::analyze`) still depend on the language tag.
pub fn build_from_program<'a>(
    program: &Program<'a>,
    language: Language,
    raw: &'a str,
) -> EslintScopeAnalysisResult {
    build::build(program, language, raw)
}
