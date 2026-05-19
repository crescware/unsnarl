//! Sibling tests for `classify_ordinary_reference.rs`, mirroring TS
//! `ts/src/boundary/eslint-scope/classify/classify-ordinary-reference.test.ts`.

use unsnarl_ir::reference::reference_flags::ReferenceFlags;
use unsnarl_ir::Language;

use crate::testing::analyze_source;

fn x_has_read_write(r: &crate::analysis_result::EslintScopeAnalysisResult) -> bool {
    let x_id = r.arena.scopes[r.global_scope]
        .set()
        .get("x")
        .copied()
        .unwrap();
    r.arena.variables[x_id].references.iter().any(|&r_id| {
        let f = r.arena.references[r_id].flags;
        (f & ReferenceFlags::READ).0 != 0 && (f & ReferenceFlags::WRITE).0 != 0
    })
}

#[test]
fn compound_assignment_classified_as_read_write() {
    let r = analyze_source("let x = 1; x += 2;\n", Language::Ts);
    assert!(x_has_read_write(&r));
}

#[test]
fn update_expression_classified_as_read_write() {
    let r = analyze_source("let x = 1; x++;\n", Language::Ts);
    assert!(x_has_read_write(&r));
}

#[test]
fn simple_assignment_left_classified_as_write_only() {
    let r = analyze_source("let x = 1; x = 2;\n", Language::Ts);
    let x_id = r.arena.scopes[r.global_scope]
        .set()
        .get("x")
        .copied()
        .unwrap();
    let has_write_only = r.arena.variables[x_id].references.iter().any(|&r_id| {
        let f = r.arena.references[r_id].flags;
        (f & ReferenceFlags::WRITE).0 != 0 && (f & ReferenceFlags::READ).0 == 0
    });
    assert!(has_write_only);
}
