use crate::parse_error::ParseError;
use crate::parsed_root_query::ParsedRootQuery;

pub fn validate_endpoint_query(eq: &ParsedRootQuery) -> Result<(), Vec<ParseError>> {
    match eq {
        ParsedRootQuery::Line { line, raw }
        | ParsedRootQuery::LineName { line, raw, .. }
        | ParsedRootQuery::LineOrName { line, raw, .. } => {
            if *line < 1 {
                return Err(vec![ParseError {
                    message: format!("line must be >= 1 in '{raw}'"),
                }]);
            }
            Ok(())
        }
        ParsedRootQuery::Range { start, end, raw }
        | ParsedRootQuery::RangeName {
            start, end, raw, ..
        } => {
            if *start < 1 {
                return Err(vec![ParseError {
                    message: format!("line must be >= 1 in '{raw}'"),
                }]);
            }
            if *start > *end {
                return Err(vec![ParseError {
                    message: format!("range start must be <= end in '{raw}'"),
                }]);
            }
            Ok(())
        }
        ParsedRootQuery::Name { .. } => Ok(()),
    }
}

#[cfg(test)]
mod test;
