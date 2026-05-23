//! Sibling tests for `reference_mapping.rs`.
//!
//! Tests parse a small source string, run `SemanticBuilder` followed
//! by [`super::super::scope_mapping::build_scopes`],
//! [`super::super::variable_mapping::build_variables`], and finally
//! [`super::build_references`], then assert properties of the
//! resulting reference table plus the cross-link updates on scopes /
//! variables / definitions. Characterization-style: pins the
//! 1:1 walk plus the adapter-only behaviours (synthetic-`arguments`
//! resolution, implicit-global synthesis with `through` chain,
//! `init = false` everywhere because `oxc_semantic` doesn't emit a
//! reference for the binding side of `var x = 0`). Order-sensitive
//! divergences with the hand-rolled walker are left for the parity
//! harness signal — see the module header.

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
    let mut definitions: IndexVec<DefinitionId, DefinitionData> = IndexVec::new();
    let var_result = build_variables(&ret.semantic, &mut scopes, &mut definitions, &translation);
    let mut variables = var_result.variables;
    let symbol_to_variable = var_result.symbol_to_variable;
    let synthetic_unresolved = var_result.synthetic_unresolved;
    let references = build_references(
        &ret.semantic,
        &mut scopes,
        &mut variables,
        &mut definitions,
        &symbol_to_variable,
        &translation,
        &synthetic_unresolved,
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
/// the root scope, since the hand-rolled walker has no
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

/// A TypeScript parameter property is reachable through the
/// hand-rolled walker as an ordinary read reference resolving to a
/// root-scope implicit global. The adapter must synthesise both the
/// implicit global on root and the read reference at the parameter
/// position, since `oxc_semantic` produces neither (it only binds
/// the symbol in the function scope, which `variable_mapping` skips).
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
fn jsx_intrinsic_lower_tag_is_not_a_reference() {
    // `<div />` is a JSX intrinsic in oxc-ast (JSXIdentifier, not
    // IdentifierReference), so `oxc_semantic` does not emit a
    // reference for it. The adapter passes that decision through.
    with_arena(
        "const _ = <div />;",
        Language::Jsx,
        SourceType::Module,
        |b| {
            assert!(
                b.references.iter().all(|r| r.identifier.name() != "div"),
                "lowercase JSX tag must not surface as a reference",
            );
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
            assert!(matches!(r.identifier.r#type, AstType::Identifier));
        },
    );
}
