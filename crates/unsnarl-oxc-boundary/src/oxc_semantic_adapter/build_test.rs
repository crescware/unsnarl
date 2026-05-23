//! Sibling tests for `build.rs`.
//!
//! Drive the full adapter pipeline through [`super::build`] and
//! assert properties of the [`BuildOutput`] bundle that downstream
//! consumers depend on. Today the only such property is the
//! `VarDetected` diagnostic list — every `var` declaration in the
//! input source must surface as a `Diagnostic` matching the parity
//! baseline's message string.

use oxc_allocator::Allocator;

use unsnarl_ir::diagnostic_kind::DiagnosticKind;
use unsnarl_ir::Language;

use crate::parser::{OxcParser, ParseOptions, SourceType};

use super::build;

fn build_from(code: &str, language: Language, source_type: SourceType) -> super::BuildOutput {
    let allocator = Allocator::default();
    let parsed = OxcParser
        .parse(
            &allocator,
            code,
            &ParseOptions {
                language,
                source_path: "input.js".to_string(),
                source_type,
            },
        )
        .expect("test source must parse cleanly");
    build(&parsed.program, source_type, language, code)
}

#[test]
fn empty_source_emits_no_diagnostics() {
    let out = build_from("", Language::Js, SourceType::Script);
    assert!(out.diagnostics.is_empty());
}

#[test]
fn let_and_const_declarations_emit_no_var_detected() {
    let out = build_from("let x = 1; const y = 2;", Language::Js, SourceType::Module);
    assert!(out.diagnostics.is_empty());
}

#[test]
fn single_var_declaration_emits_one_var_detected_diagnostic() {
    let out = build_from("var x = 1;", Language::Js, SourceType::Script);
    assert_eq!(out.diagnostics.len(), 1);
    let d = &out.diagnostics[0];
    assert!(matches!(d.kind, DiagnosticKind::VarDetected));
    assert!(
        d.message.contains("var declaration detected"),
        "expected parity-baseline message wording, got {:?}",
        d.message,
    );
}

#[test]
fn multi_declarator_var_emits_exactly_one_diagnostic() {
    // ONE diagnostic per `VariableDeclaration` node, not one per
    // declarator.
    let out = build_from("var a = 1, b = 2, c = 3;", Language::Js, SourceType::Script);
    assert_eq!(out.diagnostics.len(), 1);
}

#[test]
fn for_var_head_emits_var_detected_diagnostic() {
    let out = build_from(
        "for (var i = 0; i < 1; i++) {}",
        Language::Js,
        SourceType::Script,
    );
    assert_eq!(out.diagnostics.len(), 1);
    assert!(matches!(
        out.diagnostics[0].kind,
        DiagnosticKind::VarDetected
    ));
}

#[test]
fn nested_block_var_still_emits_diagnostic() {
    let out = build_from(
        "function f() { if (1) { var x = 0; } }",
        Language::Js,
        SourceType::Script,
    );
    assert_eq!(out.diagnostics.len(), 1);
}

#[test]
fn multiple_separate_var_declarations_emit_one_each() {
    let out = build_from("var x; var y; var z;", Language::Js, SourceType::Script);
    assert_eq!(out.diagnostics.len(), 3);
    for d in &out.diagnostics {
        assert!(matches!(d.kind, DiagnosticKind::VarDetected));
    }
}
