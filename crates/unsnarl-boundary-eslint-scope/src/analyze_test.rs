//! Sibling tests for `analyze.rs`, mirroring the entry point of the
//! eslint-scope-compatible scope-builder.
//!
//! Per-feature observations live in the sibling `*_test.rs` files
//! next to each implementation module (`enter_*`, `classify/*`,
//! `hoisting/*`, etc.). This file keeps only the entry-level smoke
//! and shape tests.

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::Language;

use crate::analyze::{analyze, AnalyzeOptions};
use crate::parser::{default_source_type_for, OxcParser, ParseOptions};
use crate::testing::{analyze_source, NoopVisitor};

#[test]
fn analyze_completes_for_trivial_source() {
    let allocator = oxc_allocator::Allocator::default();
    let code = "const x = 1;\n";
    let parsed = OxcParser
        .parse(
            &allocator,
            code,
            &ParseOptions {
                language: Language::Ts,
                source_path: "input.ts".to_string(),
                source_type: default_source_type_for(Language::Ts),
            },
        )
        .unwrap();
    let mut visitor = NoopVisitor;
    let _ = analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            raw: parsed.raw,
        },
        &mut visitor,
    );
}

#[test]
fn module_source_seeds_module_scope_with_no_upper() {
    let r = analyze_source("export const x = 1;\n", Language::Ts);
    let global = &r.arena.scopes[r.global_scope];
    assert!(matches!(global.r#type, ScopeType::Module));
    assert!(global.upper.is_none());
}

#[test]
fn script_source_seeds_global_scope() {
    let allocator = oxc_allocator::Allocator::default();
    let parsed = OxcParser
        .parse(
            &allocator,
            "var x = 1;\n",
            &ParseOptions {
                language: Language::Js,
                source_path: "input.js".to_string(),
                source_type: default_source_type_for(Language::Js),
            },
        )
        .unwrap();
    let mut visitor = NoopVisitor;
    let r = analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            raw: parsed.raw,
        },
        &mut visitor,
    );
    let global = &r.arena.scopes[r.global_scope];
    assert!(matches!(global.r#type, ScopeType::Global));
}
