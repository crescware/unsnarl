//! Entry point for the eslint-scope-compatible scope-builder.
//!
//! Mirrors `analyze` in `ts/src/boundary/eslint-scope/analyze.ts`.
//! The body now runs end-to-end: seed the root scope, hoist the
//! program-level declarations, drive the walker via
//! [`ScopeBuildVisitor`], flush accumulated diagnostics into the
//! supplied visitor, and finally drain the build state into an
//! [`EslintScopeAnalysisResult`].

use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;

use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_oxc_parity::AstType;

use crate::analysis_result::EslintScopeAnalysisResult;
use crate::hoist_into::hoist_into;
use crate::parser::SourceType;
use crate::scope_build_visitor::ScopeBuildVisitor;
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
    visitor: &mut dyn AnalysisVisitor,
) -> EslintScopeAnalysisResult {
    let root_kind = match options.source_type {
        SourceType::Module => ScopeType::Module,
        SourceType::Script => ScopeType::Global,
    };
    let root_block = AstNode {
        r#type: AstType::Program,
        span: program.span,
    };
    let mut state = ScopeBuilderState::new(root_kind, root_block);
    let global_scope = state.global_scope;
    hoist_into(&mut state, program, global_scope, options.raw);
    let mut walker = ScopeBuildVisitor::new(&mut state, visitor, options.raw);
    walker.visit_program(program);
    let (arena, global_scope, diagnostics) = finish(state);
    for diag in &diagnostics {
        visitor.on_diagnostic(diag);
    }
    EslintScopeAnalysisResult {
        arena,
        global_scope,
    }
}

#[cfg(test)]
#[path = "analyze_test.rs"]
mod analyze_test;
