//! Source-position triple used in the serialized IR.
//!
//! Internally the IR carries raw `SourceOffset` (or `oxc_span::Span`)
//! and the serializer converts those into a `Span` by walking line
//! breaks in the source text at emit time.
//!
//! `SourceOffset`, `SourceLine`, and `SourceColumn` are distinct
//! newtypes over `u32` so the type system catches accidental swaps
//! at construction sites (a byte offset, a line number, and a column
//! number are all 32-bit unsigned integers but mean different
//! things). Each is `#[serde(transparent)]` so the on-disk JSON
//! shape stays a bare number.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct SourceOffset(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct SourceLine(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct SourceColumn(pub u32);

#[derive(Clone, Serialize)]
pub struct Span {
    pub line: SourceLine,
    pub column: SourceColumn,
    pub offset: SourceOffset,
}

/// Mirrors `spanFromOffset` in `ts/src/util/span.ts`. The TS reference
/// reads source via `fs.readFileSync(..., "utf8")` and consumes
/// offsets produced by the npm `oxc-parser` package, both of which
/// operate in UTF-16 code units (JavaScript string indices). The Rust
/// pipeline consumes UTF-8 byte offsets from the `oxc_parser` crate;
/// this function converts those byte offsets into UTF-16 code-unit
/// offsets so the emitted IR matches the TS output byte-for-byte even
/// in sources containing non-ASCII characters.
///
/// `offset` is a UTF-8 byte offset into `raw`. The returned `offset`
/// and `column` are both in UTF-16 code units; `line` counts `\n`
/// occurrences (which are ASCII in either encoding).
pub fn span_from_offset(raw: &str, offset: usize) -> Span {
    let bytes = raw.as_bytes();
    let limit = offset.min(bytes.len());
    let mut line: u32 = 1;
    let mut last_newline: Option<usize> = None;
    for (i, b) in bytes[..limit].iter().enumerate() {
        if *b == b'\n' {
            line += 1;
            last_newline = Some(i);
        }
    }
    let line_start_byte = last_newline.map(|n| n + 1).unwrap_or(0);
    let overshoot = offset.saturating_sub(bytes.len());
    let column_utf16 = raw[line_start_byte..limit].encode_utf16().count() + overshoot;
    let offset_utf16 = raw[..limit].encode_utf16().count() + overshoot;
    Span {
        line: SourceLine(line),
        column: SourceColumn(u32::try_from(column_utf16).unwrap_or(0)),
        offset: SourceOffset(u32::try_from(offset_utf16).unwrap_or(u32::MAX)),
    }
}

#[cfg(test)]
#[path = "span_test.rs"]
mod span_test;
