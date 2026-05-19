//! Sibling tests for `hoisting/handle_variable_declaration.rs`,
//! mirroring TS
//! `ts/src/boundary/eslint-scope/hoisting/handle-variable-declaration.test.ts`.

use unsnarl_ir::diagnostic_kind::DiagnosticKind;
use unsnarl_ir::Language;

use crate::analyze::{analyze, AnalyzeOptions};
use crate::parser::{default_source_type_for, OxcParser, ParseOptions};
use crate::testing::{analyze_source, variable_names_in_scope};
use crate::visitor::AnalysisVisitor;

#[test]
fn var_let_const_declarations_hoist_into_target_scope() {
    let r = analyze_source("var a = 1; let b = 2; const c = 3;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "b"));
    assert!(names.iter().any(|n| n == "c"));
}

#[test]
fn var_declaration_emits_var_detected_diagnostic() {
    struct Capture {
        count: usize,
    }
    impl AnalysisVisitor for Capture {
        fn on_diagnostic(&mut self, diag: &unsnarl_ir::diagnostic::Diagnostic) {
            if matches!(diag.kind, DiagnosticKind::VarDetected) {
                self.count += 1;
            }
        }
    }
    let allocator = oxc_allocator::Allocator::default();
    let parsed = OxcParser
        .parse(
            &allocator,
            "var x = 1;\n",
            &ParseOptions {
                language: Language::Ts,
                source_path: "input.ts".to_string(),
                source_type: default_source_type_for(Language::Ts),
            },
        )
        .unwrap();
    let mut visitor = Capture { count: 0 };
    let _ = analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            raw: parsed.raw,
        },
        &mut visitor,
    );
    assert_eq!(visitor.count, 1);
}

#[test]
fn var_inside_function_body_hoists_to_function_scope() {
    let r = analyze_source("function f() { { var inner = 1; } }\n", Language::Ts);
    let function_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let function_names = variable_names_in_scope(&r.arena, function_scope);
    assert!(function_names.iter().any(|n| n == "inner"));
}
