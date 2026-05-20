//! Mirrors `ts/src/visual-graph/builder/line-for-offset.ts`.
//!
//! TS counts UTF-16 code units up to `offset`, scanning for `\n`
//! (code unit 10). The IR contract carries `offset` as a UTF-16
//! code unit count (see `unsnarl_ir::primitive::span_from_offset`),
//! so the same UTF-16 stepping in Rust keeps the result aligned
//! with both the IR side tables and the npm `oxc-parser` baseline.

pub fn line_for_offset(raw: &str, offset: u32) -> u32 {
    let offset = offset as usize;
    let mut line: u32 = 1;
    for (consumed, unit) in raw.encode_utf16().enumerate() {
        if consumed >= offset {
            break;
        }
        if unit == 10 {
            line += 1;
        }
    }
    line
}

#[cfg(test)]
#[path = "line_for_offset_test.rs"]
mod line_for_offset_test;
