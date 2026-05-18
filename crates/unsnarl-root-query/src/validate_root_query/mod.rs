use crate::parse_error::ParseError;
use crate::root_query::RootQuery;
use crate::validate_endpoint_query::validate_endpoint_query;

pub fn validate_root_query(rq: &RootQuery) -> Result<(), Vec<ParseError>> {
    match rq {
        RootQuery::Single { query, .. } => validate_endpoint_query(query),
        RootQuery::Path { lhs, rhs, .. } => {
            validate_endpoint_query(lhs)?;
            validate_endpoint_query(rhs)
        }
        RootQuery::Direction { lhs, .. } => validate_endpoint_query(lhs),
    }
}

#[cfg(test)]
mod test;
