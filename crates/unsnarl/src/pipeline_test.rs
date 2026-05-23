//! Sibling tests for [`super::pipeline`].
//!
//! Covers `source_type_from_path` and `language_for_path`. The
//! other pipeline entry points (`emit_*_text` / `emit_*_detailed`)
//! are driven end-to-end by the integration tests under
//! `crates/unsnarl/tests/parity.rs`, so they do not need duplicate
//! sibling coverage here.

use unsnarl_ir::Language;
use unsnarl_oxc_boundary::parser::SourceType;

use super::{language_for_path, source_type_from_path};

#[test]
fn mjs_extension_is_module_regardless_of_language_tag() {
    assert!(matches!(
        source_type_from_path("foo.mjs", Language::Js),
        SourceType::Module
    ));
}

#[test]
fn cjs_extension_is_script_regardless_of_language_tag() {
    assert!(matches!(
        source_type_from_path("foo.cjs", Language::Js),
        SourceType::Script
    ));
}

#[test]
fn nested_paths_still_inspect_the_trailing_suffix() {
    assert!(matches!(
        source_type_from_path("src/deep/foo.mjs", Language::Js),
        SourceType::Module
    ));
    assert!(matches!(
        source_type_from_path("src/deep/foo.cjs", Language::Js),
        SourceType::Script
    ));
}

#[test]
fn js_extension_falls_back_to_default_source_type_for_script() {
    assert!(matches!(
        source_type_from_path("foo.js", Language::Js),
        SourceType::Script
    ));
}

#[test]
fn ts_extension_falls_back_to_default_source_type_for_module() {
    assert!(matches!(
        source_type_from_path("foo.ts", Language::Ts),
        SourceType::Module
    ));
}

#[test]
fn tsx_extension_falls_back_to_default_source_type_for_module() {
    assert!(matches!(
        source_type_from_path("foo.tsx", Language::Tsx),
        SourceType::Module
    ));
}

#[test]
fn jsx_extension_falls_back_to_default_source_type_for_module() {
    assert!(matches!(
        source_type_from_path("foo.jsx", Language::Jsx),
        SourceType::Module
    ));
}

// [`language_for_path`] returns an `Option<Language>` and is the
// seam the CLI uses to map a positional file path to a parser
// language tag. The cases below cover the supported extension set
// (`.ts/.tsx/.js/.jsx/.mjs/.cjs`); the unsupported-extension path is
// surfaced as an error in `read_source_file` rather than mapped to
// any default language, so unknown extensions must yield `None`.

#[test]
fn language_for_path_tsx_extension_yields_tsx() {
    assert!(matches!(
        language_for_path("Component.tsx"),
        Some(Language::Tsx)
    ));
}

#[test]
fn language_for_path_jsx_extension_yields_jsx() {
    assert!(matches!(
        language_for_path("Component.jsx"),
        Some(Language::Jsx)
    ));
}

#[test]
fn language_for_path_js_extension_yields_js() {
    assert!(matches!(language_for_path("foo.js"), Some(Language::Js)));
}

#[test]
fn language_for_path_mjs_extension_yields_js() {
    assert!(matches!(language_for_path("foo.mjs"), Some(Language::Js)));
}

#[test]
fn language_for_path_cjs_extension_yields_js() {
    assert!(matches!(language_for_path("foo.cjs"), Some(Language::Js)));
}

#[test]
fn language_for_path_ts_extension_yields_ts() {
    assert!(matches!(language_for_path("foo.ts"), Some(Language::Ts)));
}

#[test]
fn language_for_path_nested_paths_inspect_trailing_suffix() {
    assert!(matches!(
        language_for_path("src/deep/Component.tsx"),
        Some(Language::Tsx)
    ));
}

#[test]
fn language_for_path_unknown_extension_yields_none() {
    // The CLI converts this `None` into an "unsupported language
    // extension" error rather than silently defaulting to `ts`, so
    // unknown extensions must not be re-routed back to a language.
    assert!(language_for_path("Makefile").is_none());
    assert!(language_for_path("foo.bin").is_none());
}
