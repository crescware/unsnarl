//! Disambiguate `LineOrName` queries by inspecting the graph.
//!
//! Mirrors `ts/src/visual-graph/prune/resolve-ambiguous-queries.ts`.

use std::collections::HashSet;

use unsnarl_root_query::ParsedRootQuery;

use crate::prune::iterate_visual_nodes::iterate_visual_nodes;
use crate::prune::name_query_excluded::is_name_query_excluded;
use crate::prune::root_query_resolution::{ResolvedAs, RootQueryResolution};
use crate::visual_graph::VisualGraph;

/// `^[Ll][0-9]+$` — an `L` or `l` followed by at least one ASCII
/// digit, end-of-string. Inlined because the regex crate is not in
/// the workspace dependency set and the pattern is tiny.
fn is_l_prefixed(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some('L') | Some('l') => {}
        _ => return false,
    }
    let mut has_digit = false;
    for c in chars {
        if !c.is_ascii_digit() {
            return false;
        }
        has_digit = true;
    }
    has_digit
}

pub struct ResolveResult {
    pub resolved: Vec<ParsedRootQuery>,
    pub resolutions: Vec<RootQueryResolution>,
}

pub fn resolve_ambiguous_queries(
    graph: &VisualGraph,
    queries: &[ParsedRootQuery],
) -> ResolveResult {
    let has_ambiguous = queries
        .iter()
        .any(|q| matches!(q, ParsedRootQuery::LineOrName { .. }));
    if !has_ambiguous {
        return ResolveResult {
            resolved: clone_all(queries),
            resolutions: Vec::new(),
        };
    }

    // Collect names that a `Name` query could actually match: candidate
    // nodes (per is_root_candidate_kind via iterate_visual_nodes) minus
    // the use-site kinds excluded from name matching. Anything else is
    // invisible to `-r <id>`, so it must not influence the ambiguity
    // decision either.
    let mut matchable_names: HashSet<String> = HashSet::new();
    iterate_visual_nodes(&graph.elements, &mut |node| {
        if is_name_query_excluded(node.kind()) {
            return;
        }
        matchable_names.insert(node.name().to_string());
    });

    let any_l_prefixed_matchable = matchable_names.iter().any(|n| is_l_prefixed(n));

    let mut resolved: Vec<ParsedRootQuery> = Vec::with_capacity(queries.len());
    let mut resolutions: Vec<RootQueryResolution> = Vec::new();

    for q in queries {
        match q {
            ParsedRootQuery::LineOrName { line, name, raw } => {
                if !any_l_prefixed_matchable {
                    resolved.push(ParsedRootQuery::Line {
                        line: *line,
                        raw: raw.clone(),
                    });
                    continue;
                }
                if matchable_names.contains(name) {
                    resolved.push(ParsedRootQuery::Name {
                        name: name.clone(),
                        raw: raw.clone(),
                    });
                    resolutions.push(RootQueryResolution {
                        raw: raw.clone(),
                        line: *line,
                        name: name.clone(),
                        resolved_as: ResolvedAs::Name,
                    });
                } else {
                    resolved.push(ParsedRootQuery::Line {
                        line: *line,
                        raw: raw.clone(),
                    });
                    resolutions.push(RootQueryResolution {
                        raw: raw.clone(),
                        line: *line,
                        name: name.clone(),
                        resolved_as: ResolvedAs::Line,
                    });
                }
            }
            other => resolved.push(clone_query(other)),
        }
    }

    ResolveResult {
        resolved,
        resolutions,
    }
}

fn clone_all(queries: &[ParsedRootQuery]) -> Vec<ParsedRootQuery> {
    queries.iter().map(clone_query).collect()
}

fn clone_query(q: &ParsedRootQuery) -> ParsedRootQuery {
    match q {
        ParsedRootQuery::Line { line, raw } => ParsedRootQuery::Line {
            line: *line,
            raw: raw.clone(),
        },
        ParsedRootQuery::LineName { line, name, raw } => ParsedRootQuery::LineName {
            line: *line,
            name: name.clone(),
            raw: raw.clone(),
        },
        ParsedRootQuery::Range { start, end, raw } => ParsedRootQuery::Range {
            start: *start,
            end: *end,
            raw: raw.clone(),
        },
        ParsedRootQuery::RangeName {
            start,
            end,
            name,
            raw,
        } => ParsedRootQuery::RangeName {
            start: *start,
            end: *end,
            name: name.clone(),
            raw: raw.clone(),
        },
        ParsedRootQuery::Name { name, raw } => ParsedRootQuery::Name {
            name: name.clone(),
            raw: raw.clone(),
        },
        ParsedRootQuery::LineOrName { line, name, raw } => ParsedRootQuery::LineOrName {
            line: *line,
            name: name.clone(),
            raw: raw.clone(),
        },
    }
}

#[cfg(test)]
#[path = "resolve_ambiguous_queries_test.rs"]
mod resolve_ambiguous_queries_test;
