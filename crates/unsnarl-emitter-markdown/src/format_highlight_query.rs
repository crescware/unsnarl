//! Format a [`HighlightRunOptions`] as the CLI invocation that
//! produced it.
//!
//! Mirrors `ts/src/emitter/markdown/format-highlight-query.ts`. The
//! markdown emitter surfaces the result in its `## Query` block so the
//! rendered preview round-trips back to the user's `-H` / `--highlight`
//! invocation: `-H` for the no-value (roots-tracking) mode, `-H <raw>`
//! when the user supplied a query list. The raw string is
//! reconstructed from the parsed queries' `.raw` so multi-token
//! `-H "a,L7"` keeps its comma form rather than getting normalized.

use unsnarl_root_query::ParsedRootQuery;
use unsnarl_visual_graph::highlight::HighlightRunOptions;

pub fn format_highlight_query(h: &HighlightRunOptions) -> String {
    match h {
        HighlightRunOptions::Roots => "-H".to_string(),
        HighlightRunOptions::Queries(queries) => {
            let parts: Vec<&str> = queries.iter().map(query_raw).collect();
            format!("-H {}", parts.join(","))
        }
    }
}

fn query_raw(q: &ParsedRootQuery) -> &str {
    match q {
        ParsedRootQuery::Line { raw, .. }
        | ParsedRootQuery::LineName { raw, .. }
        | ParsedRootQuery::Range { raw, .. }
        | ParsedRootQuery::RangeName { raw, .. }
        | ParsedRootQuery::Name { raw, .. }
        | ParsedRootQuery::LineOrName { raw, .. } => raw,
    }
}
