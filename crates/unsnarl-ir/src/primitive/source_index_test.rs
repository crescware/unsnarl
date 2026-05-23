use super::*;
use crate::primitive::offset::{Utf16CodeUnitOffset, Utf8ByteOffset};
use crate::primitive::span::span_from_offset;

fn assert_matches_span_from_offset(raw: &str, offset: Utf8ByteOffset) {
    let expected = span_from_offset(raw, offset);
    let actual = SourceIndex::build(raw).span_at(offset);
    assert_eq!(
        actual.line, expected.line,
        "line mismatch at offset {}",
        offset.0
    );
    assert_eq!(
        actual.column, expected.column,
        "column mismatch at offset {}",
        offset.0
    );
    assert_eq!(
        actual.offset, expected.offset,
        "offset mismatch at offset {}",
        offset.0
    );
}

#[test]
fn ascii_only_offset_matches_byte_offset() {
    assert_matches_span_from_offset("let x = 1;\nlet y = 2;\n", Utf8ByteOffset(22));
}

#[test]
fn em_dash_in_preceding_comment_shifts_offset_by_two_codeunits() {
    let raw = "/* — */\n};\n";
    assert_matches_span_from_offset(raw, Utf8ByteOffset(raw.len() as u32));
}

#[test]
fn section_sign_shifts_offset_by_one_codeunit() {
    let raw = "// §6.2.4\nx;\n";
    assert_matches_span_from_offset(raw, Utf8ByteOffset(raw.len() as u32));
}

#[test]
fn supplementary_codepoint_contributes_two_utf16_units() {
    let raw = "/* 😀 */\nx;\n";
    assert_matches_span_from_offset(raw, Utf8ByteOffset(raw.len() as u32));
}

#[test]
fn column_is_in_utf16_codeunits_relative_to_line_start() {
    let raw = "—x";
    assert_matches_span_from_offset(raw, Utf8ByteOffset(raw.len() as u32));
}

#[test]
fn non_ascii_before_newline_does_not_affect_later_line_column() {
    let raw = "// —\nlet x = 1;";
    assert_matches_span_from_offset(raw, Utf8ByteOffset(raw.len() as u32));
}

#[test]
fn offset_past_file_end_extends_with_clamp_plus_overshoot() {
    assert_matches_span_from_offset("ab", Utf8ByteOffset(5));
}

#[test]
fn empty_source() {
    assert_matches_span_from_offset("", Utf8ByteOffset(0));
}

#[test]
fn every_byte_boundary_matches_span_from_offset_ascii() {
    let raw = "let x = 1;\nlet y = 2;\nfn z() { return x + y; }\n";
    for i in 0..=raw.len() as u32 + 3 {
        assert_matches_span_from_offset(raw, Utf8ByteOffset(i));
    }
}

#[test]
fn every_char_boundary_matches_span_from_offset_mixed() {
    let raw = "// —\nlet 😀 = 1;\n// §6\nz;";
    for (i, _) in raw.char_indices() {
        assert_matches_span_from_offset(raw, Utf8ByteOffset(i as u32));
    }
    assert_matches_span_from_offset(raw, Utf8ByteOffset(raw.len() as u32));
    assert_matches_span_from_offset(raw, Utf8ByteOffset(raw.len() as u32 + 1));
}

#[test]
fn newline_at_end_of_file_emits_trailing_line() {
    let raw = "a\nb\n";
    assert_matches_span_from_offset(raw, Utf8ByteOffset(raw.len() as u32));
}

#[test]
fn line_for_utf16_offset_returns_one_at_start_of_file() {
    let index = SourceIndex::build("abc\ndef\nghi");
    assert_eq!(index.line_for_utf16_offset(Utf16CodeUnitOffset(0)), 1);
}

#[test]
fn line_for_utf16_offset_returns_previous_line_when_offset_lands_on_newline() {
    let index = SourceIndex::build("abc\ndef");
    // Offset 3 sits on the `\n` byte; the scan loop in
    // `span_from_offset` stops before consuming it, so the answer is
    // still line 1.
    assert_eq!(index.line_for_utf16_offset(Utf16CodeUnitOffset(3)), 1);
}

#[test]
fn line_for_utf16_offset_advances_to_next_line_one_unit_past_newline() {
    let index = SourceIndex::build("abc\ndef");
    // Offset 4 is the first code unit of line 2 (`d`).
    assert_eq!(index.line_for_utf16_offset(Utf16CodeUnitOffset(4)), 2);
}

#[test]
fn line_for_utf16_offset_counts_in_utf16_units_not_utf8_bytes() {
    // The em-dash `—` is 3 UTF-8 bytes / 1 UTF-16 code unit. After
    // it, a UTF-16 offset of 2 is the position right before the
    // `\n` at UTF-16 code unit 2 (still line 1); offset 3 is the
    // first code unit of line 2.
    let index = SourceIndex::build("—a\nb");
    assert_eq!(index.line_for_utf16_offset(Utf16CodeUnitOffset(2)), 1);
    assert_eq!(index.line_for_utf16_offset(Utf16CodeUnitOffset(3)), 2);
}

#[test]
fn line_for_utf16_offset_clamps_past_end_of_file() {
    let index = SourceIndex::build("a\nb\nc");
    // The source has 3 lines; any offset past the last line start
    // (UTF-16 code unit 4 = `c`) stays on line 3.
    assert_eq!(index.line_for_utf16_offset(Utf16CodeUnitOffset(100)), 3);
}

#[test]
fn slice_utf16_returns_empty_when_start_is_at_or_past_end() {
    let index = SourceIndex::build("abc");
    assert_eq!(
        index.slice_utf16(Utf16CodeUnitOffset(0), Utf16CodeUnitOffset(0)),
        ""
    );
    assert_eq!(
        index.slice_utf16(Utf16CodeUnitOffset(2), Utf16CodeUnitOffset(2)),
        ""
    );
    assert_eq!(
        index.slice_utf16(Utf16CodeUnitOffset(3), Utf16CodeUnitOffset(1)),
        ""
    );
}

#[test]
fn slice_utf16_on_ascii_only_source_matches_byte_slice() {
    let raw = "let x = 1;\nlet y = 2;";
    let index = SourceIndex::build(raw);
    assert_eq!(
        index.slice_utf16(Utf16CodeUnitOffset(4), Utf16CodeUnitOffset(9)),
        "x = 1"
    );
    assert_eq!(
        index.slice_utf16(
            Utf16CodeUnitOffset(0),
            Utf16CodeUnitOffset(raw.len() as u32)
        ),
        raw
    );
}

#[test]
fn slice_utf16_spans_multiple_lines() {
    let raw = "abc\ndef\nghi";
    let index = SourceIndex::build(raw);
    // From UTF-16 unit 2 (the `c` on line 1) to unit 9 (the `h` on
    // line 3) — covers a newline and the entirety of line 2.
    assert_eq!(
        index.slice_utf16(Utf16CodeUnitOffset(2), Utf16CodeUnitOffset(9)),
        "c\ndef\ng"
    );
}

#[test]
fn slice_utf16_handles_non_ascii_inside_a_line() {
    // `—` is 3 UTF-8 bytes / 1 UTF-16 code unit. Slicing
    // UTF-16 [0, 2) over `—a` should give `—a`.
    let raw = "—a";
    let index = SourceIndex::build(raw);
    assert_eq!(
        index.slice_utf16(Utf16CodeUnitOffset(0), Utf16CodeUnitOffset(2)),
        "—a"
    );
    assert_eq!(
        index.slice_utf16(Utf16CodeUnitOffset(1), Utf16CodeUnitOffset(2)),
        "a"
    );
}

#[test]
fn slice_utf16_handles_surrogate_pair() {
    // `😀` (U+1F600) is 4 UTF-8 bytes / 2 UTF-16 code units. Slicing
    // UTF-16 [0, 2) extracts the whole surrogate pair.
    let raw = "😀x";
    let index = SourceIndex::build(raw);
    assert_eq!(
        index.slice_utf16(Utf16CodeUnitOffset(0), Utf16CodeUnitOffset(2)),
        "😀"
    );
    assert_eq!(
        index.slice_utf16(Utf16CodeUnitOffset(0), Utf16CodeUnitOffset(3)),
        "😀x"
    );
}

#[test]
fn slice_utf16_clamps_offsets_past_end_of_file() {
    let raw = "abc";
    let index = SourceIndex::build(raw);
    assert_eq!(
        index.slice_utf16(Utf16CodeUnitOffset(1), Utf16CodeUnitOffset(100)),
        "bc"
    );
}

#[test]
fn line_for_utf16_offset_matches_span_at_line_for_ascii_only_source() {
    // For ASCII input UTF-8 byte offsets and UTF-16 code-unit offsets
    // coincide, so the two APIs should agree on every position.
    let raw = "let x = 1;\nlet y = 2;\nfn z() { return x + y; }\n";
    let index = SourceIndex::build(raw);
    for i in 0..=raw.len() as u32 + 3 {
        let from_byte = index.span_at(Utf8ByteOffset(i)).line.0;
        let from_utf16 = index.line_for_utf16_offset(Utf16CodeUnitOffset(i));
        assert_eq!(
            from_byte, from_utf16,
            "mismatch at offset {i}: span_at={from_byte} line_for_utf16_offset={from_utf16}"
        );
    }
}
