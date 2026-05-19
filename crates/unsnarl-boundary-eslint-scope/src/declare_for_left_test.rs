//! Sibling tests for `declare_for_left.rs`, mirroring TS
//! `ts/src/boundary/eslint-scope/declare-for-left.test.ts`.

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
            raw: parsed.raw,
        },
        &mut visitor,
    );
    assert_eq!(visitor.count, 1);
}
