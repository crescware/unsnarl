//! Shared test helpers for the analyzer crate.
//!
//! Mirrors the role of `unsnarl-boundary-eslint-scope/src/testing.rs`
//! within this crate. Most analyzer-side `*_test.rs` files build
//! `PathEntry` fixtures from `(AstType, span)` tuples and compare
//! returned `PredicateContainer` / `ReferenceCompletion` rows by
//! destructuring; the helpers here keep that scaffolding from being
//! duplicated across each test file.

#![cfg(test)]

use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_parser::{Parser, ParserReturn};
use oxc_span::{SourceType, Span};

use unsnarl_ir::primitive::AstNode;
use unsnarl_oxc_parity::AstType;

use crate::path_entry::PathEntry;

pub(crate) fn ast_node(ty: AstType, start: u32) -> AstNode {
    AstNode {
        r#type: ty,
        span: Span::new(start, start),
    }
}

pub(crate) fn ast_node_with_end(ty: AstType, start: u32, end: u32) -> AstNode {
    AstNode {
        r#type: ty,
        span: Span::new(start, end),
    }
}

pub(crate) fn entry(node: AstNode, key: Option<&'static str>) -> PathEntry {
    PathEntry {
        node,
        key,
        arrow_body: None,
    }
}

pub(crate) fn entry_with_arrow_body(
    node: AstNode,
    key: Option<&'static str>,
    body_span: Span,
    is_block: bool,
) -> PathEntry {
    PathEntry {
        node,
        key,
        arrow_body: Some(crate::path_entry::ArrowBodyInfo {
            span: body_span,
            is_block,
        }),
    }
}

/// Parse `source` as TypeScript and return the resulting program /
/// allocator pair. The allocator owns the AST and must outlive any
/// `&Statement`-style borrows obtained from the program.
pub(crate) fn parse_ts<'a>(allocator: &'a Allocator, source: &'a str) -> Program<'a> {
    let source_type = SourceType::ts();
    let ParserReturn {
        program,
        errors,
        panicked,
        ..
    } = Parser::new(allocator, source, source_type).parse();
    assert!(!panicked, "parser panicked on test source");
    assert!(
        errors.is_empty(),
        "parser reported errors on test source: {errors:?}"
    );
    program
}

/// Parse `source` as TypeScript and run the eslint-scope-compatible
/// scope builder over it, returning both the program (for AST-level
/// inspection) and the analysis result (arena + global scope id).
pub(crate) fn parse_and_analyze_ts<'a>(
    allocator: &'a Allocator,
    source: &'a str,
) -> (
    Program<'a>,
    unsnarl_boundary_eslint_scope::analysis_result::EslintScopeAnalysisResult,
) {
    use unsnarl_boundary_eslint_scope::analyze::{analyze, AnalyzeOptions};
    use unsnarl_boundary_eslint_scope::parser::SourceType as BoundarySourceType;
    use unsnarl_boundary_eslint_scope::visitor::AnalysisVisitor;
    let source_type = SourceType::ts();
    let ParserReturn {
        program,
        errors,
        panicked,
        ..
    } = Parser::new(allocator, source, source_type).parse();
    assert!(!panicked, "parser panicked on test source");
    assert!(
        errors.is_empty(),
        "parser reported errors on test source: {errors:?}"
    );
    struct NoopVisitor;
    impl AnalysisVisitor for NoopVisitor {}
    let mut visitor = NoopVisitor;
    let boundary_source_type = if source_type.is_module() {
        BoundarySourceType::Module
    } else {
        BoundarySourceType::Script
    };
    let result = analyze(
        &program,
        &AnalyzeOptions {
            source_type: boundary_source_type,
            raw: source,
        },
        &mut visitor,
    );
    (program, result)
}
