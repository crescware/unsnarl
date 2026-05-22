use super::*;
use unsnarl_ir::SourceLine;

fn assert_err_contains(text: &str, needle: &str) {
    let err = parse_endpoint_query(text).unwrap_err();
    let msg = &err[0].message;
    assert!(
        msg.contains(needle),
        "expected error for {text:?} to contain {needle:?}, got {msg:?}",
    );
}

#[test]
fn parses_a_bare_line_number() {
    assert_eq!(
        parse_endpoint_query("10"),
        Ok(ParsedRootQuery::Line {
            line: SourceLine(10),
            raw: "10".to_string(),
        }),
    );
}

#[test]
fn parses_line_name() {
    assert_eq!(
        parse_endpoint_query("10:counter"),
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
        parse_endpoint_query("10:$counter"),
        Ok(ParsedRootQuery::LineName {
            line: SourceLine(10),
            name: "$counter".to_string(),
            raw: "10:$counter".to_string(),
        }),
    );
    assert_eq!(
        parse_endpoint_query("10:_counter"),
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
        parse_endpoint_query("9-13"),
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
        parse_endpoint_query("9-13:value"),
        Ok(ParsedRootQuery::RangeName {
            start: SourceLine(9),
            end: SourceLine(13),
            name: "value".to_string(),
            raw: "9-13:value".to_string(),
        }),
    );
}

#[test]
fn parses_bare_identifier() {
    assert_eq!(
        parse_endpoint_query("foo"),
        Ok(ParsedRootQuery::Name {
            name: "foo".to_string(),
            raw: "foo".to_string(),
        }),
    );
}

#[test]
fn accepts_identifiers_starting_with_dollar_or_underscore() {
    assert_eq!(
        parse_endpoint_query("$ok"),
        Ok(ParsedRootQuery::Name {
            name: "$ok".to_string(),
            raw: "$ok".to_string(),
        }),
    );
    assert_eq!(
        parse_endpoint_query("_ok"),
        Ok(ParsedRootQuery::Name {
            name: "_ok".to_string(),
            raw: "_ok".to_string(),
        }),
    );
}

#[test]
fn accepts_identifiers_with_digits_in_middle_and_end() {
    assert_eq!(
        parse_endpoint_query("foo1"),
        Ok(ParsedRootQuery::Name {
            name: "foo1".to_string(),
            raw: "foo1".to_string(),
        }),
    );
    assert_eq!(
        parse_endpoint_query("bar2baz"),
        Ok(ParsedRootQuery::Name {
            name: "bar2baz".to_string(),
            raw: "bar2baz".to_string(),
        }),
    );
}

#[test]
fn accepts_single_dollar_or_underscore() {
    assert_eq!(
        parse_endpoint_query("$"),
        Ok(ParsedRootQuery::Name {
            name: "$".to_string(),
            raw: "$".to_string(),
        }),
    );
    assert_eq!(
        parse_endpoint_query("_"),
        Ok(ParsedRootQuery::Name {
            name: "_".to_string(),
            raw: "_".to_string(),
        }),
    );
}

#[test]
fn parses_uppercase_l_as_line_or_name() {
    assert_eq!(
        parse_endpoint_query("L12"),
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
        parse_endpoint_query("l1"),
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
        parse_endpoint_query("L12-34"),
        Ok(ParsedRootQuery::Range {
            start: SourceLine(12),
            end: SourceLine(34),
            raw: "L12-34".to_string(),
        }),
    );
    assert_eq!(
        parse_endpoint_query("l9-13"),
        Ok(ParsedRootQuery::Range {
            start: SourceLine(9),
            end: SourceLine(13),
            raw: "l9-13".to_string(),
        }),
    );
}

#[test]
fn ll12_is_plain_identifier() {
    assert_eq!(
        parse_endpoint_query("LL12"),
        Ok(ParsedRootQuery::Name {
            name: "LL12".to_string(),
            raw: "LL12".to_string(),
        }),
    );
}

#[test]
fn syntactically_accepts_zero_line_forms() {
    assert_eq!(
        parse_endpoint_query("0"),
        Ok(ParsedRootQuery::Line {
            line: SourceLine(0),
            raw: "0".to_string(),
        }),
    );
    assert_eq!(
        parse_endpoint_query("0:foo"),
        Ok(ParsedRootQuery::LineName {
            line: SourceLine(0),
            name: "foo".to_string(),
            raw: "0:foo".to_string(),
        }),
    );
    assert_eq!(
        parse_endpoint_query("0-3"),
        Ok(ParsedRootQuery::Range {
            start: SourceLine(0),
            end: SourceLine(3),
            raw: "0-3".to_string(),
        }),
    );
    assert_eq!(
        parse_endpoint_query("L0"),
        Ok(ParsedRootQuery::LineOrName {
            line: SourceLine(0),
            name: "L0".to_string(),
            raw: "L0".to_string(),
        }),
    );
}

#[test]
fn syntactically_accepts_descending_ranges() {
    for input in ["5-1", "L5-1", "l5-1"] {
        match parse_endpoint_query(input).unwrap() {
            ParsedRootQuery::Range { start, end, .. } => {
                assert_eq!((start, end), (SourceLine(5), SourceLine(1)));
            }
            other => panic!("expected Range for {input:?}, got {other:?}"),
        }
    }
}

#[test]
fn reports_empty_identifier_after_colon() {
    for input in ["10:", "9-13:", "L12:"] {
        assert_err_contains(input, "unexpected empty identifier after ':'");
    }
}

#[test]
fn reports_empty_range_end() {
    assert_err_contains("1-", "unexpected empty range end");
    assert_err_contains("L1-", "unexpected empty range end");
}

#[test]
fn reports_unexpected_character_in_identifier() {
    assert_err_contains("foo-bar", "unexpected character in identifier");
    assert_err_contains("foo.bar", "unexpected character in identifier");
}

#[test]
fn reports_unexpected_character_for_l_prefixed_with_colon() {
    assert_err_contains("L12:foo", "unexpected character in identifier");
}

#[test]
fn reports_unrecognized_token() {
    for input in ["1L2", "10:1foo", "-5", "1-2-3"] {
        assert_err_contains(input, "unrecognized token");
    }
}

// `u32::from_str` accepts a leading `+`, so without an explicit
// byte-level guard `+5`, `+5-3`, `+5:foo` would all be silently
// accepted as line / range / line-name forms. The endpoint grammar
// pins them to `^[0-9]+$`, so reject any byte that is not an ASCII
// digit before delegating to `u32::from_str`.
#[test]
fn rejects_leading_plus_in_numeric_forms() {
    for input in ["+5", "+5:foo", "+5-3", "+5:", "+5-"] {
        assert!(
            parse_endpoint_query(input).is_err(),
            "expected error for {input:?}, got {:?}",
            parse_endpoint_query(input),
        );
    }
}

#[test]
fn syntactically_accepts_u32_max_line() {
    assert_eq!(
        parse_endpoint_query("4294967295"),
        Ok(ParsedRootQuery::Line {
            line: SourceLine(u32::MAX),
            raw: "4294967295".to_string(),
        }),
    );
}

#[test]
fn rejects_u32_overflow_line() {
    assert_err_contains("4294967296", "unrecognized token");
}
