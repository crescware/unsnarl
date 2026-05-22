//! Per-node matcher for a single `-r` query.

use unsnarl_ir::SourceLine;
use unsnarl_root_query::ParsedRootQuery;

use crate::prune::name_query_excluded::is_name_query_excluded;
use crate::visual_node::VisualNode;

pub fn node_matches_query(node: &VisualNode, q: &ParsedRootQuery) -> bool {
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
        ParsedRootQuery::Name { name, .. } => {
            !is_name_query_excluded(node.kind()) && node.name() == name.as_str()
        }
        ParsedRootQuery::LineOrName { .. } => {
            // resolveAmbiguousQueries rewrites every line-or-name into
            // Line or Name before pruning runs, so this branch is
            // unreachable. The arm exists to keep the match exhaustive.
            false
        }
    }
}

#[cfg(test)]
#[path = "node_matches_query_test.rs"]
mod node_matches_query_test;
