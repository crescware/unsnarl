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
fn root_block_span_skips_leading_comments() {
    // The npm `oxc-parser` package the TS pipeline consumes reports
    // `Program.start` at the first body / directive / hashbang offset
    // (skipping leading comments); the Rust `oxc_parser` crate emits
    // `Program.span.start = 0`. `analyze` is responsible for
    // normalising the divergence onto the TS-side shape.
    //
    // Source layout: `// leading comment` is bytes 0..18, the `\n`
    // sits at byte 18, and `const x = 1;` starts at byte 19, which is
    // also where the only body element begins.
    let allocator = oxc_allocator::Allocator::default();
    let code = "// leading comment\nconst x = 1;\n";
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
    let r = analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            raw: parsed.raw,
        },
        &mut visitor,
    );
    let block = &r.arena.scopes[r.global_scope].block;
    assert_eq!(block.span.start, 19);
    assert_eq!(block.span.end, parsed.program.span.end);
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
