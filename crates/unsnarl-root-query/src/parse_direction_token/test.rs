use super::*;

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
        ("+a", Direction::A),
        ("+b", Direction::B),
        ("+c", Direction::C),
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
        ("+a3", Direction::A, 3),
        ("+b10", Direction::B, 10),
        ("+c1", Direction::C, 1),
    ] {
        assert_eq!(
            parse_direction_token(input),
            Ok(ParsedDirectionToken {
                dir,
                level: Some(level),
            }),
        );
    }
}

#[test]
fn syntactically_accepts_level_zero() {
    assert_eq!(
        parse_direction_token("+a0"),
        Ok(ParsedDirectionToken {
            dir: Direction::A,
            level: Some(0),
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
