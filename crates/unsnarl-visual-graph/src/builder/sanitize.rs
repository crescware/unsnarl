//! `sanitize`: replace every non-alphanumeric / non-underscore
//! character with `_`.
//!
//! Mirrors `ts/src/visual-graph/builder/sanitize.ts`. The TS
//! implementation runs `value.replace(/[^a-zA-Z0-9_]/g, "_")` which
//! steps through UTF-16 code units one at a time. The Rust port
//! mirrors that exactly via [`str::encode_utf16`] so multi-byte
//! / surrogate-pair characters produce the same number of `_`
//! characters as in TS.

pub fn sanitize(value: &str) -> String {
    value
        .encode_utf16()
        .map(|u| {
            if u < 128 {
                let c = u as u8 as char;
                if c.is_ascii_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
#[path = "sanitize_test.rs"]
mod sanitize_test;
