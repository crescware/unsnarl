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

/// Mirrors `spanFromOffset` in `ts/src/util/span.ts`: walks the raw
/// source up to `offset` (clamped to its byte length), counts line
/// breaks, and materialises a `Span` triple. `offset` is in bytes; the
/// returned `column` is the byte distance from the last `\n` (or from
/// the start of the file). The TS source treats characters as JS
/// UTF-16 code units, so consumers that need column-accuracy across
/// non-ASCII spans must already ensure offsets land on character
/// boundaries; the Rust port preserves the same byte-oriented
/// behavior.
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
    let column = u32::try_from(offset - last_newline.map(|n| n + 1).unwrap_or(0)).unwrap_or(0);
    let offset_u32 = u32::try_from(offset).unwrap_or(u32::MAX);
    Span {
        line: SourceLine(line),
        column: SourceColumn(column),
        offset: SourceOffset(offset_u32),
    }
}
