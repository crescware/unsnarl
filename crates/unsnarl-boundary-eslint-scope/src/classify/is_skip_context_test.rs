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
