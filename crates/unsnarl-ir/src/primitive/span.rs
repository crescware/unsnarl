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

#[derive(Serialize)]
pub struct Span {
    pub line: SourceLine,
    pub column: SourceColumn,
    pub offset: SourceOffset,
}
