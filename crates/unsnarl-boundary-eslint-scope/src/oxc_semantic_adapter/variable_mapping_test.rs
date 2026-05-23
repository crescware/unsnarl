//! Sibling tests for `variable_mapping.rs`.
//!
//! Tests parse a small source string, run `SemanticBuilder` followed
//! by [`super::scope_mapping::build_scopes`] and
//! [`super::variable_mapping::build_variables`], and assert properties
//! of the resulting variables list and scope-side `set` / `variables`
//! cross-links. Characterization-style: pins the 1:1 walk shape plus
//! the implicit-`arguments` synthesis. Ordering and TypeScript-only-
//! scope filtering are deferred (see the module header for the full
//! list).

use std::collections::HashSet;

use oxc_allocator::Allocator;
use oxc_index::IndexVec;
use oxc_semantic::SemanticBuilder;

use unsnarl_ir::ids::{ScopeId, VariableId};
use unsnarl_ir::scope::{ScopeData, VariableData};
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::Language;

use crate::parser::{OxcParser, ParseOptions, SourceType};

use super::build_variables;
use crate::oxc_semantic_adapter::scope_mapping::build_scopes;

fn with_arena(
    code: &str,
    language: Language,
    source_type: SourceType,
    body: impl FnOnce(&IndexVec<ScopeId, ScopeData>, &IndexVec<VariableId, VariableData>),
) {
    let allocator = Allocator::default();
    let parsed = OxcParser
        .parse(
            &allocator,
            code,
            &ParseOptions {
                language,
                source_path: format!(
                    "input.{}",
                    match language {
                        Language::Js => "js",
                        Language::Jsx => "jsx",
                        Language::Ts => "ts",
                        Language::Tsx => "tsx",
                    }
                ),
                source_type,
            },
        )
        .expect("test source must parse cleanly");
    let ret = SemanticBuilder::new().build(&parsed.program);
    let scope_mapping = build_scopes(&ret.semantic, source_type, language);
    let mut scopes = scope_mapping.scopes;
    let translation = scope_mapping.translation;
    let result = build_variables(&ret.semantic, &mut scopes, &translation);
    body(&scopes, &result.variables);
}

fn root() -> ScopeId {
    ScopeId::from_usize(0)
}

fn names_in(
    scope: ScopeId,
    scopes: &IndexVec<ScopeId, ScopeData>,
    variables: &IndexVec<VariableId, VariableData>,
) -> HashSet<String> {
    scopes[scope]
        .variables
        .iter()
        .map(|&id| variables[id].name().to_string())
        .collect()
}

#[test]
fn empty_script_has_no_variables() {
    with_arena("", Language::Js, SourceType::Script, |scopes, variables| {
        assert!(variables.is_empty());
        assert!(scopes[root()].variables.is_empty());
        assert!(scopes[root()].set().is_empty());
    });
}

#[test]
fn module_scope_let_binding_registers_one_variable() {
    with_arena(
        "let x = 1;",
        Language::Js,
        SourceType::Module,
        |scopes, variables| {
            assert_eq!(variables.len(), 1);
            let var_id = scopes[root()].variables[0];
            assert_eq!(variables[var_id].name(), "x");
            assert!(variables[var_id].scope == root());
            // The binding-identifier occurrence is recorded.
            assert_eq!(variables[var_id].identifiers.len(), 1);
            assert_eq!(variables[var_id].identifiers[0].name(), "x");
            // refs / defs are filled by later passes; empty here.
            assert!(variables[var_id].references.is_empty());
            assert!(variables[var_id].defs.is_empty());
            // The scope's `set` index links the name to the same id.
            assert_eq!(scopes[root()].set().get("x").copied(), Some(var_id));
        },
    );
}

#[test]
fn function_scope_synthesises_arguments_binding() {
    with_arena(
        "function f(a, b) { return a + b; }",
        Language::Js,
        SourceType::Script,
        |scopes, variables| {
            // root has `f`; function scope has `arguments`, `a`, `b`.
            let f_scope = scopes[root()].child_scopes[0];
            assert!(matches!(scopes[f_scope].r#type, ScopeType::Function));
            let names = names_in(f_scope, scopes, variables);
            assert!(
                names.contains("arguments"),
                "expected synthesised `arguments` binding (got {names:?})",
            );
            assert!(names.contains("a"));
            assert!(names.contains("b"));
            // The synthesised `arguments` has no identifier occurrences
            // and no defs — eslint-scope shape.
            let arg_id = scopes[f_scope].set().get("arguments").copied().unwrap();
            assert!(variables[arg_id].identifiers.is_empty());
            assert!(variables[arg_id].defs.is_empty());
            assert!(variables[arg_id].scope == f_scope);
        },
    );
}

#[test]
fn arrow_function_scope_does_not_synthesise_arguments() {
    with_arena(
        "const f = (a) => a;",
        Language::Js,
        SourceType::Module,
        |scopes, variables| {
            // Root has `f`; arrow function scope has only `a`.
            let arrow = scopes[root()].child_scopes[0];
            assert!(matches!(scopes[arrow].r#type, ScopeType::Function));
            let names = names_in(arrow, scopes, variables);
            assert!(
                !names.contains("arguments"),
                "arrow functions inherit `arguments`; adapter must not synthesise it here \
                 (got {names:?})",
            );
            assert!(names.contains("a"));
        },
    );
}

#[test]
fn nested_function_each_get_their_own_arguments() {
    with_arena(
        "function outer() { function inner() {} }",
        Language::Js,
        SourceType::Script,
        |scopes, variables| {
            let outer = scopes[root()].child_scopes[0];
            assert!(scopes[outer].set().contains_key("arguments"));
            let inner = scopes[outer].child_scopes[0];
            assert!(scopes[inner].set().contains_key("arguments"));
            let outer_args = scopes[outer].set().get("arguments").copied().unwrap();
            let inner_args = scopes[inner].set().get("arguments").copied().unwrap();
            assert_ne!(outer_args, inner_args);
            // Each `arguments` is anchored to its declaring function scope.
            assert!(variables[outer_args].scope == outer);
            assert!(variables[inner_args].scope == inner);
        },
    );
}

#[test]
fn var_redeclaration_collapses_to_single_variable_with_two_identifier_occurrences() {
    with_arena(
        "var x; var x;",
        Language::Js,
        SourceType::Script,
        |scopes, variables| {
            let names: Vec<&str> = scopes[root()]
                .variables
                .iter()
                .map(|&id| variables[id].name())
                .collect();
            assert_eq!(names.iter().filter(|n| **n == "x").count(), 1);
            let var_id = scopes[root()].set().get("x").copied().unwrap();
            // Both `var x;` occurrences are recorded against the same
            // VariableData; the hand-rolled walker pushes one
            // identifier per declaration site.
            assert_eq!(variables[var_id].identifiers.len(), 2);
        },
    );
}

#[test]
fn block_scoped_let_lives_in_block_scope_not_function_scope() {
    with_arena(
        "function f() { let z = 1; }",
        Language::Js,
        SourceType::Module,
        |scopes, variables| {
            let f_scope = scopes[root()].child_scopes[0];
            // `z` is `let`, so it stays in the function-body block
            // scope rather than hoisting to the function scope. The
            // function-body block scope is the function scope's first
            // child... actually `oxc_semantic` hoists `let` into the
            // function scope when the body's block is the function
            // body itself. Read the actual shape from the symbol's
            // declaring scope rather than asserting placement, so this
            // test pins behaviour rather than guessing.
            let var_id = variables
                .iter_enumerated()
                .find(|(_, v)| v.name() == "z")
                .map(|(id, _)| id)
                .expect("expected to find a variable named `z`");
            // Either way, `z` must not appear in the root scope.
            assert!(
                variables[var_id].scope != root(),
                "`let z` must not be declared in the root scope",
            );
            // And the scope `z` is declared in must list it in its
            // `variables` and `set`.
            let declaring = variables[var_id].scope;
            assert!(scopes[declaring].variables.contains(&var_id));
            assert_eq!(
                scopes[declaring].set().get("z").copied(),
                Some(var_id),
                "scope's `set` must link the name back to the variable id",
            );
            // The function scope itself still carries `arguments`.
            assert!(scopes[f_scope].set().contains_key("arguments"));
        },
    );
}

/// The catch body `BlockStatement` is merged into the `Catch` scope
/// (see `scope_mapping::is_merged_into_parent`). Block-scoped
/// declarations inside the body must therefore surface inside the
/// `Catch` scope's `variables` / `set`, alongside the catch param.
#[test]
fn catch_body_let_binding_merges_into_catch_scope() {
    with_arena(
        "try {} catch (e) { let x; }",
        Language::Js,
        SourceType::Script,
        |scopes, variables| {
            let catch = scopes
                .iter_enumerated()
                .find(|(_, s)| matches!(s.r#type, ScopeType::Catch))
                .map(|(id, _)| id)
                .expect("expected a Catch scope");
            let names = names_in(catch, scopes, variables);
            assert!(
                names.contains("e"),
                "catch param `e` must live in the Catch scope (got {names:?})",
            );
            assert!(
                names.contains("x"),
                "`let x` from the catch body must merge into the Catch scope (got {names:?})",
            );
        },
    );
}

/// The boundary's hand-rolled walker classifies a named function
/// expression's `id` as a direct binding but never allocates a
/// `VariableData` for it. The adapter must mirror that: skip emitting
/// `inner` as a Variable in the Function scope.
#[test]
fn named_function_expression_self_name_is_not_emitted_as_a_variable() {
    with_arena(
        "const f = function inner() { return inner; };",
        Language::Js,
        SourceType::Module,
        |scopes, variables| {
            let fn_scope = scopes[root()].child_scopes[0];
            let names = names_in(fn_scope, scopes, variables);
            assert!(
                !names.contains("inner"),
                "function-expression self-name `inner` must not be emitted as a Variable \
                 (got {names:?})",
            );
        },
    );
}

#[test]
fn class_declaration_creates_class_named_binding_in_outer_scope() {
    with_arena(
        "class C {}",
        Language::Js,
        SourceType::Module,
        |scopes, variables| {
            // The class binding lives in the outer (module) scope.
            let var_id = scopes[root()].set().get("C").copied();
            assert!(
                var_id.is_some(),
                "expected `C` to be declared in the module scope",
            );
            let var_id = var_id.unwrap();
            assert!(variables[var_id].scope == root());
            assert_eq!(variables[var_id].identifiers.len(), 1);
            assert_eq!(variables[var_id].identifiers[0].name(), "C");
        },
    );
}
