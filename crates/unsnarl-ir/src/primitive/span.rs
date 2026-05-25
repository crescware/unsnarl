//! Source-position triple used in the serialized IR.
//!
//! Internally the IR carries raw [`Utf8ByteOffset`] (or
//! `oxc_span::Span`) and the serializer converts those into a `Span`
//! by walking line breaks in the source text at emit time.
//!
//! [`SourceLine`] and [`SourceColumn`] are distinct newtypes over
//! `u32` so the type system catches accidental swaps at construction
//! sites. The `offset` field is a [`Utf16CodeUnitOffset`], i.e. the
//! IR carries UTF-16 code-unit offsets so the on-disk JSON matches
//! JavaScript-string indexing semantics. Each numeric field is
//! `#[serde(transparent)]` so the JSON shape stays a bare number.

use serde::Serialize;

use super::offset::{Utf16CodeUnitOffset, Utf8ByteOffset};

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
    pub offset: Utf16CodeUnitOffset,
}

impl Span {
    pub fn new(line: SourceLine, column: SourceColumn, offset: Utf16CodeUnitOffset) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }
}

/// The `oxc_parser` crate produces UTF-8 byte offsets, but the
/// emitted IR carries UTF-16 code-unit offsets so the on-disk shape
/// matches JavaScript-string indexing semantics. This function
/// performs the conversion so the IR stays consistent even in
/// sources containing non-ASCII characters.
///
/// `offset` is a [`Utf8ByteOffset`] into `raw`. The returned `offset`
/// and `column` are both UTF-16 code units; `line` counts `\n`
/// occurrences (which are ASCII in either encoding).
pub fn span_from_offset(raw: &str, offset: Utf8ByteOffset) -> Span {
    let bytes = raw.as_bytes();
    let offset_usize = offset.0 as usize;
    let limit = offset_usize.min(bytes.len());
    let mut line: u32 = 1;
    let mut last_newline: Option<usize> = None;
    for (i, b) in bytes[..limit].iter().enumerate() {
        if *b == b'\n' {
            line += 1;
            last_newline = Some(i);
        }
    }
    let line_start_byte = last_newline.map(|n| n + 1).unwrap_or(0);
    let overshoot = offset_usize.saturating_sub(bytes.len());
    let column_utf16 = raw[line_start_byte..limit].encode_utf16().count() + overshoot;
    let offset_utf16 = raw[..limit].encode_utf16().count() + overshoot;
    Span::new(
        SourceLine(line),
        SourceColumn(u32::try_from(column_utf16).unwrap_or(0)),
        Utf16CodeUnitOffset(u32::try_from(offset_utf16).unwrap_or(u32::MAX)),
    )
}

#[cfg(test)]
#[path = "span_test.rs"]
mod span_test;
