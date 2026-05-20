use super::*;
use unsnarl_ir::SourceLine;

fn assert_err_contains(text: &str, needle: &str) {
    let err = parse_root_query(text).unwrap_err();
    assert!(
        err.contains(needle),
        "expected error for {text:?} to contain {needle:?}, got {err:?}",
    );
}

#[test]
fn parses_a_bare_line_number() {
    assert_eq!(
        parse_root_query("10"),
        Ok(ParsedRootQuery::Line {
            line: SourceLine(10),
            raw: "10".to_string(),
        }),
    );
}

#[test]
fn parses_line_name() {
    assert_eq!(
        parse_root_query("10:counter"),
        Ok(ParsedRootQuery::LineName {
            line: SourceLine(10),
            name: "counter".to_string(),
            raw: "10:counter".to_string(),
        }),
    );
}

#[test]
fn parses_line_name_with_dollar_or_underscore_head() {
    assert_eq!(
        parse_root_query("10:$counter"),
        Ok(ParsedRootQuery::LineName {
            line: SourceLine(10),
            name: "$counter".to_string(),
            raw: "10:$counter".to_string(),
        }),
    );
    assert_eq!(
        parse_root_query("10:_counter"),
        Ok(ParsedRootQuery::LineName {
            line: SourceLine(10),
            name: "_counter".to_string(),
            raw: "10:_counter".to_string(),
        }),
    );
}

#[test]
fn parses_range() {
    assert_eq!(
        parse_root_query("9-13"),
        Ok(ParsedRootQuery::Range {
            start: SourceLine(9),
            end: SourceLine(13),
            raw: "9-13".to_string(),
        }),
    );
}

#[test]
fn parses_range_name() {
    assert_eq!(
        parse_root_query("9-13:value"),
        Ok(ParsedRootQuery::RangeName {
            start: SourceLine(9),
            end: SourceLine(13),
            name: "value".to_string(),
            raw: "9-13:value".to_string(),
        }),
    );
}

#[test]
fn parses_range_name_with_dollar_or_underscore_head() {
    assert_eq!(
        parse_root_query("9-13:$value"),
        Ok(ParsedRootQuery::RangeName {
            start: SourceLine(9),
            end: SourceLine(13),
            name: "$value".to_string(),
            raw: "9-13:$value".to_string(),
        }),
    );
    assert_eq!(
        parse_root_query("9-13:_value"),
        Ok(ParsedRootQuery::RangeName {
            start: SourceLine(9),
            end: SourceLine(13),
            name: "_value".to_string(),
            raw: "9-13:_value".to_string(),
        }),
    );
}

#[test]
fn parses_bare_identifier() {
    assert_eq!(
        parse_root_query("foo"),
        Ok(ParsedRootQuery::Name {
            name: "foo".to_string(),
            raw: "foo".to_string(),
        }),
    );
}

#[test]
fn accepts_identifiers_starting_with_dollar_or_underscore() {
    assert_eq!(
        parse_root_query("$ok"),
        Ok(ParsedRootQuery::Name {
            name: "$ok".to_string(),
            raw: "$ok".to_string(),
        }),
    );
    assert_eq!(
        parse_root_query("_ok"),
        Ok(ParsedRootQuery::Name {
            name: "_ok".to_string(),
            raw: "_ok".to_string(),
        }),
    );
}

#[test]
fn accepts_identifiers_with_digits_in_middle_and_end() {
    assert_eq!(
        parse_root_query("foo1"),
        Ok(ParsedRootQuery::Name {
            name: "foo1".to_string(),
            raw: "foo1".to_string(),
        }),
    );
    assert_eq!(
        parse_root_query("bar2baz"),
        Ok(ParsedRootQuery::Name {
            name: "bar2baz".to_string(),
            raw: "bar2baz".to_string(),
        }),
    );
}

#[test]
fn accepts_single_dollar_or_underscore() {
    assert_eq!(
        parse_root_query("$"),
        Ok(ParsedRootQuery::Name {
            name: "$".to_string(),
            raw: "$".to_string(),
        }),
    );
    assert_eq!(
        parse_root_query("_"),
        Ok(ParsedRootQuery::Name {
            name: "_".to_string(),
            raw: "_".to_string(),
        }),
    );
}

#[test]
fn accepts_identifier_with_dollar_in_middle() {
    assert_eq!(
        parse_root_query("foo$bar"),
        Ok(ParsedRootQuery::Name {
            name: "foo$bar".to_string(),
            raw: "foo$bar".to_string(),
        }),
    );
}

#[test]
fn treats_n_dash_n_as_single_line_range() {
    assert_eq!(
        parse_root_query("5-5"),
        Ok(ParsedRootQuery::Range {
            start: SourceLine(5),
            end: SourceLine(5),
            raw: "5-5".to_string(),
        }),
    );
}

#[test]
fn rejects_empty_string() {
    assert_eq!(parse_root_query(""), Err("empty root query".to_string()));
}

#[test]
fn rejects_identifier_starting_with_digit() {
    assert_err_contains("10:1foo", "unrecognized token");
}

#[test]
fn rejects_empty_identifier_after_colon() {
    assert_err_contains("10:", "unexpected empty identifier after ':'");
}

#[test]
fn rejects_descending_range() {
    assert_err_contains("5-1", "range start must be <= end");
}

#[test]
fn rejects_malformed_ranges() {
    assert_err_contains("1-", "unexpected empty range end");
    assert_err_contains("-5", "unrecognized token");
    assert_err_contains("1-2-3", "unrecognized token");
}

#[test]
fn rejects_line_zero() {
    assert_err_contains("0", "line must be >= 1");
    assert_err_contains("0-3", "line must be >= 1");
}

#[test]
fn rejects_disallowed_identifier_characters() {
    assert_err_contains("foo-bar", "unexpected character in identifier");
    assert_err_contains("foo.bar", "unexpected character in identifier");
}

#[test]
fn parses_uppercase_l_as_line_or_name() {
    assert_eq!(
        parse_root_query("L12"),
        Ok(ParsedRootQuery::LineOrName {
            line: SourceLine(12),
            name: "L12".to_string(),
            raw: "L12".to_string(),
        }),
    );
}

#[test]
fn parses_lowercase_l_as_line_or_name() {
    assert_eq!(
        parse_root_query("l1"),
        Ok(ParsedRootQuery::LineOrName {
            line: SourceLine(1),
            name: "l1".to_string(),
            raw: "l1".to_string(),
        }),
    );
}

#[test]
fn parses_l_range_preserving_raw() {
    assert_eq!(
        parse_root_query("L12-34"),
        Ok(ParsedRootQuery::Range {
            start: SourceLine(12),
            end: SourceLine(34),
            raw: "L12-34".to_string(),
        }),
    );
    assert_eq!(
        parse_root_query("l9-13"),
        Ok(ParsedRootQuery::Range {
            start: SourceLine(9),
            end: SourceLine(13),
            raw: "l9-13".to_string(),
        }),
    );
}

#[test]
fn rejects_l_zero() {
    assert_err_contains("L0", "line must be >= 1");
    assert_err_contains("l0", "line must be >= 1");
}

#[test]
fn rejects_descending_l_range() {
    assert_err_contains("L5-1", "range start must be <= end");
    assert_err_contains("l5-1", "range start must be <= end");
}

#[test]
fn ll12_is_plain_identifier() {
    assert_eq!(
        parse_root_query("LL12"),
        Ok(ParsedRootQuery::Name {
            name: "LL12".to_string(),
            raw: "LL12".to_string(),
        }),
    );
}

#[test]
fn rejects_one_l_two() {
    assert_err_contains("1L2", "unrecognized token");
}

#[test]
fn rejects_l_prefixed_with_colon_identifier() {
    assert_err_contains("L12:foo", "unexpected character in identifier");
}
