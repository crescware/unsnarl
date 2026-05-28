//! Sibling tests for `analyze.rs`, the entry point of the
//! scope-builder.
//!
//! Per-feature observations live in the sibling `*_test.rs` files
//! next to each adapter implementation module
//! (`oxc_semantic_adapter/scope_mapping`, `variable_mapping`,
//! `reference_mapping`, `definition_mapping`, `build`). This file
//! keeps the entry-level smoke and shape tests plus the integration
//! cases that exercise the full pipeline.

use unsnarl_ir::diagnostic_kind::DiagnosticKind;
use unsnarl_ir::primitive::SourceLine;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::DefinitionType;
use unsnarl_ir::Language;

use crate::analyze::{analyze, AnalyzeOptions, NoopVisitor};
use crate::boundary_fixtures::{
    analyze_source, analyze_source_as, analyze_source_with_diagnostics, assert_single_def_type,
    collect_all_scopes, find_variable_in_scope, variable_names_in_scope,
};
use crate::parser::{default_source_type_for, OxcParser, ParseOptions, SourceType};

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
        .expect("trivial test source parses cleanly under the default options");
    let mut visitor = NoopVisitor;
    let _ = analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            language: parsed.language,
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
    // The IR contract spells `Program.start` as the first body /
    // directive / hashbang offset (i.e. leading comments are
    // skipped). The Rust `oxc_parser` crate emits
    // `Program.span.start = 0` for the same input; `analyze` is
    // responsible for normalising onto the contract shape.
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
        .expect("trivial test source parses cleanly under the default options");
    let mut visitor = NoopVisitor;
    let r = analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            language: parsed.language,
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
        .expect("trivial test source parses cleanly under the default options");
    let mut visitor = NoopVisitor;
    let r = analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            language: parsed.language,
            raw: parsed.raw,
        },
        &mut visitor,
    );
    let global = &r.arena.scopes[r.global_scope];
    assert!(matches!(global.r#type, ScopeType::Global));
}

// Integration cases that drive [`analyze`] through the
// `analyze_source*` helpers and pin the resulting scope tree /
// variable / definition shape against the parity baseline.

#[test]
fn module_scope_collects_const_and_let_as_variable_defs() {
    let r = analyze_source("const a = 1;\nlet b = 2;\n", Language::Ts);
    let global = &r.arena.scopes[r.global_scope];
    assert!(matches!(global.r#type, ScopeType::Module));
    assert!(global.is_strict);
    let mut names = variable_names_in_scope(&r.arena, r.global_scope);
    names.sort();
    assert_eq!(names, vec!["a".to_string(), "b".to_string()]);
    for name in ["a", "b"] {
        let v = find_variable_in_scope(&r.arena, r.global_scope, name).expect(name);
        assert_single_def_type(&r.arena, v, DefinitionType::Variable);
    }
}

#[test]
fn module_scope_collects_function_declaration_and_class_as_their_own_def_types() {
    let r = analyze_source("function foo() {}\nclass Bar {}\n", Language::Ts);
    let mut names = variable_names_in_scope(&r.arena, r.global_scope);
    names.sort();
    assert_eq!(names, vec!["Bar".to_string(), "foo".to_string()]);
    let foo = find_variable_in_scope(&r.arena, r.global_scope, "foo").expect("foo");
    let bar = find_variable_in_scope(&r.arena, r.global_scope, "Bar").expect("Bar");
    assert_single_def_type(&r.arena, foo, DefinitionType::FunctionName);
    assert_single_def_type(&r.arena, bar, DefinitionType::ClassName);
}

#[test]
fn destructuring_patterns_expand_to_individual_variables() {
    let code = "\
      const { a, b: c } = obj;
      const [x, y, ...rest] = arr;
      const { nested: { deep } } = obj;
    ";
    let r = analyze_source(code, Language::Ts);
    let mut declared: Vec<String> = r.arena.scopes[r.global_scope]
        .variables
        .iter()
        .filter(|&&id| {
            r.arena.variables[id]
                .defs
                .iter()
                .any(|&d| r.arena.definitions[d].r#type == DefinitionType::Variable)
        })
        .map(|&id| r.arena.variables[id].name().to_string())
        .collect();
    declared.sort();
    assert_eq!(
        declared,
        vec![
            "a".to_string(),
            "c".to_string(),
            "deep".to_string(),
            "rest".to_string(),
            "x".to_string(),
            "y".to_string(),
        ]
    );
}

#[test]
fn import_bindings_register_as_import_binding_defs() {
    let code = "\
      import def from \"x\";
      import { a, b as c } from \"y\";
      import * as ns from \"z\";
    ";
    let r = analyze_source(code, Language::Ts);
    let mut names = variable_names_in_scope(&r.arena, r.global_scope);
    names.sort();
    assert_eq!(
        names,
        vec![
            "a".to_string(),
            "c".to_string(),
            "def".to_string(),
            "ns".to_string()
        ]
    );
    for &v in &r.arena.scopes[r.global_scope].variables {
        assert_single_def_type(&r.arena, v, DefinitionType::ImportBinding);
    }
}

#[test]
fn function_scope_lists_parameters_as_parameter_defs() {
    let code = "function foo(a, { b }, [c], ...rest) { const inner = 1; }\n";
    let r = analyze_source(code, Language::Ts);
    assert_eq!(
        variable_names_in_scope(&r.arena, r.global_scope),
        vec!["foo".to_string()]
    );
    let fn_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    assert!(matches!(
        r.arena.scopes[fn_scope].r#type,
        ScopeType::Function
    ));
    let mut fn_names = variable_names_in_scope(&r.arena, fn_scope);
    fn_names.sort();
    assert_eq!(
        fn_names,
        vec![
            "a".to_string(),
            "arguments".to_string(),
            "b".to_string(),
            "c".to_string(),
            "inner".to_string(),
            "rest".to_string(),
        ]
    );
    let a = find_variable_in_scope(&r.arena, fn_scope, "a").expect("a");
    assert_single_def_type(&r.arena, a, DefinitionType::Parameter);
    let inner = find_variable_in_scope(&r.arena, fn_scope, "inner").expect("inner");
    assert_single_def_type(&r.arena, inner, DefinitionType::Variable);
}

#[test]
fn block_scope_is_only_created_for_non_function_blocks() {
    let code = "\
      function foo() {
        const a = 1;
        {
          const b = 2;
        }
      }
    ";
    let r = analyze_source(code, Language::Ts);
    let fn_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    assert!(matches!(
        r.arena.scopes[fn_scope].r#type,
        ScopeType::Function
    ));
    // Only the inner `{}` block under the function body becomes a
    // child block scope; the function body itself does not.
    assert_eq!(r.arena.scopes[fn_scope].child_scopes.len(), 1);
    let inner = r.arena.scopes[fn_scope].child_scopes[0];
    assert!(matches!(r.arena.scopes[inner].r#type, ScopeType::Block));
    assert_eq!(
        variable_names_in_scope(&r.arena, inner),
        vec!["b".to_string()]
    );
    let mut fn_names = variable_names_in_scope(&r.arena, fn_scope);
    fn_names.sort();
    assert_eq!(fn_names, vec!["a".to_string(), "arguments".to_string()]);
}

#[test]
fn for_scope_binds_let_inside_the_for_init() {
    let code = "for (let i = 0; i < 10; i++) { const x = i; }\n";
    let r = analyze_source(code, Language::Ts);
    let for_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    assert!(matches!(r.arena.scopes[for_scope].r#type, ScopeType::For));
    assert_eq!(
        variable_names_in_scope(&r.arena, for_scope),
        vec!["i".to_string()]
    );
    let block = r.arena.scopes[for_scope].child_scopes[0];
    assert!(matches!(r.arena.scopes[block].r#type, ScopeType::Block));
    assert_eq!(
        variable_names_in_scope(&r.arena, block),
        vec!["x".to_string()]
    );
}

#[test]
fn catch_scope_carries_catch_clause_definition() {
    let code = "try { } catch (e) { const x = 1; }\n";
    let r = analyze_source(code, Language::Ts);
    let all = collect_all_scopes(&r.arena, r.global_scope);
    let catch_scope = all
        .into_iter()
        .find(|&s| matches!(r.arena.scopes[s].r#type, ScopeType::Catch))
        .expect("catch scope present");
    let mut names = variable_names_in_scope(&r.arena, catch_scope);
    names.sort();
    assert_eq!(names, vec!["e".to_string(), "x".to_string()]);
    let e = find_variable_in_scope(&r.arena, catch_scope, "e").expect("e");
    assert_single_def_type(&r.arena, e, DefinitionType::CatchClause);
}

#[test]
fn var_bindings_register_a_variable_but_emit_a_var_detected_diagnostic() {
    let code = "var legacy = 1;\nconst modern = 2;\n";
    let (r, diagnostics) = analyze_source_with_diagnostics(code, Language::Ts);
    let mut names = variable_names_in_scope(&r.arena, r.global_scope);
    names.sort();
    assert_eq!(names, vec!["legacy".to_string(), "modern".to_string()]);
    assert_eq!(diagnostics.len(), 1);
    assert!(matches!(diagnostics[0].kind, DiagnosticKind::VarDetected));
    assert_eq!(diagnostics[0].span.line, SourceLine(1));
}

#[test]
fn ts_type_only_top_level_declarations_are_excluded() {
    let code = "\
      interface I { x: number }
      type T = string;
      enum E { A, B }
      const value = 1;
    ";
    let r = analyze_source(code, Language::Ts);
    assert_eq!(
        variable_names_in_scope(&r.arena, r.global_scope),
        vec!["value".to_string()]
    );
}

#[test]
fn module_scope_is_strict_for_ts_input() {
    let r = analyze_source("const x = 1;\n", Language::Ts);
    assert!(matches!(
        r.arena.scopes[r.global_scope].r#type,
        ScopeType::Module
    ));
    assert!(r.arena.scopes[r.global_scope].is_strict);
}

#[test]
fn function_declarations_hoist_across_the_module_scope() {
    let code = "\
      const result = foo();
      function foo() { return 1; }
    ";
    let r = analyze_source(code, Language::Ts);
    let mut names = variable_names_in_scope(&r.arena, r.global_scope);
    names.sort();
    assert_eq!(names, vec!["foo".to_string(), "result".to_string()]);
}

#[test]
fn js_source_parsed_as_module_yields_strict_module_scope() {
    let r = analyze_source_as("const x = 1;\n", Language::Js, SourceType::Module);
    assert!(matches!(
        r.arena.scopes[r.global_scope].r#type,
        ScopeType::Module
    ));
    assert!(r.arena.scopes[r.global_scope].is_strict);
}

#[test]
fn ts_source_parsed_as_script_yields_global_scope() {
    let r = analyze_source_as("const x = 1;\n", Language::Ts, SourceType::Script);
    assert!(matches!(
        r.arena.scopes[r.global_scope].r#type,
        ScopeType::Global
    ));
}

#[test]
fn class_declaration_defines_outer_class_name_and_a_class_scope_with_inner_class_name() {
    let code = "class C { static factory() { return new C(); } }\n";
    let r = analyze_source(code, Language::Ts);
    let outer = find_variable_in_scope(&r.arena, r.global_scope, "C").expect("outer C");
    assert_single_def_type(&r.arena, outer, DefinitionType::ClassName);

    let class_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    assert!(matches!(
        r.arena.scopes[class_scope].r#type,
        ScopeType::Class
    ));
    assert_eq!(
        variable_names_in_scope(&r.arena, class_scope),
        vec!["C".to_string()]
    );
    let inner = find_variable_in_scope(&r.arena, class_scope, "C").expect("inner C");
    assert_single_def_type(&r.arena, inner, DefinitionType::ClassName);
    assert_ne!(inner, outer);
}

#[test]
fn class_expression_with_a_name_creates_a_class_scope_holding_the_inner_name() {
    let code = "const X = class C { static m() { return C; } };\n";
    let r = analyze_source(code, Language::Ts);
    assert_eq!(
        variable_names_in_scope(&r.arena, r.global_scope),
        vec!["X".to_string()]
    );
    let all = collect_all_scopes(&r.arena, r.global_scope);
    let class_scope = all
        .into_iter()
        .find(|&s| matches!(r.arena.scopes[s].r#type, ScopeType::Class))
        .expect("class scope present");
    assert_eq!(
        variable_names_in_scope(&r.arena, class_scope),
        vec!["C".to_string()]
    );
}

#[test]
fn anonymous_class_expression_creates_an_empty_class_scope() {
    let code = "const X = class { m() {} };\n";
    let r = analyze_source(code, Language::Ts);
    let all = collect_all_scopes(&r.arena, r.global_scope);
    let class_scope = all
        .into_iter()
        .find(|&s| matches!(r.arena.scopes[s].r#type, ScopeType::Class))
        .expect("class scope present");
    assert!(r.arena.scopes[class_scope].variables.is_empty());
}

#[test]
fn shadowing_inside_a_nested_function_creates_separate_variables() {
    let code = "\
      const x = 1;
      function inner() {
        const x = 2;
      }
    ";
    let r = analyze_source(code, Language::Ts);
    let mut outer_names = variable_names_in_scope(&r.arena, r.global_scope);
    outer_names.sort();
    assert_eq!(outer_names, vec!["inner".to_string(), "x".to_string()]);
    let inner_scope = r.arena.scopes[r.global_scope].child_scopes[0];
    let mut inner_names = variable_names_in_scope(&r.arena, inner_scope);
    inner_names.sort();
    assert_eq!(inner_names, vec!["arguments".to_string(), "x".to_string()]);
    let outer_x = find_variable_in_scope(&r.arena, r.global_scope, "x").expect("outer x");
    let inner_x = find_variable_in_scope(&r.arena, inner_scope, "x").expect("inner x");
    assert_ne!(outer_x, inner_x);
}
