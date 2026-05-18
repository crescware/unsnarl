//! Source-position triple used in the serialized IR.
//!
//! Internally the IR carries raw `u32` offsets (or `oxc_span::Span`)
//! and the serializer converts those into a `Span` by walking line
//! breaks in the source text at emit time.

use serde::Serialize;

#[derive(Serialize)]
pub struct Span {
    pub line: u32,
    pub column: u32,
    pub offset: u32,
}
