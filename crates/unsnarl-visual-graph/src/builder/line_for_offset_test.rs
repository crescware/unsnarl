//! Sibling tests for [`line_for_offset`]. Cases lock the basic loop
//! semantics: line counting, offset clamping, and UTF-16 stepping.

use unsnarl_ir::primitive::SourceIndex;

use super::line_for_offset;

fn idx(raw: &str) -> SourceIndex<'_> {
    SourceIndex::build(raw)
}

#[test]
fn empty_source_returns_line_one() {
    assert_eq!(line_for_offset(&idx(""), 0), 1);
    assert_eq!(line_for_offset(&idx(""), 5), 1);
}

#[test]
fn offset_zero_is_line_one() {
    assert_eq!(line_for_offset(&idx("a\nb\nc"), 0), 1);
}

#[test]
fn offset_after_first_newline_is_line_two() {
    let src = "abc\ndef";
    // 'd' sits at offset 4.
    assert_eq!(line_for_offset(&idx(src), 4), 2);
}

#[test]
fn offset_at_newline_itself_is_still_previous_line() {
    let src = "abc\ndef";
    // offset 3 == position of '\n'; the previous behaviour returned
    // the line "before" the newline, and `SourceIndex::span_at` keeps
    // that semantics.
    assert_eq!(line_for_offset(&idx(src), 3), 1);
}

#[test]
fn offset_past_end_clamps_to_full_count() {
    let src = "a\nb\nc";
    assert_eq!(line_for_offset(&idx(src), 100), 3);
}

#[test]
fn utf16_stepping_matches_js_string_indexing() {
    // The emoji 😊 (U+1F60A) is a surrogate pair (2 UTF-16 code
    // units). A newline before it lives at code-unit index 1, so
    // offset=3 (one past the emoji) is still on line 2.
    let src = "a\n\u{1F60A}b";
    assert_eq!(line_for_offset(&idx(src), 3), 2);
}
