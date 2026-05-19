//! Source-offset → `Span` (`line` / `column` / `offset`) conversion.
//!
//! Mirrors `spanFromOffset` in `ts/src/util/span.ts`. The TS version
//! lives under `util/`; the Rust port is colocated inside the boundary
//! crate because it is currently consumed only here (the
//! `var-detected` diagnostic in `handle_variable_declaration`). It
//! can be lifted into a shared crate when a second consumer
//! materialises.

use unsnarl_ir::primitive::{SourceColumn, SourceLine, SourceOffset, Span};

pub(crate) fn span_from_offset(raw: &str, offset: usize) -> Span {
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
