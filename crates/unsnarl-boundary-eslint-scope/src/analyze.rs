//! Entry point for the eslint-scope-compatible scope-builder.
//!
//! Mirrors `analyze` in `ts/src/boundary/eslint-scope/analyze.ts`. Step 8.5
//! establishes only the signature so that [`crate::parser::ParsedSource`]'s
//! API surface can be reviewed against an actual consumer; the body is
//! deferred to Step 9.

use oxc_ast::ast::Program;

use crate::analysis_result::EslintScopeAnalysisResult;
use crate::parser::SourceType;
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

/// Skeleton scope-builder entry. Body deferred to Step 9.
pub fn analyze<'a>(
    program: &Program<'a>,
    options: &AnalyzeOptions<'a>,
    visitor: &dyn AnalysisVisitor,
) -> EslintScopeAnalysisResult {
    let _ = (program, options, visitor);
    todo!("Step 9: implement scope-builder body (see ts/src/boundary/eslint-scope/analyze.ts)")
}

#[cfg(test)]
#[path = "analyze_test.rs"]
mod analyze_test;
