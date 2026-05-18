use super::*;
use crate::parse_endpoint_query::parse_endpoint_query;

fn parse(text: &str) -> ParsedRootQuery {
    parse_endpoint_query(text)
        .unwrap_or_else(|e| panic!("unexpected parse failure for {text:?}: {:?}", e[0].message))
}

fn assert_err_contains(eq: &ParsedRootQuery, needle: &str) {
    let err = validate_endpoint_query(eq).unwrap_err();
    let msg = &err[0].message;
    assert!(
        msg.contains(needle),
        "expected validate error to contain {needle:?}, got {msg:?}",
    );
}

#[test]
fn rejects_line_zero_forms() {
    for input in ["0", "0:foo", "0-3", "L0", "l0"] {
        assert_err_contains(&parse(input), "line must be >= 1");
    }
}

#[test]
fn rejects_descending_ranges() {
    for input in ["5-1", "L5-1", "l5-1"] {
        assert_err_contains(&parse(input), "range start must be <= end");
    }
}

#[test]
fn accepts_valid_endpoint_forms() {
    for input in ["1", "1-5", "foo", "L12", "L1-5", "5-5"] {
        validate_endpoint_query(&parse(input)).expect("should validate");
    }
}

#[test]
fn accepts_identifier_without_numeric_validation() {
    for input in ["$", "_", "foo1"] {
        validate_endpoint_query(&parse(input)).expect("should validate");
    }
}
