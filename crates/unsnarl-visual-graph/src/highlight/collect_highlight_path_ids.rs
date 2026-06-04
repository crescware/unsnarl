//! POC (issue #90) reachability collector for the richer highlight
//! grammar.
//!
//! Given a list of [`RootQuery`] (point / path / direction), returns a
//! [`HighlightSelection`]: the ids to highlight plus the point-query
//! subset. Reachability is computed over the VisualGraph's `edges`
//! (post-pruning) — decision 1-A, the drawn graph not the IR.
//!
//! - `Single`  -> the existing point match (every node the endpoint
//!   query matches). Recorded in `point_ids` too, so the renderer keeps
//!   the radius-1 'either endpoint' edge treatment for it.
//! - `Direction { lhs, +a }` -> `lhs` plus every node forward-reachable
//!   from it; `+b` backward; `+c` both.
//! - `Path { lhs, rhs }` -> direction-independent: the union of nodes
//!   on a directed path either way, expressed as the set intersection
//!   `(reach_fwd(lhs) n reach_bwd(rhs)) u (reach_fwd(rhs) n
//!   reach_bwd(lhs))`. No path *enumeration*, so cycles and fan-out are
//!   handled for free.
//!
//! Reachability rides `graph.edges`, whose endpoints can be node *or*
//! subgraph ids, and the emit walks the whole element tree — so a
//! subgraph or a non-root-candidate node that sits on a path is part of
//! the selection (judgment B), not silently dropped.
//!
//! Empty results (an endpoint that matches nothing, or a `Path` with no
//! connecting route) emit a stderr warning and contribute nothing,
//! leaving the exit code untouched (decision 3).

use std::collections::{HashMap, HashSet};

use unsnarl_root_query::{Direction, ParsedRootQuery, RootQuery};

use crate::highlight::node_matches_highlight_query::node_matches_highlight_query;
use crate::prune::iterate_visual_nodes::iterate_visual_nodes;
use crate::prune::resolve_ambiguous_queries;
use crate::visual_edge::VisualEdge;
use crate::visual_element::VisualElement;
use crate::visual_graph::VisualGraph;

/// What to highlight, resolved against a graph.
pub struct HighlightSelection {
    /// Every id to highlight, in element-tree walk order (nodes and
    /// subgraphs). The renderer styles these.
    pub ids: Vec<String>,
    /// The subset contributed by point (`Single`) queries. The renderer
    /// keeps the radius-1 'either endpoint' edge rule for these; ids
    /// that are *only* reachability hits paint an edge only when BOTH
    /// endpoints are in the set (judgment A — no boundary bleed).
    pub point_ids: Vec<String>,
}

pub fn collect_highlight_path_ids(
    graph: &VisualGraph,
    queries: &[RootQuery],
) -> HighlightSelection {
    if queries.is_empty() {
        return HighlightSelection {
            ids: Vec::new(),
            point_ids: Vec::new(),
        };
    }
    let fwd = build_adjacency(&graph.edges, EdgeEnd::From);
    let bwd = build_adjacency(&graph.edges, EdgeEnd::To);

    let mut set: HashSet<String> = HashSet::new();
    let mut point_set: HashSet<String> = HashSet::new();
    for q in queries {
        match q {
            RootQuery::Single { query, .. } => {
                for id in seed_ids(graph, query) {
                    set.insert(id.clone());
                    point_set.insert(id);
                }
            }
            RootQuery::Direction { lhs, dir, raw, .. } => {
                let seeds = seed_ids(graph, lhs);
                if seeds.is_empty() {
                    warn_no_match(raw);
                    continue;
                }
                match dir {
                    Direction::After => extend(&mut set, reach(&fwd, &seeds)),
                    Direction::Before => extend(&mut set, reach(&bwd, &seeds)),
                    Direction::Context => {
                        extend(&mut set, reach(&fwd, &seeds));
                        extend(&mut set, reach(&bwd, &seeds));
                    }
                }
            }
            RootQuery::Path { lhs, rhs, raw } => {
                let l = seed_ids(graph, lhs);
                let r = seed_ids(graph, rhs);
                if l.is_empty() || r.is_empty() {
                    warn_no_match(raw);
                    continue;
                }
                let lf = reach(&fwd, &l);
                let lb = reach(&bwd, &l);
                let rf = reach(&fwd, &r);
                let rb = reach(&bwd, &r);
                let before = set.len();
                for id in &lf {
                    if rb.contains(id) {
                        set.insert(id.clone());
                    }
                }
                for id in &rf {
                    if lb.contains(id) {
                        set.insert(id.clone());
                    }
                }
                if set.len() == before {
                    warn_no_path(raw);
                }
            }
        }
    }

    let ids = emit_in_walk_order(graph, &set);
    let point_ids = ids
        .iter()
        .filter(|id| point_set.contains(id.as_str()))
        .cloned()
        .collect();
    HighlightSelection { ids, point_ids }
}

enum EdgeEnd {
    From,
    To,
}

/// Build a directed adjacency map keyed by the chosen endpoint. The
/// keys / values borrow the edge strings, so the map lives only as long
/// as `edges`.
fn build_adjacency(edges: &[VisualEdge], key_on: EdgeEnd) -> HashMap<&str, Vec<&str>> {
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for e in edges {
        let (k, v) = match key_on {
            EdgeEnd::From => (e.from.as_str(), e.to.as_str()),
            EdgeEnd::To => (e.to.as_str(), e.from.as_str()),
        };
        adj.entry(k).or_default().push(v);
    }
    adj
}

/// Breadth-first reachable set including the seeds themselves.
fn reach(adj: &HashMap<&str, Vec<&str>>, seeds: &[String]) -> HashSet<String> {
    let mut visited: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = Vec::new();
    for s in seeds {
        if visited.insert(s.clone()) {
            stack.push(s.clone());
        }
    }
    while let Some(cur) = stack.pop() {
        if let Some(next) = adj.get(cur.as_str()) {
            for &n in next {
                if visited.insert(n.to_string()) {
                    stack.push(n.to_string());
                }
            }
        }
    }
    visited
}

fn extend(set: &mut HashSet<String>, ids: HashSet<String>) {
    for id in ids {
        set.insert(id);
    }
}

/// Resolve a single endpoint's `LineOrName` ambiguity against the
/// graph, then return every visible node id the endpoint matches.
fn seed_ids(graph: &VisualGraph, endpoint: &ParsedRootQuery) -> Vec<String> {
    let resolved = resolve_ambiguous_queries(graph, std::slice::from_ref(endpoint)).resolved;
    let query = resolved
        .into_iter()
        .next()
        .unwrap_or_else(|| endpoint.clone());
    let mut ids: Vec<String> = Vec::new();
    iterate_visual_nodes(&graph.elements, &mut |node| {
        if node_matches_highlight_query(node, &query) {
            ids.push(node.id().to_string());
        }
    });
    ids
}

/// Emit the highlighted ids in element-tree walk order with duplicates
/// removed. Walks *every* element (nodes and subgraphs), so a subgraph
/// or non-root-candidate node in the set is emitted too. For point
/// queries the set holds only matched root-candidate nodes, so this
/// reproduces `collect_highlight_ids`'s order byte-for-byte.
fn emit_in_walk_order(graph: &VisualGraph, set: &HashSet<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut emitted: HashSet<String> = HashSet::new();
    walk_elements(&graph.elements, set, &mut out, &mut emitted);
    out
}

fn walk_elements(
    elements: &[VisualElement],
    set: &HashSet<String>,
    out: &mut Vec<String>,
    emitted: &mut HashSet<String>,
) {
    for el in elements {
        let id = el.id();
        if set.contains(id) && emitted.insert(id.to_string()) {
            out.push(id.to_string());
        }
        if let VisualElement::Subgraph(sg) = el {
            walk_elements(sg.elements(), set, out, emitted);
        }
    }
}

fn warn_no_match(raw: &str) {
    eprintln!("warning: highlight query '{raw}' matched no node");
}

fn warn_no_path(raw: &str) {
    eprintln!("warning: highlight query '{raw}' has no connecting path");
}

#[cfg(test)]
#[path = "collect_highlight_path_ids_test.rs"]
mod collect_highlight_path_ids_test;
