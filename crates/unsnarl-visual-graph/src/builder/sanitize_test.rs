//! Sibling tests for [`sanitize`]. Cases mirror
//! `ts/src/visual-graph/builder/sanitize.test.ts`.

use super::sanitize;

#[test]
fn alphanumerics_and_underscores_pass_through() {
    assert_eq!(sanitize("abc_123_XYZ"), "abc_123_XYZ");
}

#[test]
fn dot_becomes_underscore() {
    assert_eq!(sanitize("a.b"), "a_b");
}

#[test]
fn slash_becomes_underscore() {
    assert_eq!(sanitize("a/b"), "a_b");
}

#[test]
fn hyphen_becomes_underscore() {
    assert_eq!(sanitize("a-b"), "a_b");
}

#[test]
fn colon_becomes_underscore() {
    assert_eq!(sanitize("a:b"), "a_b");
}

#[test]
fn space_becomes_underscore() {
    assert_eq!(sanitize("a b"), "a_b");
}

#[test]
fn consecutive_specials_produce_consecutive_underscores() {
    assert_eq!(sanitize("a..b"), "a__b");
}

#[test]
fn all_special_becomes_all_underscores() {
    assert_eq!(sanitize("!@#$"), "____");
}

#[test]
fn non_ascii_letter_becomes_underscore() {
    assert_eq!(sanitize("あa"), "_a");
}

#[test]
fn empty_string_returns_empty() {
    assert_eq!(sanitize(""), "");
}

#[test]
fn surrogate_pair_becomes_two_underscores() {
    // Mirrors JS string-level UTF-16 stepping: a character outside
    // the BMP (here U+1F60A 😊) is represented as a high+low
    // surrogate pair, each rejected by the alphanumeric class.
    assert_eq!(sanitize("\u{1F60A}a"), "__a");
}
