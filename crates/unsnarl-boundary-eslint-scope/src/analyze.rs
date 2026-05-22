//! Entry point for the eslint-scope-compatible scope-builder.
//!
//! Seeds the root scope, hoists the program-level declarations,
//! drives the walker via [`ScopeBuildVisitor`], flushes accumulated
//! diagnostics into the supplied visitor, and finally drains the
//! build state into an [`EslintScopeAnalysisResult`].

use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;

use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::Language;
use unsnarl_oxc_parity::AstType;

use crate::analysis_result::EslintScopeAnalysisResult;
use crate::hoist_into::hoist_into;
use crate::parser::SourceType;
use crate::scope_build_visitor::ScopeBuildVisitor;
use crate::state::{finish, ScopeBuilderState};
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
) -> EslintScopeAnalysisResult {
    let root_kind = match options.source_type {
        SourceType::Module => ScopeType::Module,
        SourceType::Script => ScopeType::Global,
    };
    // Mirror npm `oxc-parser`'s `program.start` exactly. Verified
    // empirically against `oxc-parser@0.128.0`:
    //
    //   - `lang: "ts" | "tsx"` advances past any leading hashbang and
    //     leading line / block comments, so `program.start` lands on
    //     the first directive / body statement's start (or after the
    //     hashbang if neither exists).
    //   - `lang: "js" | "jsx"` keeps `program.start = 0`; leading
    //     comments and hashbangs are part of the program span.
    //
    // The Rust `oxc_parser` crate emits `Program.span.start = 0` in
    // every case, so we have to apply the TS-only normalisation
    // ourselves. The cytoscape.min.js parity gap (the file leads with
    // a multi-line block comment) surfaced this: under the old
    // unconditional "skip to body[0].start" rule, the Rust IR
    // reported `Program.span.start = 1138` while TS reported `0`.
    let needs_ts_style_skip = matches!(options.language, Language::Ts | Language::Tsx);
    let normalised_start = if needs_ts_style_skip {
        program
            .directives
            .first()
            .map(|d| d.span.start)
            .or_else(|| {
                program
                    .body
                    .first()
                    .map(|s| oxc_span::GetSpan::span(s).start)
            })
            .or_else(|| program.hashbang.as_ref().map(|h| h.span.end))
            .unwrap_or(program.span.start)
    } else {
        program.span.start
    };
    let root_block = AstNode {
        r#type: AstType::Program,
        span: oxc_span::Span::new(normalised_start, program.span.end),
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
