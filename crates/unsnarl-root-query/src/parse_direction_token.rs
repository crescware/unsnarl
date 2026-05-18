use crate::parse_error::ParseError;
use crate::root_query::Direction;

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedDirectionToken {
    pub dir: Direction,
    pub level: Option<u32>,
}

pub fn parse_direction_token(text: &str) -> Result<ParsedDirectionToken, Vec<ParseError>> {
    let bytes = text.as_bytes();
    if bytes.len() < 2 || bytes[0] != b'+' {
        return Err(bad(text));
    }
    let dir = match bytes[1] {
        b'a' => Direction::After,
        b'b' => Direction::Before,
        b'c' => Direction::Context,
        _ => return Err(bad(text)),
    };
    let rest = &bytes[2..];
    if rest.is_empty() {
        return Ok(ParsedDirectionToken { dir, level: None });
    }
    if !rest.iter().all(|b| b.is_ascii_digit()) {
        return Err(bad(text));
    }
    let level: u32 = std::str::from_utf8(rest)
        .expect("ASCII digits are valid UTF-8")
        .parse()
        .map_err(|_| bad(text))?;
    Ok(ParsedDirectionToken {
        dir,
        level: Some(level),
    })
}

fn bad(text: &str) -> Vec<ParseError> {
    vec![ParseError {
        message: format!("unexpected direction token '{text}' (expected one of '+a', '+b', '+c')",),
    }]
}

#[cfg(test)]
#[path = "parse_direction_token_test.rs"]
mod parse_direction_token_test;
