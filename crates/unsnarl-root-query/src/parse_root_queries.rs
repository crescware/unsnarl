use crate::parse_root_query::parse_root_query;
use crate::parsed_root_query::ParsedRootQuery;

pub fn parse_root_queries(value: &str) -> Result<Vec<ParsedRootQuery>, String> {
    if value.is_empty() {
        return Err("empty --roots value".to_string());
    }
    let mut queries: Vec<ParsedRootQuery> = Vec::new();
    for token in value.split(',') {
        queries.push(parse_root_query(token)?);
    }
    Ok(queries)
}

#[cfg(test)]
#[path = "parse_root_queries_test.rs"]
mod parse_root_queries_test;
