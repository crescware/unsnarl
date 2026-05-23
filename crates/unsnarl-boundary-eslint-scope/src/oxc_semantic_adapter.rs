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

use crate::parser::SourceType;

pub(crate) mod build;
pub(crate) mod definition_mapping;
pub(crate) mod reference_mapping;
pub(crate) mod scope_mapping;
pub(crate) mod variable_mapping;

pub use build::BuildOutput;

/// Run `oxc_semantic` against `program` and adapt the result into the
/// unsnarl boundary's [`EslintScopeAnalysisResult`].
///
/// `source_type` selects whether the root scope is `Module` or
/// `Global`. The underlying [`oxc_semantic::SemanticBuilder`] cannot
/// recover this on its own because the boundary's
/// [`crate::parser::OxcParser`] always parses with `with_module(true)`
/// to keep module-only syntax (top-level `await`, `import` / `export`)
/// parsing cleanly even when the analysis treats the file as a script;
/// the analysis-level distinction is therefore carried explicitly here.
///
/// `language` selects JS / JSX / TS / TSX. The underlying
/// [`oxc_semantic::SemanticBuilder`] reads only `program.source_type`,
/// but a few normalisations the boundary applies (e.g. the TypeScript
/// hashbang/directive offset normalisation in [`crate::analyze::analyze`])
/// still depend on the language tag.
///
/// `raw` is the original source string; some downstream consumers need
/// it for span resolution.
pub fn build_from_program<'a>(
    program: &Program<'a>,
    source_type: SourceType,
    language: Language,
    raw: &'a str,
) -> BuildOutput {
    build::build(program, source_type, language, raw)
}
