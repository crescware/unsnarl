//! Characterization tests pinning `oxc_semantic = 0.128.0`'s observed
//! behavior on the four open issues called out in
//! <https://github.com/crescware/unsnarl/issues/190>:
//!
//! 1. implicit `arguments` binding
//! 2. JSXIdentifier / IdentifierName references
//! 3. `with` body resolution
//! 4. `Scoping`'s `'a` lifetime shape
//!
//! These tests are not parity tests — they do **not** assert that
//! `oxc_semantic` matches the hand-rolled eslint-scope behavior. Phase 2
//! reads them as ground truth about what `oxc_semantic` actually does,
//! so the adapter can be designed accordingly (e.g. synthesise an
//! `arguments` binding adapter-side if oxc_semantic doesn't, post-process
//! `with`-body references if their resolution diverges, etc.).

use oxc_allocator::Allocator;
use oxc_semantic::{Scoping, SemanticBuilder};

use unsnarl_ir::Language;

use crate::parser::{OxcParser, ParseOptions, SourceType};

/// Single-use scaffold: parse `code` as `language` and run
/// `SemanticBuilder::new().build`, then hand `Scoping` to the caller.
///
/// Closure form because `Scoping` borrows from the allocator; the
/// closure encloses the lifetime cleanly.
fn with_scoping(code: &str, language: Language, body: impl FnOnce(&Scoping)) {
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
                source_type: SourceType::Script,
            },
        )
        .expect("test source must parse cleanly");
    let ret = SemanticBuilder::new().build(&parsed.program);
    body(ret.semantic.scoping());
}

/// (1) Does `oxc_semantic` materialise the implicit per-function
/// `arguments` binding that eslint-scope creates?
///
/// eslint-scope inserts an `arguments` `Variable` (with zero `defs`)
/// into every non-arrow function's local scope. We need to know
/// whether `oxc_semantic` does the same.
#[test]
fn arguments_is_or_is_not_a_symbol_inside_a_function() {
    with_scoping("function f() { return 0; }", Language::Js, |scoping| {
        let names: Vec<&str> = scoping.symbol_names().collect();
        let has_arguments = names.contains(&"arguments");
        // Pinned: `oxc_semantic` does NOT register `arguments` as a
        // symbol. The function body's `arguments` references are
        // expected to surface as unresolved references that the
        // consumer must interpret specially. Phase 2 adapter must
        // synthesise the `arguments` `Variable` itself.
        assert!(
            !has_arguments,
            "expected no `arguments` symbol but found one (names = {names:?})",
        );
    });
}

/// (2a) Does an in-JSX `{x}` expression produce a resolved reference
/// against the outer `x` binding?
#[test]
fn jsx_expression_container_identifier_resolves_to_outer_binding() {
    with_scoping(
        "const x = 1; const _ = <div>{x}</div>;",
        Language::Jsx,
        |scoping| {
            // The `x` binding sits in the global scope (module / script
            // root). Walk the symbols to find its id, then ask whether
            // any reference resolves to it.
            let mut x_symbol = None;
            for sid in scoping.symbol_ids() {
                if scoping.symbol_name(sid) == "x" {
                    x_symbol = Some(sid);
                    break;
                }
            }
            let x_symbol = x_symbol.expect("`x` symbol must exist");
            let resolved = scoping.get_resolved_reference_ids(x_symbol);
            // Pinned: the JSX-expression `{x}` does produce a reference
            // and oxc_semantic resolves it. If this assertion ever flips,
            // Phase 2's adapter needs to compensate.
            assert!(
                !resolved.is_empty(),
                "expected `{{x}}` to produce a resolved reference against `x`",
            );
        },
    );
}

/// (2b) Does `oxc_semantic` produce references for the **tag name**
/// of `<MyComp />` (the `JSXIdentifier`)?
///
/// eslint-scope picks up the tag name as a reference resolving to
/// an imported / declared `MyComp`. Phase 2 needs to know whether
/// the adapter has to synthesise this lookup itself.
#[test]
fn jsx_tag_identifier_resolves_to_outer_binding() {
    with_scoping(
        "const MyComp = () => null; const _ = <MyComp />;",
        Language::Jsx,
        |scoping| {
            let mut comp_symbol = None;
            for sid in scoping.symbol_ids() {
                if scoping.symbol_name(sid) == "MyComp" {
                    comp_symbol = Some(sid);
                    break;
                }
            }
            let comp_symbol = comp_symbol.expect("`MyComp` symbol must exist");
            let resolved = scoping.get_resolved_reference_ids(comp_symbol);
            assert!(
                !resolved.is_empty(),
                "expected `<MyComp />` tag to produce a resolved reference against `MyComp`",
            );
        },
    );
}

/// (3) Inside a `with (o) { x; }` block, eslint-scope deliberately
/// leaves `x` unresolved (the `with` extends the scope chain with
/// `o` at runtime, so static resolution is impossible). What does
/// `oxc_semantic` do?
#[test]
fn with_body_identifier_resolves_to_outer_binding_diverging_from_eslint_scope() {
    with_scoping(
        // `with` is illegal in strict mode, so use a sloppy-mode
        // script. The boundary's `OxcParser` honors `SourceType::Script`.
        "var x = 0; var o = {}; with (o) { x; }",
        Language::Js,
        |scoping| {
            // Find the outer `x` binding and check whether the `x`
            // inside the `with` block resolves to it.
            let mut x_symbol = None;
            for sid in scoping.symbol_ids() {
                if scoping.symbol_name(sid) == "x" {
                    x_symbol = Some(sid);
                    break;
                }
            }
            let x_symbol = x_symbol.expect("`x` symbol must exist");
            let resolved = scoping.get_resolved_reference_ids(x_symbol);
            // Pinned: `oxc_semantic` resolves the inside-`with` `x`
            // against the outer binding — different from eslint-scope,
            // which deliberately leaves it unresolved because `with`
            // can introduce shadowing bindings from `o` at runtime.
            //
            // Note also that `var x = 0` does NOT produce a write
            // reference in `oxc_semantic`; the declaration carries the
            // init directly. So `get_resolved_reference_ids(x)` returns
            // exactly one entry, and that entry is the inside-`with`
            // read (Read flag, no Write).
            //
            // Phase 2 takeaway: parity with eslint-scope's "do not
            // resolve under `with`" semantics is not free — the
            // adapter must either (a) walk the AST to detect references
            // located inside a `with` body and unlink their
            // `resolved` field, or (b) accept the diff and update any
            // affected `expected.*` baselines.
            assert_eq!(
                resolved.len(),
                1,
                "expected exactly one resolved reference for outer `x` (got {})",
                resolved.len(),
            );
            let ref_id = resolved[0];
            let r = scoping.get_reference(ref_id);
            assert!(
                r.flags().is_read() && !r.flags().is_write(),
                "expected the inside-`with` `x` reference to be Read (got {:?})",
                r.flags(),
            );
        },
    );
}

/// (4) The shape of the `'a` lifetime on `Scoping`.
///
/// This is a compile-time observation, not a runtime assertion: if
/// `Scoping` had a lifetime parameter borrowed from the program /
/// allocator, the closure-based `with_scoping` helper above wouldn't
/// type-check the way it does (it passes a `&Scoping` without any
/// AST-tied lifetime). The fact that `with_scoping` works and is
/// callable from any signature pins `Scoping` as **owning its data**
/// (`Scoping`, not `Scoping<'a>`), which `Semantic::into_scoping`
/// also confirms — the boundary `analyze()` signature in Phase 2 does
/// not need to surface a lifetime parameter from `Scoping`.
#[test]
fn scoping_has_no_lifetime_parameter() {
    fn requires_no_lifetime(_: Scoping) {}
    let allocator = Allocator::default();
    let parsed = OxcParser
        .parse(
            &allocator,
            "let x = 1;",
            &ParseOptions {
                language: Language::Js,
                source_path: "input.js".to_string(),
                source_type: SourceType::Script,
            },
        )
        .expect("trivial source must parse");
    let scoping = SemanticBuilder::new()
        .build(&parsed.program)
        .semantic
        .into_scoping();
    requires_no_lifetime(scoping);
}
