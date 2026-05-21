//! Sibling tests for [`super`]. Ports
//! `ts/src/cli/run-cli/resolve-plugin.test.ts`, which exercises the
//! per-name resolution layer the CLI calls before handing the
//! activated list to the pipeline. The TS port loads each plugin
//! through dynamic `import()`; the Rust port resolves through the
//! compile-time [`default_registry`]. The two surfaces are otherwise
//! equivalent: a known short name resolves to the bundled plugin
//! instance, and an unknown short name produces a
//! `Plugin 'unsnarl-plugin-<name>' is not bundled with this unsnarl build.`
//! error.
//!
//! TS `validate-plugin-module.test.ts` (the "module has no default
//! export" failure mode) intentionally has no Rust counterpart:
//! [`default_registry`] is constructed in code, so a missing
//! registration is a build-time error, not a runtime one. Porting
//! that test would assert against a Rust state that cannot occur.

use super::{activate, default_registry};

#[test]
fn activate_resolves_a_bundled_plugin_to_its_unsnarl_plugin_name() {
    let registry = default_registry();
    let names = vec!["react".to_string()];
    let activated = activate(&registry, &names).expect("react should resolve");
    assert_eq!(activated.len(), 1);
    assert_eq!(activated[0].name(), "unsnarl-plugin-react");
}

#[test]
fn activate_rejects_an_unbundled_plugin_with_a_human_readable_message() {
    let registry = default_registry();
    let names = vec!["nonexistent-xyz".to_string()];
    let err = match activate(&registry, &names) {
        Ok(_) => panic!("missing plugin must error"),
        Err(e) => e,
    };
    assert_eq!(err.name(), "nonexistent-xyz");
    assert_eq!(
        format!("{err}"),
        "Plugin 'unsnarl-plugin-nonexistent-xyz' is not bundled with this unsnarl build."
    );
}
