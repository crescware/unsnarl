//! Entry point for the eslint-scope-compatible scope-builder.
//!
//! Drives [`crate::oxc_semantic_adapter::build_from_program`] against
//! the parsed AST, dispatches any diagnostics the adapter collects
//! ([`unsnarl_ir::diagnostic_kind::DiagnosticKind::VarDetected`])
//! through the supplied [`AnalysisVisitor::on_diagnostic`] callback,
//! and returns the resulting [`ScopeAnalysisResult`].

use oxc_ast::ast::Program;

use unsnarl_ir::Language;

use crate::analysis_result::ScopeAnalysisResult;
use crate::oxc_semantic_adapter::build_from_program;
use crate::parser::SourceType;
use crate::visitor::AnalysisVisitor;

/// Options accepted by [`analyze`].
///
/// The `(source_type, raw)` shape carries exactly the
/// [`ParsedSource`] fields that the scope-builder actually consumes.
///
/// [`ParsedSource`]: crate::parser::ParsedSource
pub struct AnalyzeOptions<'a> {
    pub source_type: SourceType,
    pub language: Language,
    pub raw: &'a str,
}

pub fn analyze<'a>(
    program: &Program<'a>,
    options: &AnalyzeOptions<'a>,
    visitor: &mut dyn AnalysisVisitor,
) -> ScopeAnalysisResult {
    let output = build_from_program(program, options.source_type, options.language, options.raw);
    for diag in &output.diagnostics {
        visitor.on_diagnostic(diag);
    }
    output.analysis
}

#[cfg(test)]
#[path = "analyze_test.rs"]
mod analyze_test;
