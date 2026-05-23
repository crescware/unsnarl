//! Sibling tests for [`line_for_offset`]. Cases lock the basic
//! line-counting semantics on top of `SourceIndex`, including the
//! "offset on a `\n` reports the previous line" detail and UTF-16
//! stepping for surrogate-pair characters.

use unsnarl_ir::primitive::{SourceIndex, Utf16CodeUnitOffset};

use super::line_for_offset;

fn idx(raw: &str) -> SourceIndex<'_> {
    SourceIndex::build(raw)
}

#[test]
fn empty_source_returns_line_one() {
    assert_eq!(line_for_offset(&idx(""), Utf16CodeUnitOffset(0)), 1);
    assert_eq!(line_for_offset(&idx(""), Utf16CodeUnitOffset(5)), 1);
}

#[test]
fn offset_zero_is_line_one() {
    assert_eq!(line_for_offset(&idx("a\nb\nc"), Utf16CodeUnitOffset(0)), 1);
}

#[test]
fn offset_after_first_newline_is_line_two() {
    let src = "abc\ndef";
    // 'd' sits at offset 4.
    assert_eq!(line_for_offset(&idx(src), Utf16CodeUnitOffset(4)), 2);
}

#[test]
fn offset_at_newline_itself_is_still_previous_line() {
    let src = "abc\ndef";
    // offset 3 == position of '\n'; the search stops one short of
    // the newline so the line returned is the one *before*.
    assert_eq!(line_for_offset(&idx(src), Utf16CodeUnitOffset(3)), 1);
}

#[test]
fn offset_past_end_clamps_to_full_count() {
    let src = "a\nb\nc";
    assert_eq!(line_for_offset(&idx(src), Utf16CodeUnitOffset(100)), 3);
}

#[test]
fn utf16_stepping_matches_js_string_indexing() {
    // The emoji 😊 (U+1F60A) is a surrogate pair (2 UTF-16 code
    // units). A newline before it lives at code-unit index 1, so
    // offset=3 (one past the emoji) is still on line 2.
    let src = "a\n\u{1F60A}b";
    assert_eq!(line_for_offset(&idx(src), Utf16CodeUnitOffset(3)), 2);
}
