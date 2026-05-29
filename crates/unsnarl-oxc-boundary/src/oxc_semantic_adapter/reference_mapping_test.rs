//! Sibling tests for `reference_mapping.rs`.
//!
//! Tests parse a small source string, run `SemanticBuilder` followed
//! by [`super::super::scope_mapping::build_scopes`],
//! [`super::super::variable_mapping::build_variables`], and finally
//! [`super::build_references`], then assert properties of the
//! resulting reference table plus the cross-link updates on scopes /
//! variables / definitions. Characterization-style: pins the 1:1
//! walk plus the adapter-only behaviours (synthetic-`arguments`
//! resolution, implicit-global synthesis with `through` chain,
//! synthesised `init = true` declarator writes and the immediate-child
//! init-read flag, decorator reparenting, and the final source-order
//! sort of the per-scope / per-variable reference lists).

use oxc_allocator::Allocator;
use oxc_index::IndexVec;
use oxc_semantic::SemanticBuilder;

use unsnarl_ir::ids::{DefinitionId, ReferenceId, ScopeId, VariableId};
use unsnarl_ir::reference::reference_flags::ReferenceFlags;
use unsnarl_ir::reference::ReferenceData;
use unsnarl_ir::scope::{DefinitionData, ScopeData, VariableData};
use unsnarl_ir::DefinitionType;
use unsnarl_ir::Language;
use unsnarl_oxc_parity::AstType;

use crate::oxc_semantic_adapter::scope_mapping::build_scopes;
use crate::oxc_semantic_adapter::variable_mapping::build_variables;
use crate::parser::{OxcParser, ParseOptions, SourceType};

use super::build_references;

struct Built {
    scopes: IndexVec<ScopeId, ScopeData>,
    variables: IndexVec<VariableId, VariableData>,
    references: IndexVec<ReferenceId, ReferenceData>,
    definitions: IndexVec<DefinitionId, DefinitionData>,
}

fn with_arena(code: &str, language: Language, source_type: SourceType, body: impl FnOnce(&Built)) {
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
    let switch_cases = scope_mapping.switch_cases;
    let mut definitions: IndexVec<DefinitionId, DefinitionData> = IndexVec::new();
    let var_result = build_variables(
        &ret.semantic,
        &mut scopes,
        &mut definitions,
        &translation,
        &switch_cases,
    );
    let mut variables = var_result.variables;
    let symbol_to_variable = var_result.symbol_to_variable;
    let synthetic_unresolved = var_result.synthetic_unresolved;
    let inner_class_names = var_result.inner_class_names;
    let references = build_references(
        &ret.semantic,
        &mut scopes,
        &mut variables,
        &mut definitions,
        &symbol_to_variable,
        &translation,
        &synthetic_unresolved,
        &switch_cases,
        &inner_class_names,
    );
    body(&Built {
        scopes,
        variables,
        references,
        definitions,
    });
}

fn root() -> ScopeId {
    ScopeId::from_usize(0)
}

/// `oxc_semantic` resolves the inside-body `inner` reference against
/// the named function expression's self-name symbol; the adapter
/// re-emits those resolved references as implicit-global reads on
/// the root scope, since the parity baseline carries no
/// `VariableData` for `inner` to bind against.
#[test]
fn named_function_expression_self_reference_becomes_implicit_global() {
    with_arena(
        "const f = function inner() { return inner; };",
        Language::Js,
        SourceType::Module,
        |b| {
            let inner_var = b.scopes[root()]
                .set()
                .get("inner")
                .copied()
                .expect("expected implicit global `inner` on root scope");
            assert!(b.variables[inner_var].scope == root());
            // The implicit global has one ImplicitGlobalVariable def.
            assert_eq!(b.variables[inner_var].defs.len(), 1);
            let def = &b.definitions[b.variables[inner_var].defs[0]];
            assert!(matches!(def.r#type, DefinitionType::ImplicitGlobalVariable));
            // The inside-body read reference resolves to that implicit
            // global.
            let inner_ref = b
                .references
                .iter()
                .find(|r| r.identifier.name() == "inner")
                .expect("expected a reference for `inner`");
            assert_eq!(inner_ref.resolved, Some(inner_var));
            assert!((inner_ref.flags & ReferenceFlags::READ).0 != 0);
        },
    );
}

#[test]
fn empty_script_has_no_references() {
    with_arena("", Language::Js, SourceType::Script, |b| {
        assert!(b.references.is_empty());
        assert!(b.definitions.is_empty());
        assert!(b.scopes[root()].references.is_empty());
        assert!(b.scopes[root()].through.is_empty());
    });
}

#[test]
fn resolved_read_links_back_to_variable() {
    with_arena("let x = 1; x;", Language::Js, SourceType::Module, |b| {
        let var_id = b.scopes[root()].set().get("x").copied().expect("x exists");
        // Two references: the synthesised `init = true` write at the
        // declarator's `x`, and the trailing `x;` read.
        assert_eq!(b.references.len(), 2);
        let read = b
            .references
            .iter()
            .find(|r| !r.init && (r.flags & ReferenceFlags::READ).0 != 0)
            .expect("expected a non-init read reference");
        assert_eq!(read.identifier.name(), "x");
        assert!(matches!(read.identifier.r#type, AstType::Identifier));
        assert_eq!(read.resolved, Some(var_id));
        assert!((read.flags & ReferenceFlags::WRITE).0 == 0);
        assert_eq!(read.from, root());
        // No implicit-global created → no through walk.
        assert!(b.scopes[root()].through.is_empty());
    });
}

#[test]
fn resolved_write_carries_write_flag() {
    with_arena("let x = 1; x = 2;", Language::Js, SourceType::Module, |b| {
        let var_id = b.scopes[root()].set().get("x").copied().expect("x exists");
        // Two references: the synthesised `init = true` write at the
        // declarator's `x`, and the subsequent `x = 2` write (not
        // init). Locate the non-init write.
        let assign_write = b
            .references
            .iter()
            .find(|r| !r.init && (r.flags & ReferenceFlags::WRITE).0 != 0)
            .expect("expected a non-init write reference for `x = 2`");
        assert_eq!(assign_write.identifier.name(), "x");
        assert_eq!(assign_write.resolved, Some(var_id));
    });
}

#[test]
fn variable_declarator_with_init_emits_synthetic_init_reference() {
    with_arena("let x = 1;", Language::Js, SourceType::Module, |b| {
        let var_id = b.scopes[root()].set().get("x").copied().expect("x exists");
        assert_eq!(
            b.references.len(),
            1,
            "expected exactly the synthesised init reference"
        );
        let init = &b.references[ReferenceId::from_usize(0)];
        assert!(init.init, "expected `init = true` on the synthesised ref");
        assert_eq!(init.resolved, Some(var_id));
        assert!((init.flags & ReferenceFlags::WRITE).0 != 0);
        assert!((init.flags & ReferenceFlags::READ).0 == 0);
        // The init ref is registered under the variable's scope.
        assert_eq!(init.from, root());
        assert!(b.scopes[root()]
            .references
            .contains(&ReferenceId::from_usize(0)));
        assert!(b.variables[var_id]
            .references
            .contains(&ReferenceId::from_usize(0)));
    });
}

#[test]
fn destructuring_pattern_emits_no_init_writes_at_leaf_bindings() {
    // Mirrors `classify_identifier`: a `BindingIdentifier` reached
    // through a destructuring pattern step (ObjectProperty.value,
    // ArrayPattern element, AssignmentPattern.left) returns
    // `ClassifyResult::Binding` — no reference row. Only the immediate
    // `VariableDeclarator.id = BindingIdentifier` shape promotes to a
    // synthetic `WRITE + init = true` reference. The parity baseline
    // therefore carries no init-write reference for `a` / `c` in
    // `const { a, b: c } = obj;`; the only `init = true` reference is
    // the read of `obj` itself, which sits at
    // `VariableDeclarator.init` and gets the flag via
    // `mark_variable_declarator_init_reads`.
    with_arena(
        "const { a, b: c } = obj;",
        Language::Js,
        SourceType::Module,
        |b| {
            let _a_var = b.scopes[root()].set().get("a").copied().expect("a exists");
            let _c_var = b.scopes[root()].set().get("c").copied().expect("c exists");
            let init_writes: Vec<_> = b
                .references
                .iter()
                .filter(|r| r.init && (r.flags & ReferenceFlags::WRITE).0 != 0)
                .collect();
            assert_eq!(init_writes.len(), 0);
            let init_reads: Vec<_> = b
                .references
                .iter()
                .filter(|r| r.init && (r.flags & ReferenceFlags::WRITE).0 == 0)
                .collect();
            assert_eq!(init_reads.len(), 1);
            assert_eq!(init_reads[0].identifier.name(), "obj");
        },
    );
}

/// A TypeScript parameter property appears in the parity baseline as
/// an ordinary read reference resolving to a root-scope implicit
/// global. The adapter must synthesise both the implicit global on
/// root and the read reference at the parameter position, since
/// `oxc_semantic` produces neither (it only binds the symbol in the
/// function scope, which `variable_mapping` skips).
#[test]
fn typescript_parameter_property_synthesises_read_against_implicit_global() {
    with_arena(
        "class C {\n  constructor(public x: number) {}\n}",
        Language::Ts,
        SourceType::Module,
        |b| {
            // Implicit global `x` lives on the module scope.
            let x_var = b.scopes[root()]
                .set()
                .get("x")
                .copied()
                .expect("expected implicit global `x` on module scope");
            assert!(b.variables[x_var].scope == root());
            // Exactly one ImplicitGlobalVariable def.
            assert_eq!(b.variables[x_var].defs.len(), 1);
            assert!(matches!(
                b.definitions[b.variables[x_var].defs[0]].r#type,
                DefinitionType::ImplicitGlobalVariable
            ));
            // The synthesised reference is a Read resolving to `x`.
            let r = b
                .references
                .iter()
                .find(|r| r.identifier.name() == "x")
                .expect("expected an `x` reference at the parameter position");
            assert_eq!(r.resolved, Some(x_var));
            assert!((r.flags & ReferenceFlags::READ).0 != 0);
            assert!(!r.init);
        },
    );
}

#[test]
fn variable_declarator_without_init_does_not_emit_init_reference() {
    with_arena("let x;", Language::Js, SourceType::Module, |b| {
        // No init expression → no synthesised reference.
        assert!(b.references.is_empty());
    });
}

#[test]
fn unresolved_reference_creates_implicit_global_and_through_chain() {
    with_arena(
        "function f() { return missing; }",
        Language::Js,
        SourceType::Script,
        |b| {
            // `missing` is unresolved at the root, so the adapter
            // creates an implicit global `VariableData` on the root
            // scope plus an `ImplicitGlobalVariable` `DefinitionData`.
            let missing_var = b.scopes[root()]
                .set()
                .get("missing")
                .copied()
                .expect("implicit global `missing` should be on root");
            assert!(b.variables[missing_var].scope == root());
            assert_eq!(b.variables[missing_var].defs.len(), 1);
            let def_id = b.variables[missing_var].defs[0];
            assert!(matches!(
                b.definitions[def_id].r#type,
                DefinitionType::ImplicitGlobalVariable,
            ));
            // The reference resolves to that implicit-global variable.
            let r = b
                .references
                .iter_enumerated()
                .find(|(_, r)| r.identifier.name() == "missing")
                .map(|(id, _)| id)
                .expect("expected a reference for `missing`");
            assert_eq!(b.references[r].resolved, Some(missing_var));
            // The reference is registered against the function-body
            // scope where it appears, not against the root scope.
            let f_scope = b.scopes[root()].child_scopes[0];
            assert!(b.scopes[f_scope].references.contains(&r));
            // The `through` walk goes from `from` up to and including
            // root. So the function-body scope's `through` and the
            // root's `through` both list the reference.
            assert!(b.scopes[f_scope].through.contains(&r));
            assert!(b.scopes[root()].through.contains(&r));
        },
    );
}

#[test]
fn multiple_unresolved_for_same_name_share_one_implicit_global() {
    with_arena(
        "function f() { return foo; } function g() { return foo; }",
        Language::Js,
        SourceType::Script,
        |b| {
            // Only one `foo` Variable on root, even though there are
            // two reference sites.
            let foo_var = b.scopes[root()].set().get("foo").copied().expect("foo");
            let foo_refs: Vec<_> = b
                .references
                .iter()
                .filter(|r| r.identifier.name() == "foo")
                .collect();
            assert_eq!(foo_refs.len(), 2);
            for r in &foo_refs {
                assert_eq!(r.resolved, Some(foo_var));
            }
            // The implicit-global Variable has one Definition only
            // (created at first occurrence), and both references are
            // recorded against it.
            assert_eq!(b.variables[foo_var].defs.len(), 1);
            assert_eq!(b.variables[foo_var].references.len(), 2);
        },
    );
}

#[test]
fn arguments_inside_function_resolves_to_synthetic_arguments() {
    with_arena(
        "function f() { return arguments; }",
        Language::Js,
        SourceType::Script,
        |b| {
            let f_scope = b.scopes[root()].child_scopes[0];
            let synth_args =
                b.scopes[f_scope].set().get("arguments").copied().expect(
                    "variable_mapping synthesises `arguments` in non-arrow function scopes",
                );
            let r = b
                .references
                .iter_enumerated()
                .find(|(_, r)| r.identifier.name() == "arguments")
                .map(|(id, _)| id)
                .expect("`arguments` reference must exist");
            assert_eq!(
                b.references[r].resolved,
                Some(synth_args),
                "`arguments` must resolve to the function-local synthetic binding",
            );
            // Resolved-path references do NOT populate `through`.
            assert!(b.scopes[f_scope].through.is_empty());
            assert!(b.scopes[root()].through.is_empty());
            // No implicit-global `arguments` was created.
            assert!(b.scopes[root()].set().get("arguments").is_none());
        },
    );
}

#[test]
fn arguments_in_arrow_resolves_against_outer_function_arguments() {
    with_arena(
        "function outer() { const a = () => arguments; }",
        Language::Js,
        SourceType::Script,
        |b| {
            let outer = b.scopes[root()].child_scopes[0];
            let outer_args = b.scopes[outer]
                .set()
                .get("arguments")
                .copied()
                .expect("outer function has synthetic arguments");
            // The arrow inherits `arguments` from `outer`, so the
            // arrow body's `arguments` reference must resolve up the
            // scope chain to `outer_args`.
            let r = b
                .references
                .iter()
                .find(|r| r.identifier.name() == "arguments")
                .expect("arrow body references `arguments`");
            assert_eq!(r.resolved, Some(outer_args));
        },
    );
}

#[test]
fn arguments_at_module_top_level_falls_through_to_implicit_global() {
    with_arena("arguments;", Language::Js, SourceType::Module, |b| {
        // Top-level `arguments` doesn't sit under any function
        // scope, so no synthetic `arguments` exists in the chain.
        // The adapter must fall through to the implicit-global
        // path, mirroring `resolve_in_scope_chain`'s behaviour.
        let args_var = b.scopes[root()].set().get("arguments").copied().expect(
            "top-level `arguments` reference must create an implicit-global \
                 Variable on root",
        );
        let r = b
            .references
            .iter()
            .find(|r| r.identifier.name() == "arguments")
            .expect("`arguments` reference exists");
        assert_eq!(r.resolved, Some(args_var));
        // Through chain only contains root (the reference's scope
        // is root itself, so the pre-root walk pushes nothing).
        assert_eq!(b.scopes[root()].through.len(), 1);
    });
}

#[test]
fn jsx_intrinsic_lower_tag_emits_implicit_global_jsx_reference() {
    // `<div />` parses as a `JSXIdentifier`, not an
    // `IdentifierReference`, so `oxc_semantic` does not emit a
    // reference row for it. The parity baseline lands the tag on a
    // root-scope implicit global with `AstType::JSXIdentifier` on
    // both the reference and the implicit-global definition;
    // `synthesise_identifier_name_references` provides that.
    with_arena(
        "const _ = <div />;",
        Language::Jsx,
        SourceType::Module,
        |b| {
            let div_var = b.scopes[root()]
                .set()
                .get("div")
                .copied()
                .expect("`div` implicit global on root scope");
            let r = b
                .references
                .iter()
                .find(|r| r.identifier.name() == "div")
                .expect("expected reference for `<div />` JSX tag");
            assert_eq!(r.resolved, Some(div_var));
            assert!(matches!(r.identifier.r#type, AstType::JSXIdentifier));
            assert!((r.flags & ReferenceFlags::READ).0 != 0);
            assert!((r.flags & ReferenceFlags::WRITE).0 == 0);
        },
    );
}

#[test]
fn jsx_uppercase_tag_resolves_to_outer_binding() {
    // Uppercase JSX tags are parsed as `IdentifierReference`, so
    // they flow through `reference_mapping` like any other identifier.
    with_arena(
        "const MyComp = () => null; const _ = <MyComp />;",
        Language::Jsx,
        SourceType::Module,
        |b| {
            let comp_var = b.scopes[root()]
                .set()
                .get("MyComp")
                .copied()
                .expect("MyComp binding");
            let r = b
                .references
                .iter()
                .find(|r| r.identifier.name() == "MyComp")
                .expect("JSX tag reference for `MyComp`");
            assert_eq!(r.resolved, Some(comp_var));
            // `build_identifier` rewrites the type to `JSXIdentifier`
            // when the reference node's parent is `JSXOpeningElement`,
            // matching the parity baseline, so the JSX tag carries
            // that shape even though oxc represents it as an
            // `IdentifierReference`.
            assert!(matches!(r.identifier.r#type, AstType::JSXIdentifier));
        },
    );
}

/// A `C` reference inside the body of a class `C` nested inside
/// another class `C` must resolve to the innermost synthesised
/// inner-`ClassName` row. `inner_class_names` is built in
/// `scope_descendants_from_root` DFS order, so the outer match
/// appears first; a naive walk would bind the reference to the
/// outer inner row, leaving the innermost class's self-reference
/// pointing at the wrong variable.
#[test]
fn nested_same_named_class_resolves_inner_reference_to_innermost_inner_name() {
    let source = "class C { method() { class C { foo() { return C; } } } }";
    with_arena(source, Language::Js, SourceType::Module, |b| {
        // Find both class scopes (outer + inner) by walking the
        // scope tree.
        let outer_class = b.scopes[root()].child_scopes[0];
        let outer_method = b.scopes[outer_class].child_scopes[0];
        let inner_class = b.scopes[outer_method].child_scopes[0];
        // The two inner `C` bindings are the synthesised inner
        // `ClassName` rows in each class scope.
        let outer_inner_c = b.scopes[outer_class]
            .set()
            .get("C")
            .copied()
            .expect("outer class scope must carry inner `C`");
        let inner_inner_c = b.scopes[inner_class]
            .set()
            .get("C")
            .copied()
            .expect("inner class scope must carry inner `C`");
        assert_ne!(outer_inner_c, inner_inner_c);
        // The `return C` reference in `foo` resolves to the
        // innermost (`inner_inner_c`), not the outer one.
        let r = b
            .references
            .iter()
            .find(|r| r.identifier.name() == "C")
            .expect("expected a `C` reference inside foo()");
        assert_eq!(
            r.resolved,
            Some(inner_inner_c),
            "inner-class self-reference must bind to the innermost inner ClassName, \
             not the outer one",
        );
        // Cross-link bookkeeping: the inner row owns the reference;
        // the outer row does not.
        assert!(b.variables[inner_inner_c]
            .references
            .iter()
            .any(|&id| b.references[id].identifier.name() == "C"));
        assert!(b.variables[outer_inner_c]
            .references
            .iter()
            .all(|&id| b.references[id].identifier.name() != "C"));
    });
}

/// `mark_variable_declarator_init_reads` stamps `init = true` only on a
/// read whose identifier is the *immediate* `init` of a
/// `VariableDeclarator`. An identifier nested inside a wrapping
/// expression (`a + 1`) keeps `init = false`.
#[test]
fn init_read_flag_marks_only_immediate_child_identifier() {
    with_arena(
        "let a = 1; let b = a; let c = a + 1;",
        Language::Js,
        SourceType::Module,
        |b| {
            // The reads of `a` are `let b = a` (immediate init child) and
            // `let c = a + 1` (nested under a BinaryExpression). Exclude
            // the synthetic `init` WRITE at the `a` binding itself.
            let mut a_reads: Vec<_> = b
                .references
                .iter()
                .filter(|r| {
                    r.identifier.name() == "a"
                        && (r.flags & ReferenceFlags::READ).0 != 0
                        && (r.flags & ReferenceFlags::WRITE).0 == 0
                })
                .collect();
            a_reads.sort_by_key(|r| r.identifier.span.start);
            assert_eq!(a_reads.len(), 2);
            assert!(
                a_reads[0].init,
                "immediate-child `a` in `let b = a` must carry init = true"
            );
            assert!(
                !a_reads[1].init,
                "nested `a` in `let c = a + 1` must keep init = false"
            );
        },
    );
}

/// A class decorator reference is recorded by `oxc_semantic` in the
/// class's enclosing scope; `reparent_decorator_references` moves it to
/// the class scope when the identifier span lies inside the decorator's
/// span. A reference outside any decorator span is left in place.
#[test]
fn decorator_reference_is_reparented_into_class_scope() {
    with_arena(
        "@dec\nclass C {}\nother;",
        Language::Ts,
        SourceType::Module,
        |b| {
            let class_scope = b.scopes[root()].child_scopes[0];
            // The `dec` reference (inside the decorator span) is moved to
            // the class scope and removed from the module scope.
            let dec = b
                .references
                .iter()
                .find(|r| r.identifier.name() == "dec")
                .expect("expected a `dec` decorator reference");
            assert_eq!(dec.from, class_scope);
            assert!(b.scopes[class_scope]
                .references
                .iter()
                .any(|&id| b.references[id].identifier.name() == "dec"));
            assert!(b.scopes[root()]
                .references
                .iter()
                .all(|&id| b.references[id].identifier.name() != "dec"));
            // `other`, outside the decorator span, stays in the module
            // scope.
            let other = b
                .references
                .iter()
                .find(|r| r.identifier.name() == "other")
                .expect("expected an `other` reference");
            assert_eq!(other.from, root());
        },
    );
}

/// `sort_reference_lists_by_source_order` leaves every scope's
/// `references` list and every variable's `references` list ordered by
/// the identifier's source offset, even though the multi-pass walk
/// inserts them out of order. In `let x = 1; x;` the resolved loop
/// inserts the `x;` read first, then the synthesis pass inserts the
/// declarator's init write at the earlier offset.
#[test]
fn reference_lists_are_sorted_by_source_offset() {
    with_arena("let x = 1; x;", Language::Js, SourceType::Module, |b| {
        let refs = &b.scopes[root()].references;
        assert_eq!(refs.len(), 2);
        let spans: Vec<u32> = refs
            .iter()
            .map(|&id| b.references[id].identifier.span.start)
            .collect();
        assert!(
            spans[0] < spans[1],
            "scope.references must be ascending by source offset, got {spans:?}"
        );
        // The earlier entry is the declarator init write; the later one
        // is the trailing read.
        assert!(b.references[refs[0]].init);
        assert!(!b.references[refs[1]].init);
        // The variable's reference list is sorted the same way.
        let var_id = b.scopes[root()].set().get("x").copied().expect("x exists");
        let var_spans: Vec<u32> = b.variables[var_id]
            .references
            .iter()
            .map(|&id| b.references[id].identifier.span.start)
            .collect();
        let mut sorted = var_spans.clone();
        sorted.sort_unstable();
        assert_eq!(var_spans, sorted);
    });
}

/// Reparenting a reference into a synthesised per-case Block scope
/// (`reparent_to_switch_case`) does not disturb the final source-order
/// sort. In `case 1: a; b; a;` the resolved loop groups by symbol and
/// inserts the two `a` reads before the `b` read, but the case scope's
/// `references` list ends up in source order (`a`, `b`, `a`).
#[test]
fn switch_case_reparenting_preserves_source_order() {
    with_arena(
        "let a = 1; let b = 2; switch (0) { case 1: a; b; a; }",
        Language::Js,
        SourceType::Module,
        |b| {
            let switch_scope = b.scopes[root()].child_scopes[0];
            let case_scope = b.scopes[switch_scope].child_scopes[0];
            // All three case-body reads were reparented into the case
            // scope.
            for r in b.references.iter() {
                if matches!(r.identifier.name(), "a" | "b")
                    && (r.flags & ReferenceFlags::WRITE).0 == 0
                {
                    assert_eq!(r.from, case_scope);
                }
            }
            let ordered: Vec<(&str, u32)> = b.scopes[case_scope]
                .references
                .iter()
                .map(|&id| {
                    (
                        b.references[id].identifier.name(),
                        b.references[id].identifier.span.start,
                    )
                })
                .collect();
            let spans: Vec<u32> = ordered.iter().map(|&(_, s)| s).collect();
            let mut sorted = spans.clone();
            sorted.sort_unstable();
            assert_eq!(
                spans, sorted,
                "case scope references must be source-ordered"
            );
            assert_eq!(
                ordered.iter().map(|&(n, _)| n).collect::<Vec<_>>(),
                vec!["a", "b", "a"],
            );
        },
    );
}
