//! Per-node matcher for a single highlight query.
//!
//! Mirrors `ts/src/visual-graph/highlight/node-matches-highlight-query.ts`.
//! Highlight reuses the `-r/--roots` query grammar but intentionally
//! diverges on matching semantics: pruning applies `NAME_QUERY_EXCLUDED`
//! on bare name queries (so `-r counter` does not drag every assignment /
//! JSX read into the root set), whereas highlight is about painting
//! "every place this identifier appears" and benefits from matching
//! those use-sites. The grammar (line / line-name / range / range-name /
//! name) stays the same — only the name-query exclusion is dropped.

use unsnarl_ir::SourceLine;
use unsnarl_root_query::ParsedRootQuery;

use crate::visual_node::VisualNode;

pub fn node_matches_highlight_query(node: &VisualNode, q: &ParsedRootQuery) -> bool {
    let start_line = SourceLine(node.line());
    let end_line = SourceLine(node.end_line().unwrap_or_else(|| node.line()));
    match q {
        ParsedRootQuery::Line { line, .. } => *line >= start_line && *line <= end_line,
        ParsedRootQuery::LineName { line, name, .. } => {
            *line >= start_line && *line <= end_line && node.name() == name.as_str()
        }
        ParsedRootQuery::Range { start, end, .. } => start_line <= *end && end_line >= *start,
        ParsedRootQuery::RangeName {
            start, end, name, ..
        } => start_line <= *end && end_line >= *start && node.name() == name.as_str(),
        ParsedRootQuery::Name { name, .. } => node.name() == name.as_str(),
        ParsedRootQuery::LineOrName { .. } => {
            // resolve_ambiguous_queries rewrites every line-or-name into
            // Line or Name before highlight runs; this arm stays only
            // for switch exhaustiveness.
            false
        }
    }
}

#[cfg(test)]
#[path = "node_matches_highlight_query_test.rs"]
mod node_matches_highlight_query_test;
