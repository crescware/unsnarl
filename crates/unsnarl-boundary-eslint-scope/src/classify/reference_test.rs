//! Sibling tests for `reference.rs`.
//!
//! The TS `reference(flags, init)` factory is exercised inside
//! `classify_*` paths. The Rust port keeps the function for parity
//! with the TS source layout; its observable effect is the flag
//! shape on `ReferenceData`, which we check here via a couple of
//! representative scenarios.

use unsnarl_ir::reference::reference_flags::ReferenceFlags;
use unsnarl_ir::Language;

use crate::testing::analyze_source;

#[test]
fn plain_read_reference_has_read_flag_and_no_init() {
    let r = analyze_source("const x = 1; x;\n", Language::Ts);
    let any_pure_read = r.arena.references.iter().any(|r| {
        r.identifier.name() == "x"
            && !r.init
            && (r.flags & ReferenceFlags::READ).0 != 0
            && (r.flags & ReferenceFlags::WRITE).0 == 0
    });
    assert!(any_pure_read);
}

#[test]
fn init_reference_carries_init_flag() {
    let r = analyze_source("const x = 1; const y = x;\n", Language::Ts);
    let init_ref = r
        .arena
        .references
        .iter()
        .find(|r| r.identifier.name() == "x" && r.init);
    assert!(init_ref.is_some());
}
