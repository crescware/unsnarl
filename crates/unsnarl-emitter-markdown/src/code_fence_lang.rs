//! Pick the code-fence language tag for the `## Input` block.
//!
//! Every recognised `Language` round-trips to its short form (`tsx`
//! / `jsx` / `js` / `ts`). `Language::Ts` is listed explicitly so
//! the default branch could be reintroduced (pointing at `"ts"`) if
//! `Language` grows new variants later without changing output for
//! existing cases.

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
