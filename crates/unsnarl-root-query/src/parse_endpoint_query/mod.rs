use crate::parse_error::ParseError;
use crate::parsed_root_query::ParsedRootQuery;

pub fn parse_endpoint_query(text: &str) -> Result<ParsedRootQuery, Vec<ParseError>> {
    if let Some(line) = match_all_digits(text) {
        return Ok(ParsedRootQuery::Line {
            line,
            raw: text.to_string(),
        });
    }

    if let Some((line, name)) = match_line_name(text) {
        return Ok(ParsedRootQuery::LineName {
            line,
            name: name.to_string(),
            raw: text.to_string(),
        });
    }

    if let Some((start, end, name)) = match_range_name(text) {
        return Ok(ParsedRootQuery::RangeName {
            start,
            end,
            name: name.to_string(),
            raw: text.to_string(),
        });
    }

    if let Some((start, end)) = match_range(text) {
        return Ok(ParsedRootQuery::Range {
            start,
            end,
            raw: text.to_string(),
        });
    }

    if let Some((start, end)) = match_l_range(text) {
        return Ok(ParsedRootQuery::Range {
            start,
            end,
            raw: text.to_string(),
        });
    }

    if let Some(line) = match_l_line_or_name(text) {
        return Ok(ParsedRootQuery::LineOrName {
            line,
            name: text.to_string(),
            raw: text.to_string(),
        });
    }

    if is_identifier(text) {
        return Ok(ParsedRootQuery::Name {
            name: text.to_string(),
            raw: text.to_string(),
        });
    }

    Err(vec![ParseError {
        message: diagnose(text),
    }])
}

fn diagnose(text: &str) -> String {
    if is_empty_after_colon(text) {
        return format!("unexpected empty identifier after ':' in '{text}'");
    }
    if is_empty_range_end(text) {
        return format!("unexpected empty range end in '{text}' (expected 'n-m')");
    }
    if has_identifier_like_head_with_disallowed_char(text) {
        return format!("unexpected character in identifier '{text}'");
    }
    format!("unrecognized token '{text}'")
}

fn match_all_digits(s: &str) -> Option<u32> {
    parse_digits(s)
}

fn match_line_name(s: &str) -> Option<(u32, &str)> {
    let (lhs, rhs) = split_once(s, b':')?;
    let line = parse_digits(lhs)?;
    if !is_identifier(rhs) {
        return None;
    }
    Some((line, rhs))
}

fn match_range(s: &str) -> Option<(u32, u32)> {
    let (lhs, rhs) = split_once(s, b'-')?;
    let start = parse_digits(lhs)?;
    let end = parse_digits(rhs)?;
    Some((start, end))
}

fn match_range_name(s: &str) -> Option<(u32, u32, &str)> {
    let (lhs, name) = split_once(s, b':')?;
    if !is_identifier(name) {
        return None;
    }
    let (start, end) = match_range(lhs)?;
    Some((start, end, name))
}

fn match_l_range(s: &str) -> Option<(u32, u32)> {
    let rest = strip_l_prefix(s)?;
    match_range(rest)
}

fn match_l_line_or_name(s: &str) -> Option<u32> {
    let rest = strip_l_prefix(s)?;
    parse_digits(rest)
}

fn strip_l_prefix(s: &str) -> Option<&str> {
    s.strip_prefix('L').or_else(|| s.strip_prefix('l'))
}

fn parse_digits(s: &str) -> Option<u32> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok()
}

fn split_once(s: &str, sep: u8) -> Option<(&str, &str)> {
    let i = s.bytes().position(|b| b == sep)?;
    Some((&s[..i], &s[i + 1..]))
}

fn is_id_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_' || b == b'$'
}

fn is_id_cont(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'$'
}

fn is_identifier(s: &str) -> bool {
    let mut bytes = s.bytes();
    match bytes.next() {
        None => false,
        Some(b) if !is_id_start(b) => false,
        _ => bytes.all(is_id_cont),
    }
}

// EMPTY_AFTER_COLON_RE = /^(?:[0-9]+(?:-[0-9]+)?|[Ll][0-9]+):$/
fn is_empty_after_colon(s: &str) -> bool {
    let Some(head) = s.strip_suffix(':') else {
        return false;
    };
    if head.is_empty() {
        return false;
    }
    // Variant A: digits, optionally followed by '-' digits.
    if let Some((a, b)) = split_once(head, b'-') {
        return parse_digits(a).is_some() && parse_digits(b).is_some();
    }
    if parse_digits(head).is_some() {
        return true;
    }
    // Variant B: [Ll] digits.
    if let Some(rest) = strip_l_prefix(head) {
        return parse_digits(rest).is_some();
    }
    false
}

// EMPTY_RANGE_END_RE = /^(?:[0-9]+|[Ll][0-9]+)-$/
fn is_empty_range_end(s: &str) -> bool {
    let Some(head) = s.strip_suffix('-') else {
        return false;
    };
    if head.is_empty() {
        return false;
    }
    if parse_digits(head).is_some() {
        return true;
    }
    if let Some(rest) = strip_l_prefix(head) {
        return parse_digits(rest).is_some();
    }
    false
}

// Identifier-like head: starts with [A-Za-z_$].
// Disallowed char: any byte outside [A-Za-z0-9_$].
fn has_identifier_like_head_with_disallowed_char(s: &str) -> bool {
    let bytes = s.as_bytes();
    let Some(&first) = bytes.first() else {
        return false;
    };
    if !is_id_start(first) {
        return false;
    }
    s.bytes().any(|b| !is_id_cont(b))
}

#[cfg(test)]
mod test;
