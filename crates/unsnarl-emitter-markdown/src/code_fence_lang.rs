//! Pick the code-fence language tag for the `## Input` block.
//!
//! Mirrors `ts/src/emitter/markdown/code-fence-lang.ts`. Every
//! recognised `Language` round-trips to its short form (`tsx` / `jsx`
//! / `js`); the TS port falls back to `ts` for the default branch, so
//! the Rust port lists `Language::Ts` explicitly and keeps a `_`
//! arm pointing at the same literal to preserve byte-for-byte
//! equivalence if `Language` grows new variants later.

use unsnarl_ir::language::Language;

pub fn code_fence_lang(language: Language) -> &'static str {
    match language {
        Language::Tsx => "tsx",
        Language::Jsx => "jsx",
        Language::Js => "js",
        Language::Ts => "ts",
    }
}

#[cfg(test)]
#[path = "code_fence_lang_test.rs"]
mod code_fence_lang_test;
