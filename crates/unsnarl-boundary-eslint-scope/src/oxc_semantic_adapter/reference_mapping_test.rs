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
    let (mut variables, symbol_to_variable) =
        build_variables(&ret.semantic, &mut scopes, &translation);
    let mut definitions: IndexVec<DefinitionId, DefinitionData> = IndexVec::new();
    let references = build_references(
        &ret.semantic,
        &mut scopes,
        &mut variables,
        &mut definitions,
        &symbol_to_variable,
        &translation,
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
        assert_eq!(
            b.references.len(),
            1,
            "expected exactly one reference for `x`"
        );
        let r = &b.references[ReferenceId::from_usize(0)];
        assert_eq!(r.identifier.name(), "x");
        assert!(matches!(r.identifier.r#type, AstType::Identifier));
        assert_eq!(r.resolved, Some(var_id));
        assert!(!r.init);
        assert!((r.flags & ReferenceFlags::READ).0 != 0);
        assert!((r.flags & ReferenceFlags::WRITE).0 == 0);
        assert_eq!(r.from, root());
        assert_eq!(
            b.scopes[root()].references,
            vec![ReferenceId::from_usize(0)]
        );
        assert_eq!(
            b.variables[var_id].references,
            vec![ReferenceId::from_usize(0)]
        );
        // No implicit-global created → no through walk.
        assert!(b.scopes[root()].through.is_empty());
    });
}

#[test]
fn resolved_write_carries_write_flag() {
    with_arena("let x = 1; x = 2;", Language::Js, SourceType::Module, |b| {
        let var_id = b.scopes[root()].set().get("x").copied().expect("x exists");
        // `x = 2` is a Write reference. `oxc_semantic` does not emit
        // a reference for the binding side of `let x = 1`, so we
        // expect exactly one reference here.
        assert_eq!(b.references.len(), 1);
        let r = &b.references[ReferenceId::from_usize(0)];
        assert_eq!(r.identifier.name(), "x");
        assert_eq!(r.resolved, Some(var_id));
        assert!((r.flags & ReferenceFlags::WRITE).0 != 0);
        assert!(!r.init);
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
