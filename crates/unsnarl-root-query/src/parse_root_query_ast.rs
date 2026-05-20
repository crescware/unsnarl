use crate::parse_direction_token::parse_direction_token;
use crate::parse_endpoint_query::parse_endpoint_query;
use crate::parse_error::ParseError;
use crate::root_query::RootQuery;
use crate::root_query_scope::RootQueryScope;

pub fn parse_root_query_ast(
    token: &str,
    scope: &RootQueryScope,
) -> Result<RootQuery, Vec<ParseError>> {
    if token.is_empty() {
        return Err(vec![ParseError {
            message: "empty root query".to_string(),
        }]);
    }

    let mut parts = token.splitn(3, "..");
    let first = parts
        .next()
        .expect("splitn always yields at least one item");
    let second = parts.next();
    let third = parts.next();

    if third.is_some() {
        return Err(vec![ParseError {
            message: format!("unexpected duplicate '..' in '{token}'"),
        }]);
    }

    let Some(rhs_text) = second else {
        let query = parse_endpoint_query(first)?;
        if !scope.point {
            return Err(vec![ParseError {
                message: format!("unexpected token '{token}'"),
            }]);
        }
        return Ok(RootQuery::Single {
            query,
            raw: token.to_string(),
        });
    };

    let lhs_text = first;

    if lhs_text.is_empty() {
        return Err(vec![ParseError {
            message: format!("unexpected empty left-hand side of '..' in '{token}'"),
        }]);
    }
    if rhs_text.is_empty() {
        return Err(vec![ParseError {
            message: format!("unexpected empty right-hand side of '..' in '{token}'"),
        }]);
    }

    let lhs = parse_endpoint_query(lhs_text)?;

    if rhs_text.starts_with('+') {
        let dir_tok = parse_direction_token(rhs_text)?;
        if !scope.direction {
            return Err(vec![ParseError {
                message: format!("unexpected direction token '{rhs_text}' in '{token}'"),
            }]);
        }
        if dir_tok.level.is_some() && !scope.direction_level {
            return Err(vec![ParseError {
                message: format!("unexpected level in direction token '{rhs_text}'"),
            }]);
        }
        return Ok(RootQuery::Direction {
            lhs,
            dir: dir_tok.dir,
            level: dir_tok.level,
            raw: token.to_string(),
        });
    }

    let rhs = parse_endpoint_query(rhs_text)?;
    if !scope.path {
        return Err(vec![ParseError {
            message: format!("unexpected '..' in '{token}'"),
        }]);
    }
    Ok(RootQuery::Path {
        lhs,
        rhs,
        raw: token.to_string(),
    })
}

#[cfg(test)]
#[path = "parse_root_query_ast_test.rs"]
mod parse_root_query_ast_test;
