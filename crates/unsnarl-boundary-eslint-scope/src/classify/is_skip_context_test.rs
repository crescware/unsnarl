//! Sibling tests for `is_skip_context.rs`, mirroring TS
//! `ts/src/boundary/eslint-scope/classify/is-skip-context.test.ts`.

use unsnarl_ir::Language;

use crate::testing::analyze_source;

fn ref_names(r: &crate::analysis_result::EslintScopeAnalysisResult) -> Vec<String> {
    r.arena
        .references
        .iter()
        .map(|r| r.identifier.name().to_string())
        .collect()
}

#[test]
fn static_member_property_name_is_skipped() {
    let r = analyze_source("obj.prop;\n", Language::Ts);
    assert!(!ref_names(&r).iter().any(|n| n == "prop"));
}

#[test]
fn object_literal_static_key_is_skipped() {
    let r = analyze_source("const obj = { key: 1 };\n", Language::Ts);
    assert!(!ref_names(&r).iter().any(|n| n == "key"));
}

#[test]
fn export_specifier_exported_name_is_skipped() {
    // `export { foo as bar }` — `bar` is the exported alias and is a
    // string/IdentifierName, not a reference.
    let r = analyze_source("const foo = 1; export { foo as bar };\n", Language::Ts);
    assert!(!ref_names(&r).iter().any(|n| n == "bar"));
}

#[test]
fn labeled_statement_label_is_skipped() {
    let r = analyze_source(
        "outer: for (let i = 0; i < 1; i++) { break outer; }\n",
        Language::Ts,
    );
    // `outer` here is a LabelIdentifier, not an IdentifierReference.
    assert!(!ref_names(&r).iter().any(|n| n == "outer"));
}

#[test]
fn jsx_closing_element_name_is_skipped() {
    // `const Foo = () => null; <Foo>{bar}</Foo>;` produces exactly
    // two `Foo` references: the init write reference for `const Foo`
    // (the `let x = 1` shape -- a `BindingIdentifier` in a
    // `VariableDeclarator.id` with an `init`) and the JSX opening
    // element's read reference. The JSX *closing* element name must
    // be skipped by the `JSXClosingElement` arm; if it were not,
    // the count would jump to three.
    let r = analyze_source("const Foo = () => null; <Foo>{bar}</Foo>;\n", Language::Tsx);
    let foo_refs = ref_names(&r).iter().filter(|n| *n == "Foo").count();
    assert_eq!(
        foo_refs, 2,
        "JSXClosingElement name must be skipped so `Foo` is referenced exactly twice (init write + opening read)"
    );
    assert!(
        ref_names(&r).iter().any(|n| n == "bar"),
        "the JSX child expression `{{bar}}` must still record a reference"
    );
}

#[test]
fn jsx_attribute_name_is_not_recorded_as_reference() {
    // `<Foo onClick={handleClick} />` — `onClick` is a `JSXIdentifier`
    // sitting in `JSXAttribute.name` (not an `IdentifierReference`),
    // so the typed AST already filters it before `classify_identifier`
    // ever runs. `handleClick` lives in the attribute's
    // `JSXExpressionContainer` value and must still resolve.
    let r = analyze_source(
        "const handleClick = () => {}; const Foo = () => null; <Foo onClick={handleClick} />;\n",
        Language::Tsx,
    );
    let names = ref_names(&r);
    assert!(!names.iter().any(|n| n == "onClick"));
    assert!(names.iter().any(|n| n == "handleClick"));
    assert!(names.iter().any(|n| n == "Foo"));
}

#[test]
fn jsx_member_expression_property_is_not_recorded_as_reference() {
    // `<Foo.Bar />` — `Bar` is a `JSXIdentifier` sitting in
    // `JSXMemberExpression.property` (not an `IdentifierReference`),
    // so it never reaches `visit_identifier_reference`. `Foo` (the
    // member-expression object) IS an `IdentifierReference` and must
    // be recorded.
    let r = analyze_source(
        "const Foo = { Bar: () => null }; <Foo.Bar />;\n",
        Language::Tsx,
    );
    let names = ref_names(&r);
    assert!(!names.iter().any(|n| n == "Bar"));
    assert!(names.iter().any(|n| n == "Foo"));
}
