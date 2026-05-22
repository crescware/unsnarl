use super::*;
use crate::primitive::span::span_from_offset;

fn assert_matches_span_from_offset(raw: &str, offset: usize) {
    let expected = span_from_offset(raw, offset);
    let actual = SourceIndex::build(raw).span_at(offset);
    assert_eq!(
        actual.line, expected.line,
        "line mismatch at offset {offset}"
    );
    assert_eq!(
        actual.column, expected.column,
        "column mismatch at offset {offset}"
    );
    assert_eq!(
        actual.offset, expected.offset,
        "offset mismatch at offset {offset}"
    );
}

#[test]
fn ascii_only_offset_matches_byte_offset() {
    assert_matches_span_from_offset("let x = 1;\nlet y = 2;\n", 22);
}

#[test]
fn em_dash_in_preceding_comment_shifts_offset_by_two_codeunits() {
    let raw = "/* — */\n};\n";
    assert_matches_span_from_offset(raw, raw.len());
}

#[test]
fn section_sign_shifts_offset_by_one_codeunit() {
    let raw = "// §6.2.4\nx;\n";
    assert_matches_span_from_offset(raw, raw.len());
}

#[test]
fn supplementary_codepoint_contributes_two_utf16_units() {
    let raw = "/* 😀 */\nx;\n";
    assert_matches_span_from_offset(raw, raw.len());
}

#[test]
fn column_is_in_utf16_codeunits_relative_to_line_start() {
    let raw = "—x";
    assert_matches_span_from_offset(raw, raw.len());
}

#[test]
fn non_ascii_before_newline_does_not_affect_later_line_column() {
    let raw = "// —\nlet x = 1;";
    assert_matches_span_from_offset(raw, raw.len());
}

#[test]
fn offset_past_file_end_extends_with_clamp_plus_overshoot() {
    assert_matches_span_from_offset("ab", 5);
}

#[test]
fn empty_source() {
    assert_matches_span_from_offset("", 0);
}

#[test]
fn every_byte_boundary_matches_span_from_offset_ascii() {
    let raw = "let x = 1;\nlet y = 2;\nfn z() { return x + y; }\n";
    for i in 0..=raw.len() + 3 {
        assert_matches_span_from_offset(raw, i);
    }
}

#[test]
fn every_char_boundary_matches_span_from_offset_mixed() {
    let raw = "// —\nlet 😀 = 1;\n// §6\nz;";
    for (i, _) in raw.char_indices() {
        assert_matches_span_from_offset(raw, i);
    }
    assert_matches_span_from_offset(raw, raw.len());
    assert_matches_span_from_offset(raw, raw.len() + 1);
}

#[test]
fn newline_at_end_of_file_emits_trailing_line() {
    let raw = "a\nb\n";
    assert_matches_span_from_offset(raw, raw.len());
}
