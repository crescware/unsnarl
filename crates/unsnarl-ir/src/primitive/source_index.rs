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

    /// Resolve a UTF-16 code-unit offset (the encoding IR fields
    /// such as `Span::offset` and `block_context::parent_span_offset`
    /// carry) into the 1-based line number containing that position.
    /// Matches the legacy `span_from_offset` line-counting behaviour:
    /// an offset that lands exactly on a `\n` boundary is reported as
    /// still on the line *before* the newline, because the source
    /// scan stops one position short of `offset`.
    pub fn line_for_utf16_offset(&self, offset: Utf16CodeUnitOffset) -> u32 {
        let line_idx = match self.line_start_utf16.binary_search(&offset.0) {
            Ok(i) => i,
            // `Err(0)` is impossible because `line_start_utf16[0] == 0`
            // and the search target is a non-negative `u32`.
            Err(i) => i - 1,
        };
        (line_idx + 1) as u32
    }

    /// Slice `raw` between two UTF-16 code-unit offsets (the encoding
    /// IR offsets carry), returning the substring as a `&str`.
    ///
    /// `start..end` semantics: if `start >= end` the result is empty;
    /// if either bound falls past the end of `raw` it is clamped to
    /// the file end. Bounds on the same line short-circuit through
    /// the `line_is_ascii` flag so an ASCII region is resolved with
    /// two `Vec` lookups and no per-character UTF-16 stepping.
    pub fn slice_utf16(&self, start: Utf16CodeUnitOffset, end: Utf16CodeUnitOffset) -> &'a str {
        if start.0 >= end.0 {
            return "";
        }
        let start_byte = self.utf16_to_byte(start);
        let end_byte = self.utf16_to_byte(end);
        &self.raw[start_byte..end_byte]
    }

    /// Translate a UTF-16 code-unit offset to its UTF-8 byte position
    /// inside `raw`. Clamped to `raw.len()` when the offset overshoots
    /// the end of the source.
    fn utf16_to_byte(&self, offset: Utf16CodeUnitOffset) -> usize {
        let target = offset.0;
        let line_idx = match self.line_start_utf16.binary_search(&target) {
            Ok(i) => i,
            // `Err(0)` is impossible because `line_start_utf16[0] == 0`
            // and `target` is a non-negative `u32`.
            Err(i) => i - 1,
        };
        let line_start_byte = self.line_starts[line_idx] as usize;
        let line_start_utf16 = self.line_start_utf16[line_idx];
        let within_utf16 = target - line_start_utf16;
        let next_line_byte = self
            .line_starts
            .get(line_idx + 1)
            .copied()
            .map(|b| b as usize)
            .unwrap_or(self.raw.len());
        if self.line_is_ascii[line_idx] {
            // ASCII line: 1 byte == 1 UTF-16 code unit, so the offset
            // within the line is the same in both encodings. Clamp to
            // the line's byte length to keep an out-of-range target
            // (e.g. a synthesised end_offset that runs past EOF) from
            // returning a position past the file.
            let clamped = (within_utf16 as usize).min(next_line_byte - line_start_byte);
            return line_start_byte + clamped;
        }
        // Non-ASCII line: walk it once, stopping when we have consumed
        // `within_utf16` UTF-16 code units. `len_utf16` is 1 for BMP
        // characters and 2 for supplementary-plane characters
        // (surrogate pair); `len_utf8` is the matching byte count.
        let line_str = &self.raw[line_start_byte..next_line_byte];
        let mut consumed: u32 = 0;
        let mut byte_offset = line_start_byte;
        for ch in line_str.chars() {
            if consumed >= within_utf16 {
                break;
            }
            consumed += ch.len_utf16() as u32;
            byte_offset += ch.len_utf8();
        }
        byte_offset
    }
}

#[cfg(test)]
#[path = "source_index_test.rs"]
mod source_index_test;
