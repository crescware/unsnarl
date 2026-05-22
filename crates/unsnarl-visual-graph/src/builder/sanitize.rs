//! `sanitize`: replace every non-alphanumeric / non-underscore
//! character with `_`.
//!
//! Steps through UTF-16 code units via [`str::encode_utf16`] so the
//! number of `_` characters emitted for multi-byte / surrogate-pair
//! characters matches what the regex `value.replace(/[^a-zA-Z0-9_]/g, "_")`
//! would produce in a JavaScript string.

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
