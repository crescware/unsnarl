use super::*;
use crate::generation_count::GenerationCount;

fn assert_err_contains(text: &str, needle: &str) {
    let err = parse_direction_token(text).unwrap_err();
    let msg = &err[0].message;
    assert!(
        msg.contains(needle),
        "expected error for {text:?} to contain {needle:?}, got {msg:?}",
    );
}

#[test]
fn parses_bare_direction_with_level_none() {
    for (input, dir) in [
        ("+a", Direction::After),
        ("+b", Direction::Before),
        ("+c", Direction::Context),
    ] {
        assert_eq!(
            parse_direction_token(input),
            Ok(ParsedDirectionToken { dir, level: None }),
        );
    }
}

#[test]
fn parses_direction_with_explicit_level() {
    for (input, dir, level) in [
        ("+a3", Direction::After, 3),
        ("+b10", Direction::Before, 10),
        ("+c1", Direction::Context, 1),
    ] {
        assert_eq!(
            parse_direction_token(input),
            Ok(ParsedDirectionToken {
                dir,
                level: Some(GenerationCount(level)),
            }),
        );
    }
}

#[test]
fn syntactically_accepts_level_zero() {
    assert_eq!(
        parse_direction_token("+a0"),
        Ok(ParsedDirectionToken {
            dir: Direction::After,
            level: Some(GenerationCount(0)),
        }),
    );
}

#[test]
fn rejects_bare_plus_or_empty() {
    assert_err_contains("+", "unexpected direction token");
    assert_err_contains("", "unexpected direction token");
}

#[test]
fn rejects_unknown_direction_letters() {
    assert_err_contains("+x", "unexpected direction token");
    assert_err_contains("+d", "unexpected direction token");
}

#[test]
fn rejects_multi_letter_directions() {
    assert_err_contains("+ab", "unexpected direction token");
    assert_err_contains("+aa", "unexpected direction token");
}

#[test]
fn rejects_trailing_garbage_after_digits() {
    assert_err_contains("+a3b", "unexpected direction token");
    assert_err_contains("+a-3", "unexpected direction token");
}

#[test]
fn rejects_direction_like_text_without_plus() {
    assert_err_contains("a", "unexpected direction token");
    assert_err_contains("a3", "unexpected direction token");
}

// `u32::from_str("+5")` returns `Ok(5)`, so without the byte-level
// `is_ascii_digit` guard `+a+5` would silently parse to level 5. Lock in
// the guard so a future "drop the redundant pre-scan" refactor cannot
// regress this case.
#[test]
fn rejects_signed_level_suffix() {
    assert_err_contains("+a+5", "unexpected direction token");
    assert_err_contains("+a-5", "unexpected direction token");
}

#[test]
fn rejects_u32_overflow_level() {
    assert_err_contains("+a4294967296", "unexpected direction token");
}

#[test]
fn syntactically_accepts_u32_max_level() {
    assert_eq!(
        parse_direction_token("+a4294967295"),
        Ok(ParsedDirectionToken {
            dir: Direction::After,
            level: Some(GenerationCount(u32::MAX)),
        }),
    );
}
