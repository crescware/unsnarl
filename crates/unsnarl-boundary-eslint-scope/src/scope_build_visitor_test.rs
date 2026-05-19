//! Sibling tests for `scope_build_visitor.rs`.
//!
//! Collapses the TS test surface for `handle-enter.test.ts`,
//! `handle-leave.test.ts`, `walk/walk.test.ts`, and
//! `eslint-compat.test.ts` because the Rust walker subsumes all four
//! into one `ScopeBuildVisitor` (each TS module's `case` arm is now
//! a `visit_*` override on this struct).

use oxc_allocator::Allocator;
use unsnarl_ir::ids::{ReferenceId, ScopeId};
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::{IrArena, Language};
use unsnarl_oxc_parity::AstType;

use crate::analyze::{analyze, AnalyzeOptions};
use crate::parser::{default_source_type_for, OxcParser, ParseOptions};
use crate::state::ScopeBuilderState;
use crate::testing::analyze_source;
use crate::visitor::AnalysisVisitor;

fn variable_names(arena: &IrArena) -> Vec<String> {
    arena
        .variables
        .iter()
        .map(|v| v.name().to_string())
        .collect()
}

fn reference_identifier_names(arena: &IrArena) -> Vec<String> {
    arena
        .references
        .iter()
        .map(|r| r.identifier.name().to_string())
        .collect()
}

#[test]
fn walker_descends_through_nested_blocks_and_pops_correctly() {
    // `{{{}}}` produces three nested Block scopes; the walker must
    // both push and pop each one. If pop_scope is off, the
    // `current_scope` panics on the next push.
    let r = analyze_source("{ { { let inner = 1; } } }\n", Language::Ts);
    let mut depth = 0;
    let mut cur = r.arena.scopes[r.global_scope].child_scopes.first().copied();
    while let Some(s) = cur {
        if !matches!(r.arena.scopes[s].r#type, ScopeType::Block) {
            break;
        }
        depth += 1;
        cur = r.arena.scopes[s].child_scopes.first().copied();
    }
    assert_eq!(depth, 3);
}

#[test]
fn walker_visits_every_scope_only_once() {
    // A nontrivial source must not double-create any scope; child
    // counts at each level are the obvious sanity check.
    let r = analyze_source("function outer() { function inner() {} }\n", Language::Ts);
    let outer = r.arena.scopes[r.global_scope].child_scopes.clone();
    assert_eq!(
        outer.len(),
        1,
        "exactly one direct child for `function outer`"
    );
    let outer_scope = outer[0];
    let inner = r.arena.scopes[outer_scope].child_scopes.clone();
    assert_eq!(inner.len(), 1, "exactly one inner function scope");
}

#[test]
fn eslint_compat_module_scope_chain_terminates_at_module_root() {
    let r = analyze_source("export const x = 1;\n", Language::Ts);
    // Module root has no upper.
    assert!(r.arena.scopes[r.global_scope].upper.is_none());
}

#[test]
fn export_named_declaration_routes_declaration_slot_key_to_inner_class_scope() {
    // Parity regression: the npm `oxc-parser` package's visitorKeys
    // list `["declaration", "specifiers", "source", "attributes"]` for
    // `ExportNamedDeclaration`, so the TS reference fires `on_scope`
    // for an `export class Foo {}` inner class scope with `key =
    // Some("declaration")`. Without an explicit override, oxc's
    // auto-generated walker leaks whatever the surrounding
    // statement-list pushed (typically `Some("body")` from
    // `Program.body`).
    #[derive(Default)]
    struct Capture {
        rows: Vec<(AstType, Option<AstType>, Option<String>)>,
    }
    impl AnalysisVisitor for Capture {
        fn on_scope(
            &mut self,
            scope_id: ScopeId,
            parent: Option<&AstNode>,
            key: Option<&str>,
            _path: &[AstNode],
            state: &ScopeBuilderState,
        ) {
            let block_type = state.arena.scopes[scope_id].block.r#type.clone();
            self.rows.push((
                block_type,
                parent.map(|p| p.r#type.clone()),
                key.map(str::to_string),
            ));
        }
        fn on_reference(
            &mut self,
            _ref_id: ReferenceId,
            _parent: Option<&AstNode>,
            _key: Option<&str>,
            _path: &[AstNode],
            _scope_id: ScopeId,
            _state: &ScopeBuilderState,
        ) {
        }
    }

    let allocator = Allocator::default();
    let parsed = OxcParser
        .parse(
            &allocator,
            "export class Foo {}\n",
            &ParseOptions {
                language: Language::Ts,
                source_path: "input.ts".to_string(),
                source_type: default_source_type_for(Language::Ts),
            },
        )
        .expect("parse");
    let mut visitor = Capture::default();
    analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            raw: parsed.raw,
        },
        &mut visitor,
    );
    let class_row = visitor
        .rows
        .iter()
        .find(|(block_type, _, _)| matches!(block_type, AstType::ClassDeclaration))
        .expect("class scope must fire on_scope");
    assert!(
        matches!(class_row.1, Some(AstType::ExportNamedDeclaration)),
        "class scope's parent must be ExportNamedDeclaration"
    );
    assert_eq!(
        class_row.2.as_deref(),
        Some("declaration"),
        "class scope's slot key must be \"declaration\", not the inherited \"body\""
    );
}

#[test]
fn parameter_default_immediate_identifier_does_not_register_as_runtime_reference() {
    // `function f(b = a) { ... }` -- TS classifies `a` (the direct
    // child of the parameter's initializer slot) as a binding, not a
    // reference, because in its ESTree shape the parent is
    // `AssignmentPattern` (a pattern step). oxc instead stores this
    // as `FormalParameter { pattern: b, initializer: a }`, so we
    // must short-circuit the classify walk at
    // `(parent=FormalParameter, key="initializer")` to produce the
    // same outcome.
    let r = analyze_source("function f(a = 0, b = a) { return a + b; }\n", Language::Ts);
    let refs = reference_identifier_names(&r.arena);
    // Body references: `a` (in `a + b`), `b` (in `a + b`). The
    // initializer-direct `a` (in `b = a`) must not be tracked.
    assert_eq!(refs, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn parameter_default_nested_identifier_still_registers_as_runtime_reference() {
    // `function f(c = a + b)` -- `a` and `b` sit inside a
    // BinaryExpression nested in the initializer, so their immediate
    // parent is the BinaryExpression (not `FormalParameter`). The
    // classify fast-path for FormalParameter must NOT fire; they
    // should remain ordinary references.
    let r = analyze_source("function f(c = a + b) { return c; }\n", Language::Ts);
    let refs = reference_identifier_names(&r.arena);
    // Order: parameter-default `a`, then `b`, then body `c`.
    assert_eq!(
        refs,
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
}

#[test]
fn ts_parameter_property_identifier_still_classifies_as_reference() {
    // `constructor(public x: number)` -- the TS parameter-property
    // case still falls through to `classify_ordinary_reference`,
    // producing a read reference resolved against the implicit
    // globals scope. The new initializer-slot fast-path must not
    // apply (`key="params"`, not `"initializer"`).
    let r = analyze_source(
        "class C {\n  constructor(public x: number) {}\n}\n",
        Language::Ts,
    );
    let refs = reference_identifier_names(&r.arena);
    assert!(
        refs.iter().any(|n| n == "x"),
        "TS parameter property `public x` must still register `x` as a runtime reference; got {refs:?}"
    );
}

#[test]
fn ts_as_const_does_not_register_const_as_runtime_reference() {
    // Parity regression: oxc's auto-generated `walk_ts_as_expression`
    // descends into `type_annotation` without recording the
    // `typeAnnotation` slot key, so the `const` identifier inside
    // `as const` (a TS literal-type marker, not a runtime binding)
    // would slip through `is_type_only_subtree` and be classified as
    // a global implicit reference. After the fix the type subtree is
    // entered with `key = Some("typeAnnotation")`, `type_only_depth`
    // increments, and the identifier is skipped entirely.
    let r = analyze_source("export const X = { a: 1 } as const;\n", Language::Ts);
    assert!(
        !variable_names(&r.arena).iter().any(|n| n == "const"),
        "`as const` must not register a `const` variable; got {:?}",
        variable_names(&r.arena)
    );
    assert!(
        !reference_identifier_names(&r.arena)
            .iter()
            .any(|n| n == "const"),
        "`as const` must not register a `const` reference; got {:?}",
        reference_identifier_names(&r.arena)
    );
}

#[test]
fn ts_as_named_type_does_not_register_type_name_as_runtime_reference() {
    // `as UnsnarlPlugin` -- the type name is a TS-only reference and
    // must not appear in `arena.references`.
    let r = analyze_source(
        "type T = number;\nconst x: unknown = 0;\nconst y = x as T;\n",
        Language::Ts,
    );
    let refs = reference_identifier_names(&r.arena);
    assert!(
        !refs.iter().any(|n| n == "T"),
        "`as T` must not register `T` as a runtime reference; got {refs:?}"
    );
}

#[test]
fn ts_satisfies_does_not_register_type_name_as_runtime_reference() {
    let r = analyze_source(
        "type T = { a: number };\nconst y = { a: 1 } satisfies T;\n",
        Language::Ts,
    );
    let refs = reference_identifier_names(&r.arena);
    assert!(
        !refs.iter().any(|n| n == "T"),
        "`satisfies T` must not register `T` as a runtime reference; got {refs:?}"
    );
}

#[test]
fn ts_legacy_type_assertion_does_not_register_type_name_as_runtime_reference() {
    let r = analyze_source(
        "type T = number;\nconst x: unknown = 0;\nconst y = <T>x;\n",
        Language::Ts,
    );
    let refs = reference_identifier_names(&r.arena);
    assert!(
        !refs.iter().any(|n| n == "T"),
        "`<T>x` must not register `T` as a runtime reference; got {refs:?}"
    );
}

#[test]
fn property_definition_type_annotation_does_not_register_named_type_as_runtime_reference() {
    // `class C { items: Diagnostic[] = []; }` -- `Diagnostic` is a
    // TS-only type reference and must not appear in
    // `arena.references`. This matches the parity-bench failures in
    // `src/util/diagnostic.ts` / `src/parser/parse-error.ts` where
    // class property type annotations were leaking their named types
    // as extra runtime references.
    let r = analyze_source(
        "type Diagnostic = { message: string };\nclass C {\n  items: Diagnostic[] = [];\n}\n",
        Language::Ts,
    );
    let refs = reference_identifier_names(&r.arena);
    assert!(
        !refs.iter().any(|n| n == "Diagnostic"),
        "class property type annotation must not register `Diagnostic` as a runtime reference; got {refs:?}"
    );
}

#[test]
fn accessor_property_type_annotation_does_not_register_named_type_as_runtime_reference() {
    let r = analyze_source(
        "type Tag = string;\nclass C {\n  accessor name: Tag = \"x\";\n}\n",
        Language::Ts,
    );
    let refs = reference_identifier_names(&r.arena);
    assert!(
        !refs.iter().any(|n| n == "Tag"),
        "accessor-property type annotation must not register `Tag` as a runtime reference; got {refs:?}"
    );
}

#[test]
fn call_expression_type_argument_is_not_registered_as_runtime_reference() {
    // `f<Tag>(x)` -- the `Tag` is a TS-only type argument and must
    // not appear in `arena.references`.
    let r = analyze_source(
        "type Tag = number;\nfunction f<U>(x: U): U { return x; }\nconst y = f<Tag>(1);\n",
        Language::Ts,
    );
    let refs = reference_identifier_names(&r.arena);
    assert!(
        !refs.iter().any(|n| n == "Tag"),
        "`f<Tag>(x)`'s type argument must not become a runtime reference; got {refs:?}"
    );
}

#[test]
fn new_expression_type_argument_is_not_registered_as_runtime_reference() {
    // `new Set<Item>(arr)` -- same as above for the `new` form. This
    // matches the parity-bench failure shape in
    // `src/visual-graph/prune/name-query-excluded.ts`, where the
    // generic type argument on a `new Set<NodeKind>` call was being
    // mis-emitted as an extra `Reference` row.
    let r = analyze_source(
        "type Item = number;\nconst s = new Set<Item>([1, 2]);\n",
        Language::Ts,
    );
    let refs = reference_identifier_names(&r.arena);
    assert!(
        !refs.iter().any(|n| n == "Item"),
        "`new Set<Item>(...)`'s type argument must not become a runtime reference; got {refs:?}"
    );
}

#[test]
fn tagged_template_type_argument_is_not_registered_as_runtime_reference() {
    let r = analyze_source(
        "type T = number;\ndeclare function tag<U>(s: TemplateStringsArray): U;\nconst v = tag<T>`hello`;\n",
        Language::Ts,
    );
    let refs = reference_identifier_names(&r.arena);
    assert!(
        !refs.iter().any(|n| n == "T"),
        "tag<T>`...`'s type argument must not become a runtime reference; got {refs:?}"
    );
}

#[test]
fn jsx_opening_element_type_argument_is_not_registered_as_runtime_reference() {
    // `<Foo<Item>/>` -- TSX generic-component invocation. The `Item`
    // type argument must not become a runtime reference.
    let r = analyze_source(
        "type Item = number;\nfunction Foo<U>(_p: { x: U }) { return null; }\nconst node = <Foo<Item> x={1} />;\n",
        Language::Tsx,
    );
    let refs = reference_identifier_names(&r.arena);
    assert!(
        !refs.iter().any(|n| n == "Item"),
        "`<Foo<Item>/>`'s type argument must not become a runtime reference; got {refs:?}"
    );
}

#[test]
fn ts_instantiation_expression_does_not_register_type_argument_as_runtime_reference() {
    // `f<T>` -- the type arguments are TS-only and must not appear
    // in `arena.references`.
    let r = analyze_source(
        "type T = number;\nfunction f<U>(x: U): U { return x; }\nconst g = f<T>;\n",
        Language::Ts,
    );
    let refs = reference_identifier_names(&r.arena);
    assert!(
        !refs.iter().any(|n| n == "T"),
        "`f<T>` must not register the type argument `T` as a runtime reference; got {refs:?}"
    );
}
