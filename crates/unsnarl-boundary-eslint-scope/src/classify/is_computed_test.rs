//! Sibling tests for `is_computed.rs`, mirroring TS
//! `ts/src/boundary/eslint-scope/classify/is-computed.test.ts`.

use unsnarl_ir::Language;

use crate::testing::analyze_source;

#[test]
fn static_member_access_skips_property_name_reference() {
    // `obj.prop` should record a reference for `obj` only.
    let r = analyze_source("obj.prop;\n", Language::Ts);
    let names: Vec<_> = r
        .arena
        .references
        .iter()
        .map(|r| r.identifier.name().to_string())
        .collect();
    assert!(names.iter().any(|n| n == "obj"));
    assert!(!names.iter().any(|n| n == "prop"));
}

#[test]
fn computed_member_access_records_expression_reference() {
    // `obj[key]` should record references for both `obj` and `key`.
    let r = analyze_source("obj[key];\n", Language::Ts);
    let names: Vec<_> = r
        .arena
        .references
        .iter()
        .map(|r| r.identifier.name().to_string())
        .collect();
    assert!(names.iter().any(|n| n == "obj"));
    assert!(names.iter().any(|n| n == "key"));
}

#[test]
fn object_literal_static_key_does_not_record_property_name_reference() {
    let r = analyze_source("const obj = { a: 1 };\n", Language::Ts);
    let names: Vec<_> = r
        .arena
        .references
        .iter()
        .map(|r| r.identifier.name().to_string())
        .collect();
    assert!(!names.iter().any(|n| n == "a"));
}

#[test]
fn object_literal_computed_key_records_expression_reference() {
    let r = analyze_source("const k = 'x'; const obj = { [k]: 1 };\n", Language::Ts);
    let names: Vec<_> = r
        .arena
        .references
        .iter()
        .map(|r| r.identifier.name().to_string())
        .collect();
    assert!(names.iter().any(|n| n == "k"));
}
