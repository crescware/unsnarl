//! Mirrors `ts/src/cli/run-cli/resolve-output-path/root-query-token.ts`.

use unsnarl_root_query::ParsedRootQuery;

pub fn root_query_token(q: &ParsedRootQuery) -> String {
    match q {
        ParsedRootQuery::Name { name, .. } => name.clone(),
        ParsedRootQuery::Line { line, .. } => format!("l{line}"),
        ParsedRootQuery::LineName { line, name, .. } => format!("l{line}-{name}"),
        ParsedRootQuery::Range { start, end, .. } => format!("l{start}-{end}"),
        ParsedRootQuery::RangeName {
            start, end, name, ..
        } => format!("l{start}-{end}-{name}"),
        // Filename is derived from CLI args before the resolver runs, so
        // we only know the line number here. Normalize to the same
        // lowercase shape as a plain Line query to keep `-r L12` and
        // `-r 12` filenames aligned (the L-prefix is a typing-convenience
        // syntax).
        ParsedRootQuery::LineOrName { line, .. } => format!("l{line}"),
    }
}

#[cfg(test)]
#[path = "root_query_token_test.rs"]
mod root_query_token_test;
