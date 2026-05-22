use super::*;

#[test]
fn ascii_only_offset_matches_byte_offset() {
    let raw = "let x = 1;\nlet y = 2;\n";
    let span = span_from_offset(raw, raw.len());
    assert_eq!(span.line, SourceLine(3));
    assert_eq!(span.column, SourceColumn(0));
    assert_eq!(span.offset, SourceOffset(22));
}

#[test]
fn em_dash_in_preceding_comment_shifts_offset_by_two_codeunits() {
    // `—` is 3 UTF-8 bytes / 1 UTF-16 code unit, so a span end past
    // it should land 2 UTF-16 units below its UTF-8 byte position.
    // Regression shape: a file whose only non-ASCII is an em-dash
    // inside a comment, where the expected `endSpan.offset` is
    // `byte_len(file) - 2`.
    let raw = "/* — */\n};\n";
    let byte_len = raw.len();
    assert_eq!(byte_len, 13);
    let span = span_from_offset(raw, byte_len);
    assert_eq!(span.line, SourceLine(3));
    assert_eq!(span.column, SourceColumn(0));
    assert_eq!(span.offset, SourceOffset((byte_len - 2) as u32));
}

#[test]
fn section_sign_shifts_offset_by_one_codeunit() {
    // `§` is 2 UTF-8 bytes / 1 UTF-16 code unit.
    let raw = "// §6.2.4\nx;\n";
    let byte_len = raw.len();
    let span = span_from_offset(raw, byte_len);
    assert_eq!(span.offset, SourceOffset((byte_len - 1) as u32));
}

#[test]
fn supplementary_codepoint_contributes_two_utf16_units() {
    // `😀` (U+1F600) is 4 UTF-8 bytes / 2 UTF-16 code units (surrogate
    // pair). UTF-16 offset shifts by `bytes - codeunits = 4 - 2 = 2`.
    let raw = "/* 😀 */\nx;\n";
    let byte_len = raw.len();
    let span = span_from_offset(raw, byte_len);
    assert_eq!(span.offset, SourceOffset((byte_len - 2) as u32));
}

#[test]
fn column_is_in_utf16_codeunits_relative_to_line_start() {
    // Non-ASCII appears on the same line as the target offset; the
    // column must be measured in UTF-16 code units, not UTF-8 bytes.
    let raw = "—x";
    // Byte offset at end of file is 4 (3 for `—`, 1 for `x`); UTF-16
    // column is 2 (1 for `—`, 1 for `x`).
    let span = span_from_offset(raw, raw.len());
    assert_eq!(span.line, SourceLine(1));
    assert_eq!(span.column, SourceColumn(2));
    assert_eq!(span.offset, SourceOffset(2));
}

#[test]
fn non_ascii_before_newline_does_not_affect_later_line_column() {
    // After a `\n`, the column resets and is counted in UTF-16 units
    // from the newline. The non-ASCII char on a prior line still
    // affects the absolute `offset` but not the `column`.
    let raw = "// —\nlet x = 1;";
    let byte_offset = raw.len();
    let span = span_from_offset(raw, byte_offset);
    assert_eq!(span.line, SourceLine(2));
    assert_eq!(span.column, SourceColumn(10));
    // UTF-16 offset = 5 (line 1: `// — \n` is 5 code units) + 10 = 15.
    assert_eq!(span.offset, SourceOffset(15));
}

#[test]
fn offset_past_file_end_extends_with_clamp_plus_overshoot() {
    // Out-of-range offsets are tolerated: the in-range portion is
    // converted to UTF-16 and the overshoot is added 1:1 to both
    // `offset` and `column`.
    let raw = "ab";
    let span = span_from_offset(raw, 5);
    assert_eq!(span.offset, SourceOffset(5));
    assert_eq!(span.column, SourceColumn(5));
}

#[test]
fn empty_source() {
    let span = span_from_offset("", 0);
    assert_eq!(span.line, SourceLine(1));
    assert_eq!(span.column, SourceColumn(0));
    assert_eq!(span.offset, SourceOffset(0));
}
