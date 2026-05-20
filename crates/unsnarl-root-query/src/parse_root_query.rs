use crate::parse_error::ParseError;
use crate::parse_root_query_ast::parse_root_query_ast;
use crate::parsed_root_query::ParsedRootQuery;
use crate::root_query::RootQuery;
use crate::root_query_scope::ROOT_QUERY_SCOPE_POINT_ONLY;
use crate::validate_root_query::validate_root_query;

pub fn parse_root_query(token: &str) -> Result<ParsedRootQuery, String> {
    let ast = parse_root_query_ast(token, &ROOT_QUERY_SCOPE_POINT_ONLY).map_err(first_message)?;
    validate_root_query(&ast).map_err(first_message)?;
    match ast {
        RootQuery::Single { query, .. } => Ok(query),
        // Under `ROOT_QUERY_SCOPE_POINT_ONLY`, `parse_root_query_ast`
        // rejects `Path` and `Direction` at the scope check before
        // returning. Reaching this arm would mean that contract was
        // broken upstream — panic to surface the bug at its source.
        RootQuery::Path { .. } | RootQuery::Direction { .. } => {
            unreachable!("ROOT_QUERY_SCOPE_POINT_ONLY must reject non-Single shapes upstream")
        }
    }
}

fn first_message(errs: Vec<ParseError>) -> String {
    errs.into_iter()
        .next()
        .map(|e| e.message)
        .unwrap_or_else(|| "(no message)".to_string())
}

#[cfg(test)]
#[path = "parse_root_query_test.rs"]
mod parse_root_query_test;
