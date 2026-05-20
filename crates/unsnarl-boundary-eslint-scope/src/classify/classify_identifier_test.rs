//! Sibling tests for `classify_identifier.rs`, mirroring TS
//! `ts/src/boundary/eslint-scope/classify/classify-identifier.test.ts`.

use unsnarl_ir::reference::reference_flags::ReferenceFlags;
use unsnarl_ir::Language;

use crate::testing::analyze_source;

#[test]
fn assignment_left_classified_as_write_reference() {
    let r = analyze_source("let x = 1; x = 2;\n", Language::Ts);
    let x_id = r.arena.scopes[r.global_scope]
        .set()
        .get("x")
        .copied()
        .unwrap();
    let has_write = r.arena.variables[x_id]
        .references
        .iter()
        .any(|&r_id| (r.arena.references[r_id].flags & ReferenceFlags::WRITE).0 != 0);
    assert!(has_write);
}

#[test]
fn variable_declarator_init_classified_as_init_read() {
    let r = analyze_source("const x = 1; const y = x;\n", Language::Ts);
    let init_refs: Vec<_> = r
        .arena
        .references
        .iter()
        .filter(|r| r.init && r.identifier.name() == "x")
        .collect();
    assert!(!init_refs.is_empty());
}

#[test]
fn plain_identifier_reads_classified_as_read() {
    let r = analyze_source("const x = 1; x;\n", Language::Ts);
    let x_id = r.arena.scopes[r.global_scope]
        .set()
        .get("x")
        .copied()
        .unwrap();
    let has_pure_read = r.arena.variables[x_id].references.iter().any(|&r_id| {
        let f = r.arena.references[r_id].flags;
        (f & ReferenceFlags::READ).0 != 0 && (f & ReferenceFlags::WRITE).0 == 0
    });
    assert!(has_pure_read);
}
