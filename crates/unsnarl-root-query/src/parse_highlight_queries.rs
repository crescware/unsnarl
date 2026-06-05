//! Parse a `-H` / `--highlight` value into the richer [`RootQuery`]
//! AST (point / path / direction), as opposed to
//! [`crate::parse_root_queries`], which collapses every token to a
//! point-only [`crate::ParsedRootQuery`] for `-r`.
//!
//! Issue #90: this is the single entry point that lets the `..`
//! operator and `+a` / `+b` / `+c` direction tokens through, via
//! [`ROOT_QUERY_SCOPE_HIGHLIGHT`]. `-r` keeps using the point-only
//! scope and is untouched.

use crate::parse_error::ParseError;
use crate::parse_root_query_ast::parse_root_query_ast;
use crate::root_query::RootQuery;
use crate::root_query_scope::ROOT_QUERY_SCOPE_HIGHLIGHT;
use crate::validate_root_query::validate_root_query;

pub fn parse_highlight_queries(value: &str) -> Result<Vec<RootQuery>, String> {
    if value.is_empty() {
        return Err("empty --highlight value".to_string());
    }
    let mut queries: Vec<RootQuery> = Vec::new();
    for token in value.split(',') {
        let ast =
            parse_root_query_ast(token, &ROOT_QUERY_SCOPE_HIGHLIGHT).map_err(first_message)?;
        validate_root_query(&ast).map_err(first_message)?;
        queries.push(ast);
    }
    Ok(queries)
}

fn first_message(errs: Vec<ParseError>) -> String {
    errs.into_iter()
        .next()
        .map(|e| e.message)
        .unwrap_or_else(|| "(no message)".to_string())
}

#[cfg(test)]
#[path = "parse_highlight_queries_test.rs"]
mod parse_highlight_queries_test;
