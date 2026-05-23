//! Sibling tests for `scope_mapping.rs`.
//!
//! Each test parses a small source string, runs `SemanticBuilder`
//! followed by [`super::build_scopes`], and asserts properties of the
//! resulting scope tree. The tests are characterization-style: they
//! pin the adapter's mapping decisions (anchor → `ScopeType`,
//! `is_strict` propagation, `variable_scope` resolution, `upper` /
//! `child_scopes` wiring) and document known divergences from the
//! eslint-scope-compatible hand-rolled walker that follow-up commits
//! must address.

use oxc_allocator::Allocator;
use oxc_index::IndexVec;
use oxc_semantic::SemanticBuilder;

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::scope::ScopeData;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::Language;

use crate::parser::{OxcParser, ParseOptions, SourceType};

use super::build_scopes;

fn with_scopes(
    code: &str,
    language: Language,
    source_type: SourceType,
    body: impl FnOnce(&IndexVec<ScopeId, ScopeData>),
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
    let scope_mapping = build_scopes(&ret.semantic, source_type);
    body(&scope_mapping.scopes);
}

fn root() -> ScopeId {
    ScopeId::from_usize(0)
}

#[test]
fn empty_script_yields_single_global_scope() {
    with_scopes("", Language::Js, SourceType::Script, |scopes| {
        assert_eq!(scopes.len(), 1);
        let s = &scopes[root()];
        assert!(matches!(s.r#type, ScopeType::Global));
        assert!(!s.is_strict);
        assert!(s.upper.is_none());
        assert!(s.child_scopes.is_empty());
        assert!(s.variable_scope == root());
    });
}

#[test]
fn empty_module_yields_single_module_scope() {
    with_scopes("", Language::Js, SourceType::Module, |scopes| {
        assert_eq!(scopes.len(), 1);
        let s = &scopes[root()];
        assert!(matches!(s.r#type, ScopeType::Module));
        assert!(s.is_strict);
    });
}

#[test]
fn function_declaration_adds_function_scope_under_root() {
    with_scopes(
        "function f() {}",
        Language::Js,
        SourceType::Script,
        |scopes| {
            assert_eq!(scopes.len(), 2);
            let r = &scopes[root()];
            assert_eq!(r.child_scopes.len(), 1);
            let fn_id = r.child_scopes[0];
            let f = &scopes[fn_id];
            assert!(matches!(f.r#type, ScopeType::Function));
            assert!(f.upper == Some(root()));
            assert!(f.variable_scope == fn_id);
            assert!(!f.is_strict);
        },
    );
}

#[test]
fn arrow_function_collapses_to_function_scope_type() {
    with_scopes(
        "const f = () => 0;",
        Language::Js,
        SourceType::Script,
        |scopes| {
            assert_eq!(scopes.len(), 2);
            let arrow = scopes[root()].child_scopes[0];
            assert!(matches!(scopes[arrow].r#type, ScopeType::Function));
            assert!(scopes[arrow].variable_scope == arrow);
        },
    );
}

#[test]
fn top_level_block_statement_creates_block_scope_with_parent_var_scope() {
    with_scopes("{ }", Language::Js, SourceType::Script, |scopes| {
        assert_eq!(scopes.len(), 2);
        let block = scopes[root()].child_scopes[0];
        let b = &scopes[block];
        assert!(matches!(b.r#type, ScopeType::Block));
        assert!(b.upper == Some(root()));
        // Block scopes do not introduce a var scope; they share the
        // enclosing var-creating scope.
        assert!(b.variable_scope == root());
    });
}

#[test]
fn class_declaration_creates_class_scope_inheriting_parent_strictness() {
    with_scopes("class C {}", Language::Js, SourceType::Script, |scopes| {
        assert_eq!(scopes.len(), 2);
        let class = scopes[root()].child_scopes[0];
        let c = &scopes[class];
        assert!(matches!(c.r#type, ScopeType::Class));
        // The hand-rolled scope-builder propagates `is_strict` purely
        // from the root scope's analysis-level source type (Module ⇒
        // strict, Script ⇒ sloppy) without recognising
        // class-body auto-strictness. Mirror that behaviour: a class
        // in a script stays `is_strict = false`.
        assert!(!c.is_strict);
    });

    with_scopes("class C {}", Language::Js, SourceType::Module, |scopes| {
        let class = scopes[root()].child_scopes[0];
        // In module mode the root is strict, so the class inherits
        // strict — same as the hand-rolled implementation.
        assert!(scopes[class].is_strict);
    });
}

#[test]
fn for_statement_creates_for_scope() {
    with_scopes(
        "for (let i = 0; i < 1; i++) {}",
        Language::Js,
        SourceType::Script,
        |scopes| {
            // Root + For + (BlockStatement body)
            assert!(scopes.len() >= 2);
            let for_scope = scopes[root()].child_scopes[0];
            assert!(matches!(scopes[for_scope].r#type, ScopeType::For));
        },
    );
}

#[test]
fn switch_statement_creates_switch_scope() {
    with_scopes(
        "switch (x) { case 1: break; }",
        Language::Js,
        SourceType::Script,
        |scopes| {
            let switch = scopes[root()].child_scopes[0];
            assert!(matches!(scopes[switch].r#type, ScopeType::Switch));
        },
    );
}

/// eslint-scope creates one `Block` scope per `SwitchCase`, anchored
/// to the `SwitchCase` AST node. `oxc_semantic` does not, so the
/// adapter synthesises these rows immediately after each
/// `SwitchStatement` scope it emits.
#[test]
fn switch_statement_synthesises_one_block_per_case() {
    with_scopes(
        "switch (k) { case 1: break; case 2: break; default: break; }",
        Language::Js,
        SourceType::Script,
        |scopes| {
            let switch = scopes[root()].child_scopes[0];
            assert!(matches!(scopes[switch].r#type, ScopeType::Switch));
            // Three synthetic case scopes hang off the switch.
            assert_eq!(scopes[switch].child_scopes.len(), 3);
            for &case_id in &scopes[switch].child_scopes {
                assert!(matches!(scopes[case_id].r#type, ScopeType::Block));
                assert!(scopes[case_id].variable_scope == root());
                assert!(scopes[case_id].upper == Some(switch));
            }
        },
    );
}

/// A scope nested inside a `case` consequent (here a `BlockStatement`
/// around `let x`) must be parented to the synthetic case row, not
/// the bare `Switch` row that `oxc_semantic` would otherwise pick as
/// its `upper`.
#[test]
fn switch_case_nested_block_attaches_to_synthetic_case_scope() {
    with_scopes(
        "switch (k) { case 1: { let x; break; } default: break; }",
        Language::Js,
        SourceType::Script,
        |scopes| {
            let switch = scopes[root()].child_scopes[0];
            assert!(matches!(scopes[switch].r#type, ScopeType::Switch));
            assert_eq!(scopes[switch].child_scopes.len(), 2);
            // The first case (the `case 1:` arm) carries the nested
            // `{ let x; ... }` BlockStatement scope as its sole child.
            let case_one = scopes[switch].child_scopes[0];
            assert_eq!(scopes[case_one].child_scopes.len(), 1);
            let inner = scopes[case_one].child_scopes[0];
            assert!(matches!(scopes[inner].r#type, ScopeType::Block));
            assert!(scopes[inner].upper == Some(case_one));
            // The `default:` arm has no nested scopes.
            let case_two = scopes[switch].child_scopes[1];
            assert!(scopes[case_two].child_scopes.is_empty());
        },
    );
}

#[test]
fn with_statement_creates_with_scope_in_sloppy_mode() {
    with_scopes(
        "var o; with (o) { x; }",
        Language::Js,
        SourceType::Script,
        |scopes| {
            let with = scopes[root()].child_scopes[0];
            assert!(matches!(scopes[with].r#type, ScopeType::With));
            assert!(!scopes[with].is_strict);
        },
    );
}

#[test]
fn class_static_block_creates_class_static_block_scope() {
    with_scopes(
        "class C { static { let x; } }",
        Language::Js,
        SourceType::Module,
        |scopes| {
            // Module + Class + ClassStaticBlock
            let class = scopes[root()].child_scopes[0];
            let static_block = scopes[class].child_scopes[0];
            assert!(matches!(
                scopes[static_block].r#type,
                ScopeType::ClassStaticBlock
            ));
            assert!(scopes[static_block].variable_scope == static_block);
        },
    );
}

/// `oxc_semantic` emits two scopes for `try {} catch (e) {}`: the
/// `CatchClause` (parameter scope, `ScopeFlags::CatchClause`) and an
/// inner `BlockStatement` (the catch body, empty flags). Eslint-scope
/// collapses these into a single `Catch` scope holding both the param
/// and the body's declarations.
///
/// The adapter folds the catch body `BlockStatement` into the
/// `CatchClause`'s IR row so the emitted tree matches eslint-scope's
/// shape: a single `Catch` scope under the `TryStatement`'s siblings,
/// with no nested body block of its own.
#[test]
fn catch_clause_merges_body_block_into_catch_scope() {
    with_scopes(
        "try {} catch (e) { let x; }",
        Language::Js,
        SourceType::Script,
        |scopes| {
            let catch = scopes
                .iter_enumerated()
                .find(|(_, s)| matches!(s.r#type, ScopeType::Catch))
                .map(|(id, _)| id)
                .expect("expected a Catch scope for `catch (e) {}`");
            // After the merge, the catch body `BlockStatement` is
            // absorbed into the `Catch` row: no spurious Block child.
            assert!(
                scopes[catch].child_scopes.is_empty(),
                "expected catch scope to have no child scopes after merge",
            );
            // No standalone `Block` scope should be visible for the
            // catch body — only the try body's block remains.
            let block_count = scopes
                .iter()
                .filter(|s| matches!(s.r#type, ScopeType::Block))
                .count();
            assert_eq!(
                block_count, 1,
                "expected exactly one Block scope (the try body) after catch merge",
            );
        },
    );
}

/// Direct 1:1 mapping currently does NOT synthesise the
/// `FunctionExpressionName` wrapper scope. The named function
/// expression's binding therefore ends up inside the function scope
/// itself in `oxc_semantic`. This test pins the divergence so the
/// follow-up commit that adds the wrapper is observable.
#[test]
fn named_function_expression_currently_lacks_function_expression_name_wrapper() {
    with_scopes(
        "const f = function inner() { return inner; };",
        Language::Js,
        SourceType::Script,
        |scopes| {
            let any_fen = scopes
                .iter()
                .any(|s| matches!(s.r#type, ScopeType::FunctionExpressionName));
            assert!(
                !any_fen,
                "FunctionExpressionName wrapper should be absent prior to synthesis",
            );
            // Exactly one Function scope is present, holding the
            // params + body. The binding `inner` (visible inside it)
            // is registered in that scope by `oxc_semantic`.
            let fn_count = scopes
                .iter()
                .filter(|s| matches!(s.r#type, ScopeType::Function))
                .count();
            assert_eq!(fn_count, 1);
        },
    );
}

/// TypeScript type-only scopes (`namespace X { ... }`, `interface X`,
/// `type X = ...`, mapped / conditional types) are emitted by
/// `oxc_semantic` but eslint-scope never sees them — the hand-rolled
/// walker treats their AST subtrees as type-only via
/// `unsnarl_oxc_parity::is_type_only_subtree`. The adapter must drop
/// these scopes from the IR tree so the parity harness compares like
/// for like.
#[test]
fn typescript_type_alias_scope_is_filtered_out() {
    with_scopes(
        "type X = number; const y = 0;",
        Language::Ts,
        SourceType::Module,
        |scopes| {
            // Only the module scope remains; the `TSTypeAliasDeclaration`
            // scope is dropped along with any binding inside it.
            assert_eq!(scopes.len(), 1);
            assert!(matches!(scopes[root()].r#type, ScopeType::Module));
        },
    );
}

#[test]
fn typescript_namespace_scope_is_filtered_out() {
    with_scopes(
        "namespace N { export const x = 1; } const y = 0;",
        Language::Ts,
        SourceType::Module,
        |scopes| {
            // The `TSModuleDeclaration` ("namespace N") scope is
            // dropped; the `const x` binding inside it is filtered
            // along with the surrounding subtree.
            assert_eq!(
                scopes.len(),
                1,
                "expected only the module root scope; got {} scopes",
                scopes.len(),
            );
        },
    );
}

#[test]
fn typescript_conditional_type_scope_is_filtered_out() {
    with_scopes(
        "type If<C, T, F> = C extends true ? T : F;",
        Language::Ts,
        SourceType::Module,
        |scopes| {
            // `TSTypeAliasDeclaration` + its nested `TSConditionalType`
            // both vanish from the IR scope tree.
            assert_eq!(scopes.len(), 1);
        },
    );
}

#[test]
fn typescript_interface_does_not_emit_a_scope() {
    with_scopes(
        "interface Shape { x: number; }",
        Language::Ts,
        SourceType::Module,
        |scopes| {
            assert_eq!(scopes.len(), 1);
        },
    );
}

#[test]
fn variable_scope_chains_through_nested_blocks() {
    with_scopes(
        "function f() { { { let z = 1; } } }",
        Language::Js,
        SourceType::Script,
        |scopes| {
            let fn_scope = scopes[root()].child_scopes[0];
            assert!(matches!(scopes[fn_scope].r#type, ScopeType::Function));
            let outer_block = scopes[fn_scope].child_scopes[0];
            assert!(matches!(scopes[outer_block].r#type, ScopeType::Block));
            assert!(scopes[outer_block].variable_scope == fn_scope);
            let inner_block = scopes[outer_block].child_scopes[0];
            assert!(scopes[inner_block].variable_scope == fn_scope);
        },
    );
}
