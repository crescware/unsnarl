use std::collections::BTreeSet;

use super::{AST_TYPE_ENUM_VARIANTS, OXC_TYPES_DTS_ENTRIES, OXC_UNKNOWN_ALIASES};

fn require_oxc_types_dts(test_name: &str) -> bool {
    if !OXC_TYPES_DTS_ENTRIES.is_empty() {
        return true;
    }
    if std::env::var("CI").is_ok() {
        panic!(
            "{test_name}: types.d.ts not found but CI requires it \
             — run `pnpm install` in fixtures/oxc-parity/"
        );
    }
    eprintln!(
        "skipping {test_name}: types.d.ts not found \
         (run `pnpm install` in fixtures/oxc-parity/)"
    );
    false
}

#[test]
fn alias_expansions_cover_every_alias_in_types_dts() {
    if !require_oxc_types_dts("alias_expansions_cover_every_alias_in_types_dts") {
        return;
    }
    assert!(
        OXC_UNKNOWN_ALIASES.is_empty(),
        "types.d.ts contains unknown type-alias discriminators \
         not listed in build.rs OXC_TYPE_ALIAS_EXPANSIONS: {OXC_UNKNOWN_ALIASES:?}",
    );
}

#[test]
fn ast_type_contains_no_entries_that_oxc_does_not_emit() {
    if !require_oxc_types_dts("ast_type_contains_no_entries_that_oxc_does_not_emit") {
        return;
    }
    let oxc: BTreeSet<&str> = OXC_TYPES_DTS_ENTRIES.iter().copied().collect();
    let extra: Vec<&str> = AST_TYPE_ENUM_VARIANTS
        .iter()
        .copied()
        .filter(|v| !oxc.contains(v) && *v != "UnknownAstType")
        .collect();
    assert!(
        extra.is_empty(),
        "AstType contains entries that types.d.ts does not declare: {extra:?}",
    );
}

#[test]
fn ast_type_lists_every_type_string_oxc_emits() {
    if !require_oxc_types_dts("ast_type_lists_every_type_string_oxc_emits") {
        return;
    }
    let declared: BTreeSet<&str> = AST_TYPE_ENUM_VARIANTS.iter().copied().collect();
    let missing: Vec<&str> = OXC_TYPES_DTS_ENTRIES
        .iter()
        .copied()
        .filter(|v| !declared.contains(v))
        .collect();
    assert!(
        missing.is_empty(),
        "types.d.ts declares types that AstType does not list: {missing:?}",
    );
}
