//! Pre-computed line-start index over a source string.
//!
//! `span_from_offset` walks the prefix of `raw` once per call, which
//! costs O(file_size) per lookup; on a 587 KB JavaScript bundle the
//! serializer issued tens of thousands of calls and spent ~40 s
//! exclusively inside that prefix walk. `SourceIndex` pre-walks the
//! source once and answers each subsequent query in
//! O(log lines + line_length), and short-circuits ASCII-only lines so
//! the per-line scan does not invoke the UTF-16 iterator at all.
//!
//! The result of `span_at` is byte-for-byte identical to
//! `span_from_offset(raw, offset)` for every offset, including
//! offsets past the end of the source (the same "clamp plus
//! overshoot" semantics apply).

use crate::primitive::offset::{Utf16CodeUnitOffset, Utf8ByteOffset};
use crate::primitive::span::{SourceColumn, SourceLine, Span};

pub struct SourceIndex<'a> {
    raw: &'a str,
    // Byte offset where each line begins. `line_starts[0]` is always 0;
    // entry `i` corresponds to line `i + 1` in 1-based numbering.
    line_starts: Vec<u32>,
    // Cumulative UTF-16 code-unit offset at each line's start byte.
    line_start_utf16: Vec<u32>,
    // `true` iff every byte from this line's start up to (but not
    // including) the next line's start is ASCII. Lets `span_at`
    // compute the UTF-16 column as a subtraction instead of walking
    // the line.
    line_is_ascii: Vec<bool>,
}

impl<'a> SourceIndex<'a> {
    pub fn build(raw: &'a str) -> Self {
        let approx_lines = raw.len() / 40 + 1;
        let mut line_starts: Vec<u32> = Vec::with_capacity(approx_lines);
        let mut line_start_utf16: Vec<u32> = Vec::with_capacity(approx_lines);
        let mut line_is_ascii: Vec<bool> = Vec::with_capacity(approx_lines);
        line_starts.push(0);
        line_start_utf16.push(0);

        let mut current_utf16: u32 = 0;
        let mut line_ascii = true;

        for (i, ch) in raw.char_indices() {
            if !ch.is_ascii() {
                line_ascii = false;
            }
            current_utf16 += ch.len_utf16() as u32;
            if ch == '\n' {
                line_is_ascii.push(line_ascii);
                let next_byte = (i + 1) as u32;
                let next_utf16 = line_start_utf16[line_start_utf16.len() - 1] + current_utf16;
                line_starts.push(next_byte);
                line_start_utf16.push(next_utf16);
                current_utf16 = 0;
                line_ascii = true;
            }
        }
        line_is_ascii.push(line_ascii);

        Self {
            raw,
            line_starts,
            line_start_utf16,
            line_is_ascii,
        }
    }

    pub fn raw(&self) -> &'a str {
        self.raw
    }

    /// Resolve a UTF-8 byte offset (the encoding `oxc_parser` produces)
    /// into a [`Span`] whose `offset` / `column` are UTF-16 code units.
    pub fn span_at(&self, offset: Utf8ByteOffset) -> Span {
        let byte_len = self.raw.len();
        let offset_usize = offset.0 as usize;
        let clamped = offset_usize.min(byte_len);
        let overshoot = (offset_usize - clamped) as u32;

        let line_idx = match self.line_starts.binary_search(&(clamped as u32)) {
            Ok(i) => i,
            // `Err(0)` is impossible because `line_starts[0] == 0 <= clamped`.
            Err(i) => i - 1,
        };
        let line_start_byte = self.line_starts[line_idx] as usize;
        let line_start_utf16 = self.line_start_utf16[line_idx];
        let line_ascii = self.line_is_ascii[line_idx];

        let column_utf16: u32 = if line_ascii {
            (clamped - line_start_byte) as u32
        } else {
            self.raw[line_start_byte..clamped].encode_utf16().count() as u32
        };

        Span {
            line: SourceLine((line_idx + 1) as u32),
            column: SourceColumn(column_utf16 + overshoot),
            offset: Utf16CodeUnitOffset(line_start_utf16 + column_utf16 + overshoot),
        }
    }
}

#[cfg(test)]
#[path = "source_index_test.rs"]
mod source_index_test;
