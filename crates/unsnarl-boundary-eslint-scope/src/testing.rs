//! Build helpers shared by the boundary-crate `*_test.rs` files.
//!
//! Mirrors `ts/src/boundary/eslint-scope/testing/`. The TS layer
//! offers `parse(code, language)` and `findFirst(root, type)`; the
//! Rust port replaces both with a higher-level helper
//! ([`analyze_source`]) that drives `OxcParser` → `analyze` and
//! returns the populated [`EslintScopeAnalysisResult`].
//!
//! Per issue #118, boundary tests stay integration-style — source
//! string in, IR observation out — so individual `enter_*` / classify
//! helpers don't need `&'a Program<'a>` mocks. The TS-side
//! `findFirst` (used to pull AST sub-nodes for unit tests) has no
//! Rust counterpart for that reason.

#![cfg(test)]

use oxc_allocator::Allocator;

use unsnarl_ir::Language;

use crate::analysis_result::EslintScopeAnalysisResult;
use crate::analyze::{analyze, AnalyzeOptions};
use crate::parser::{default_source_type_for, OxcParser, ParseOptions};
use crate::visitor::AnalysisVisitor;

pub(crate) struct NoopVisitor;
impl AnalysisVisitor for NoopVisitor {}

/// Parse `code` as the requested language and run the full
/// scope-builder pass against it. The allocator is dropped on the
/// caller's behalf once the returned arena leaves the parser
/// lifetime, so callers don't need to manage it.
pub(crate) fn analyze_source(code: &str, language: Language) -> EslintScopeAnalysisResult {
    let allocator = Allocator::default();
    let parsed = OxcParser
        .parse(
            &allocator,
            code,
            &ParseOptions {
                language,
                source_path: format!("input.{}", language_extension(language)),
                source_type: default_source_type_for(language),
            },
        )
        .expect("test source must parse cleanly");
    let mut visitor = NoopVisitor;
    analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            raw: parsed.raw,
        },
        &mut visitor,
    )
}

fn language_extension(language: Language) -> &'static str {
    match language {
        Language::Js => "js",
        Language::Jsx => "jsx",
        Language::Ts => "ts",
        Language::Tsx => "tsx",
    }
}
