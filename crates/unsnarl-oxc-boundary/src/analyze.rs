//! Entry point for the scope-builder.
//!
//! Drives [`crate::oxc_semantic_adapter::build_from_program`] against
//! the parsed AST, dispatches any diagnostics the adapter collects
//! ([`unsnarl_ir::diagnostic_kind::DiagnosticKind::VarDetected`])
//! through the supplied [`AnalysisVisitor::on_diagnostic`] callback,
//! and returns the resulting [`ScopeAnalysisResult`].

use oxc_allocator::Allocator;
use oxc_ast::ast::Program;

use unsnarl_ir::Language;

use crate::analysis_result::ScopeAnalysisResult;
use crate::oxc_semantic_adapter::build_from_program;
use crate::parser::{default_source_type_for, OxcParser, ParseError, ParseOptions, SourceType};
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

pub fn parse_and_analyze(
    code: &str,
    language: Language,
) -> Result<ScopeAnalysisResult, ParseError> {
    parse_and_analyze_with(
        code,
        language,
        default_source_type_for(language),
        &mut NoopVisitor,
    )
}

pub fn parse_and_analyze_with<V: AnalysisVisitor>(
    code: &str,
    language: Language,
    source_type: SourceType,
    visitor: &mut V,
) -> Result<ScopeAnalysisResult, ParseError> {
    let allocator = Allocator::default();
    let extension = match language {
        Language::Js => "js",
        Language::Jsx => "jsx",
        Language::Ts => "ts",
        Language::Tsx => "tsx",
    };
    let parsed = OxcParser.parse(
        &allocator,
        code,
        &ParseOptions {
            language,
            source_path: format!("input.{extension}"),
            source_type,
        },
    )?;
    Ok(analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            language,
            raw: parsed.raw,
        },
        visitor,
    ))
}

pub(crate) struct NoopVisitor;
impl AnalysisVisitor for NoopVisitor {}

#[cfg(test)]
#[path = "analyze_test.rs"]
mod analyze_test;
