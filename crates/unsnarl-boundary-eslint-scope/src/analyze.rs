//! Entry point for the eslint-scope-compatible scope-builder.
//!
//! Mirrors `analyze` in `ts/src/boundary/eslint-scope/analyze.ts`.
//! Step 9 fills the body incrementally: this initial layer seeds the
//! root scope but does not yet walk or hoist. Subsequent commits port
//! hoisting, the `enter_*` group, classify, and the per-AST-type
//! `visit_*` overrides; later commits then turn the seeded state into
//! a populated [`EslintScopeAnalysisResult`].

use oxc_ast::ast::Program;
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_oxc_parity::AstType;

use crate::analysis_result::EslintScopeAnalysisResult;
use crate::parser::SourceType;
use crate::state::{finish, ScopeBuilderState};
use crate::visitor::AnalysisVisitor;

/// Options accepted by [`analyze`].
///
/// Mirrors the `AnalyzeOptions` type alias in
/// `ts/src/boundary/eslint-scope/analyze.ts`. The shape (`source_type` +
/// `raw`) corresponds 1:1 to the [`ParsedSource`] fields that
/// `runAnalysis` actually consumes on the TS side; this colocation is
/// the YAGNI-evidence Step 8.5 was carved out to surface.
///
/// [`ParsedSource`]: crate::parser::ParsedSource
pub struct AnalyzeOptions<'a> {
    pub source_type: SourceType,
    pub raw: &'a str,
}

pub fn analyze<'a>(
    program: &Program<'a>,
    options: &AnalyzeOptions<'a>,
    visitor: &dyn AnalysisVisitor,
) -> EslintScopeAnalysisResult {
    let _ = (options.raw, visitor);
    let root_kind = match options.source_type {
        SourceType::Module => ScopeType::Module,
        SourceType::Script => ScopeType::Global,
    };
    let root_block = AstNode {
        r#type: AstType::Program,
        span: program.span,
    };
    let state = ScopeBuilderState::new(root_kind, root_block);
    let (_arena, _global_scope) = finish(state);
    EslintScopeAnalysisResult {}
}

#[cfg(test)]
#[path = "analyze_test.rs"]
mod analyze_test;
