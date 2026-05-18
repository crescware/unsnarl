use super::*;

#[test]
fn non_negative_integer_zero() {
    assert_eq!(parse_generation_count("0"), Ok(0));
}

#[test]
fn non_negative_integer_one() {
    assert_eq!(parse_generation_count("1"), Ok(1));
}

#[test]
fn non_negative_integer_forty_two() {
    assert_eq!(parse_generation_count("42"), Ok(42));
}

#[test]
fn negative_is_rejected() {
    assert!(parse_generation_count("-1").is_err());
}

#[test]
fn decimal_is_rejected() {
    assert!(parse_generation_count("1.5").is_err());
}

#[test]
fn non_numeric_is_rejected() {
    assert!(parse_generation_count("abc").is_err());
}

#[test]
fn empty_string_is_rejected() {
    assert!(parse_generation_count("").is_err());
}

#[test]
fn whitespace_padded_is_rejected() {
    assert!(parse_generation_count(" 1 ").is_err());
}

#[test]
fn leading_plus_is_rejected() {
    assert!(parse_generation_count("+1").is_err());
}

#[test]
fn hex_is_rejected() {
    assert!(parse_generation_count("0x10").is_err());
}

#[test]
fn scientific_is_rejected() {
    assert!(parse_generation_count("1e3").is_err());
}
