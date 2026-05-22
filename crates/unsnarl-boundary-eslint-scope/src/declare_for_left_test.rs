//! Sibling tests for `declare_for_left.rs`.

use unsnarl_ir::diagnostic_kind::DiagnosticKind;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::Language;

use crate::analyze::{analyze, AnalyzeOptions};
use crate::parser::{default_source_type_for, OxcParser, ParseOptions};
use crate::testing::{analyze_source, find_first_descendant_scope, variable_names_in_scope};
use crate::visitor::AnalysisVisitor;

#[test]
fn for_let_head_lives_in_for_scope() {
    let r = analyze_source("for (let i = 0; i < 10; i++) {}\n", Language::Ts);
    let for_scope = find_first_descendant_scope(&r.arena, r.global_scope, ScopeType::For).unwrap();
    let names = variable_names_in_scope(&r.arena, for_scope);
    assert!(names.iter().any(|n| n == "i"));
}

#[test]
fn for_var_head_hoists_out_to_module_scope() {
    let r = analyze_source("for (var i = 0; i < 10; i++) {}\n", Language::Ts);
    let module_names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(module_names.iter().any(|n| n == "i"));
}

#[test]
fn for_var_head_emits_var_detected_diagnostic() {
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
            "for (var i = 0; i < 1; i++) {}\n",
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
            language: parsed.language,
            raw: parsed.raw,
        },
        &mut visitor,
    );
    assert_eq!(visitor.count, 1);
}

#[test]
fn for_of_using_head_is_skipped_by_declare_for_left() {
    // `for (using x of arr)` uses a `using` head — neither
    // Var/Let/Const, so `declare_for_left` must skip it: no binding
    // is registered on the For scope, and the `var-detected`
    // diagnostic is not emitted.
    struct Capture {
        var_detected: usize,
    }
    impl AnalysisVisitor for Capture {
        fn on_diagnostic(&mut self, diag: &unsnarl_ir::diagnostic::Diagnostic) {
            if matches!(diag.kind, DiagnosticKind::VarDetected) {
                self.var_detected += 1;
            }
        }
    }
    let allocator = oxc_allocator::Allocator::default();
    let parsed = OxcParser
        .parse(
            &allocator,
            "for (using x of arr) {}\n",
            &ParseOptions {
                language: Language::Ts,
                source_path: "input.ts".to_string(),
                source_type: default_source_type_for(Language::Ts),
            },
        )
        .unwrap();
    let mut visitor = Capture { var_detected: 0 };
    let r = analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            language: parsed.language,
            raw: parsed.raw,
        },
        &mut visitor,
    );
    let for_scope = find_first_descendant_scope(&r.arena, r.global_scope, ScopeType::For)
        .expect("a For scope must exist");
    let names = variable_names_in_scope(&r.arena, for_scope);
    assert!(
        !names.iter().any(|n| n == "x"),
        "`using` head must not register a binding on the For scope"
    );
    assert_eq!(
        visitor.var_detected, 0,
        "`using` head must not trigger the var-detected diagnostic"
    );
}
