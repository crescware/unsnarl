//! Integration tests for the eslint-scope-compatible scope-builder.
//!
//! Per issue #118, boundary tests are integration-style — source
//! string in, IR observation out — so the TS `*.test.ts` suite that
//! used to drive `enter_function` / `handle_identifier_reference` /
//! classify helpers directly is consolidated here as scenarios
//! against [`analyze_source`]. Each `#[test]` mirrors one or more
//! TS test cases by name.

use unsnarl_ir::diagnostic_kind::DiagnosticKind;
use unsnarl_ir::reference::reference_flags::ReferenceFlags;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::DefinitionType;
use unsnarl_ir::IrArena;
use unsnarl_ir::Language;

use crate::analysis_result::EslintScopeAnalysisResult;
use crate::analyze::{analyze, AnalyzeOptions};
use crate::parser::{default_source_type_for, OxcParser, ParseOptions};
use crate::testing::{analyze_source, NoopVisitor};
use crate::visitor::AnalysisVisitor;

fn variable_names_in_scope(arena: &IrArena, scope: unsnarl_ir::ids::ScopeId) -> Vec<String> {
    arena.scopes[scope]
        .variables
        .iter()
        .map(|&id| arena.variables[id].name().to_string())
        .collect()
}

fn variable_has_function_name_def(arena: &IrArena, name: &str) -> bool {
    arena
        .variables
        .iter()
        .filter(|v| v.name() == name)
        .flat_map(|v| v.defs.iter())
        .any(|&d| matches!(arena.definitions[d].r#type, DefinitionType::FunctionName))
}

fn variable_has_catch_clause_def(arena: &IrArena, name: &str) -> bool {
    arena
        .variables
        .iter()
        .filter(|v| v.name() == name)
        .flat_map(|v| v.defs.iter())
        .any(|&d| matches!(arena.definitions[d].r#type, DefinitionType::CatchClause))
}

fn variable_has_implicit_global_def(arena: &IrArena, name: &str) -> bool {
    arena
        .variables
        .iter()
        .filter(|v| v.name() == name)
        .flat_map(|v| v.defs.iter())
        .any(|&d| {
            matches!(
                arena.definitions[d].r#type,
                DefinitionType::ImplicitGlobalVariable
            )
        })
}

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
fn seeds_module_scope_for_module_source() {
    let r: EslintScopeAnalysisResult = analyze_source("export const x = 1;\n", Language::Ts);
    let global = &r.arena.scopes[r.global_scope];
    assert!(matches!(global.r#type, ScopeType::Module));
    assert!(global.upper.is_none());
}

#[test]
fn hoist_var_let_const_into_module_scope() {
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
fn function_declaration_hoists_into_enclosing_scope() {
    let r = analyze_source("function f() {}\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "f"));
    assert!(variable_has_function_name_def(&r.arena, "f"));
}

#[test]
fn function_creates_function_scope_with_implicit_arguments() {
    let r = analyze_source("function f(a, b) { return a + b; }\n", Language::Ts);
    let function_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let scope = &r.arena.scopes[function_scope];
    assert!(matches!(scope.r#type, ScopeType::Function));
    let names = variable_names_in_scope(&r.arena, function_scope);
    assert!(names.iter().any(|n| n == "arguments"));
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "b"));
}

#[test]
fn arrow_function_skips_implicit_arguments() {
    let r = analyze_source("const f = (a, b) => a + b;\n", Language::Ts);
    let arrow_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let names = variable_names_in_scope(&r.arena, arrow_scope);
    assert!(!names.iter().any(|n| n == "arguments"));
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "b"));
}

#[test]
fn class_declaration_hoists_outer_binding_and_creates_inner_scope() {
    let r = analyze_source("class C {}\n", Language::Ts);
    let outer_names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(outer_names.iter().any(|n| n == "C"));
    let class_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let inner = &r.arena.scopes[class_scope];
    assert!(matches!(inner.r#type, ScopeType::Class));
    let inner_names = variable_names_in_scope(&r.arena, class_scope);
    assert!(inner_names.iter().any(|n| n == "C"));
}

#[test]
fn block_statement_creates_block_scope_for_let() {
    let r = analyze_source("{ let inner = 1; }\n", Language::Ts);
    let block_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let scope = &r.arena.scopes[block_scope];
    assert!(matches!(scope.r#type, ScopeType::Block));
    let names = variable_names_in_scope(&r.arena, block_scope);
    assert!(names.iter().any(|n| n == "inner"));
}

#[test]
fn catch_scope_binds_param() {
    let r = analyze_source("try { } catch (err) { }\n", Language::Ts);
    let mut catch_scope = None;
    for child in r.arena.scopes[r.global_scope].child_scopes.clone() {
        if matches!(r.arena.scopes[child].r#type, ScopeType::Catch) {
            catch_scope = Some(child);
        }
        for grand in r.arena.scopes[child].child_scopes.clone() {
            if matches!(r.arena.scopes[grand].r#type, ScopeType::Catch) {
                catch_scope = Some(grand);
            }
        }
    }
    let catch_scope = catch_scope.expect("a catch scope must exist");
    let names = variable_names_in_scope(&r.arena, catch_scope);
    assert!(names.iter().any(|n| n == "err"));
    assert!(variable_has_catch_clause_def(&r.arena, "err"));
}

#[test]
fn for_let_binding_lives_in_for_scope() {
    let r = analyze_source("for (let i = 0; i < 10; i++) { i; }\n", Language::Ts);
    let for_scope = r.arena.scopes[r.global_scope]
        .child_scopes
        .iter()
        .copied()
        .find(|&id| matches!(r.arena.scopes[id].r#type, ScopeType::For))
        .expect("a for scope must exist");
    let names = variable_names_in_scope(&r.arena, for_scope);
    assert!(names.iter().any(|n| n == "i"));
}

#[test]
fn for_var_hoists_out_to_module() {
    let r = analyze_source("for (var i = 0; i < 10; i++) { i; }\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "i"));
}

#[test]
fn switch_statement_creates_switch_scope() {
    let r = analyze_source("switch (1) { case 1: { break; } }\n", Language::Ts);
    let switch_scope = r.arena.scopes[r.global_scope]
        .child_scopes
        .iter()
        .copied()
        .find(|&id| matches!(r.arena.scopes[id].r#type, ScopeType::Switch))
        .expect("a switch scope must exist");
    assert!(matches!(
        r.arena.scopes[switch_scope].r#type,
        ScopeType::Switch
    ));
}

#[test]
fn switch_case_creates_block_scope() {
    let r = analyze_source("switch (1) { case 1: { let x = 2; } }\n", Language::Ts);
    let switch_scope = r.arena.scopes[r.global_scope]
        .child_scopes
        .iter()
        .copied()
        .find(|&id| matches!(r.arena.scopes[id].r#type, ScopeType::Switch))
        .unwrap();
    assert!(r.arena.scopes[switch_scope]
        .child_scopes
        .iter()
        .any(|&c| matches!(r.arena.scopes[c].r#type, ScopeType::Block)));
}

#[test]
fn destructuring_binding_pattern_collects_each_identifier() {
    let r = analyze_source("const { a, b: c, ...rest } = obj;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "c"));
    assert!(names.iter().any(|n| n == "rest"));
}

#[test]
fn array_destructuring_binding_pattern_collects_each_identifier() {
    let r = analyze_source("const [a, , b, ...rest] = arr;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "b"));
    assert!(names.iter().any(|n| n == "rest"));
}

#[test]
fn identifier_reference_resolves_to_module_binding() {
    let r = analyze_source("const x = 1; const y = x;\n", Language::Ts);
    let x_id = r.arena.scopes[r.global_scope]
        .set()
        .get("x")
        .copied()
        .expect("x must be bound");
    let refs_to_x: Vec<_> = r
        .arena
        .references
        .iter()
        .filter(|r| r.resolved == Some(x_id))
        .collect();
    assert!(!refs_to_x.is_empty());
}

#[test]
fn undefined_reference_creates_implicit_global() {
    let r = analyze_source("y;\n", Language::Ts);
    assert!(r.arena.scopes[r.global_scope].set().contains_key("y"));
    assert!(variable_has_implicit_global_def(&r.arena, "y"));
}

#[test]
fn assignment_left_classified_as_write_reference() {
    let r = analyze_source("let x = 1; x = 2;\n", Language::Ts);
    let x_id = r.arena.scopes[r.global_scope]
        .set()
        .get("x")
        .copied()
        .unwrap();
    let has_write = r.arena.variables[x_id]
        .references
        .iter()
        .any(|&r_id| (r.arena.references[r_id].flags & ReferenceFlags::WRITE).0 != 0);
    assert!(has_write);
}

#[test]
fn compound_assignment_classified_as_read_write_reference() {
    let r = analyze_source("let x = 1; x += 2;\n", Language::Ts);
    let x_id = r.arena.scopes[r.global_scope]
        .set()
        .get("x")
        .copied()
        .unwrap();
    let has_read_write = r.arena.variables[x_id].references.iter().any(|&r_id| {
        let f = r.arena.references[r_id].flags;
        (f & ReferenceFlags::READ).0 != 0 && (f & ReferenceFlags::WRITE).0 != 0
    });
    assert!(has_read_write);
}

#[test]
fn update_expression_classified_as_read_write_reference() {
    let r = analyze_source("let x = 1; x++;\n", Language::Ts);
    let x_id = r.arena.scopes[r.global_scope]
        .set()
        .get("x")
        .copied()
        .unwrap();
    let has_read_write = r.arena.variables[x_id].references.iter().any(|&r_id| {
        let f = r.arena.references[r_id].flags;
        (f & ReferenceFlags::READ).0 != 0 && (f & ReferenceFlags::WRITE).0 != 0
    });
    assert!(has_read_write);
}

#[test]
fn variable_init_classified_as_init_reference() {
    let r = analyze_source("let x = 1; let y = x;\n", Language::Ts);
    let init_refs: Vec<_> = r
        .arena
        .references
        .iter()
        .filter(|r| r.init && r.identifier.name() == "x")
        .collect();
    assert!(!init_refs.is_empty());
}

#[test]
fn import_specifier_local_binds_in_module_scope() {
    let r = analyze_source(
        "import { bar as baz } from 'mod'; import foo from 'mod'; import * as ns from 'mod';\n",
        Language::Ts,
    );
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "baz"));
    assert!(names.iter().any(|n| n == "foo"));
    assert!(names.iter().any(|n| n == "ns"));
}

#[test]
fn member_property_access_does_not_create_reference_for_property_name() {
    let r = analyze_source("obj.prop;\n", Language::Ts);
    let names: Vec<_> = r
        .arena
        .references
        .iter()
        .map(|r| r.identifier.name().to_string())
        .collect();
    assert!(names.iter().any(|n| n == "obj"));
    assert!(!names.iter().any(|n| n == "prop"));
}

#[test]
fn export_named_declaration_hoists_inner_declaration() {
    let r = analyze_source("export const x = 1;\n", Language::Ts);
    let names = variable_names_in_scope(&r.arena, r.global_scope);
    assert!(names.iter().any(|n| n == "x"));
}

#[test]
fn function_param_default_classifies_outer_reference_as_read() {
    let r = analyze_source(
        "const fallback = 1; function f(a = fallback) { return a; }\n",
        Language::Ts,
    );
    let fallback_id = r.arena.scopes[r.global_scope]
        .set()
        .get("fallback")
        .copied()
        .unwrap();
    let resolved_to_fallback = r
        .arena
        .references
        .iter()
        .any(|r| r.resolved == Some(fallback_id) && r.identifier.name() == "fallback");
    assert!(resolved_to_fallback);
}
