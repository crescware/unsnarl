use crate::parse_root_query_ast::parse_root_query_ast;
use crate::parsed_root_query::ParsedRootQuery;
use crate::root_query::RootQuery;
use crate::root_query_scope::ROOT_QUERY_SCOPE_POINT_ONLY;
use crate::validate_root_query::validate_root_query;

pub fn parse_root_query(token: &str) -> Result<ParsedRootQuery, String> {
    let ast = parse_root_query_ast(token, &ROOT_QUERY_SCOPE_POINT_ONLY).map_err(first_message)?;
    let kind_label = root_query_kind_label(&ast);
    validate_root_query(&ast).map_err(first_message)?;
    match ast {
        RootQuery::Single { query, .. } => Ok(query),
        _ => Err(format!(
            "internal error: expected 'single' RootQuery, got '{kind_label}'",
        )),
    }
}

fn first_message(errs: Vec<crate::parse_error::ParseError>) -> String {
    errs.into_iter()
        .next()
        .map(|e| e.message)
        .unwrap_or_else(|| "(no message)".to_string())
}

fn root_query_kind_label(rq: &RootQuery) -> &'static str {
    match rq {
        RootQuery::Single { .. } => "single",
        RootQuery::Path { .. } => "path",
        RootQuery::Direction { .. } => "direction",
    }
}

#[cfg(test)]
mod test;
