//! Path-extension → [`Language`] / [`SourceType`] mapping helpers.

use std::path::Path;

use unsnarl_ir::Language;
use unsnarl_oxc_boundary::parser::{default_source_type_for, SourceType};

/// Map a path's extension to a [`Language`]. `.mjs` / `.cjs` map to
/// `Js` because they are JavaScript at the parser level;
/// module-vs-script is resolved separately via
/// [`source_type_from_path`].
pub fn language_for_path(path: &str) -> Option<Language> {
    let ext = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    match ext {
        "ts" => Some(Language::Ts),
        "tsx" => Some(Language::Tsx),
        "jsx" => Some(Language::Jsx),
        "js" | "mjs" | "cjs" => Some(Language::Js),
        _ => None,
    }
}

/// `.mjs` / `.cjs` are spec-pinned to module / script; every other
/// extension falls back to the language-level default.
pub fn source_type_from_path(path: &str, language: Language) -> SourceType {
    if path.ends_with(".mjs") {
        return SourceType::Module;
    }
    if path.ends_with(".cjs") {
        return SourceType::Script;
    }
    default_source_type_for(language)
}
