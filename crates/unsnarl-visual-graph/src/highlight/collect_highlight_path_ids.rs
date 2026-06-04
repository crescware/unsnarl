//! POC (issue #90) reachability collector for the richer highlight
//! grammar.
//!
//! Given a list of [`RootQuery`] (point / path / direction), returns
//! the ids of every visible node to highlight, computed over the
//! VisualGraph's `edges` (post-pruning). This is decision 1-A from the
//! design discussion: reachability rides the drawn graph, not the IR.
//!
//! - `Single`  -> the existing point match (every node the endpoint
//!   query matches).
//! - `Direction { lhs, +a }` -> `lhs` plus every node forward-reachable
//!   from it; `+b` backward; `+c` both.
//! - `Path { lhs, rhs }` -> direction-independent: the union of nodes
//!   on a directed path either way, expressed as the set intersection
//!   `(reach_fwd(lhs) ∩ reach_bwd(rhs)) ∪ (reach_fwd(rhs) ∩
//!   reach_bwd(lhs))`. No path *enumeration*, so cycles and fan-out
//!   are handled for free.
//!
//! Empty results (an endpoint that matches nothing, or a `Path` with
//! no connecting route) emit a stderr warning and contribute nothing,
//! leaving the exit code untouched (decision 3).
//!
//! Known POC gaps, deliberately left to surface in review:
//! - The reachable set can contain subgraph ids and non-root-candidate
//!   nodes that the final walk-order emit drops, so a path that travels
//!   *through* a subgraph border paints the inner nodes but not the
//!   subgraph itself (judgment B).
//! - Edge painting still uses the existing "either endpoint" rule, so a
//!   set boundary bleeds one edge outward (judgment A).

use std::collections::{HashMap, HashSet};

use unsnarl_root_query::{Direction, ParsedRootQuery, RootQuery};

use crate::highlight::node_matches_highlight_query::node_matches_highlight_query;
use crate::prune::iterate_visual_nodes::iterate_visual_nodes;
use crate::prune::resolve_ambiguous_queries;
use crate::visual_edge::VisualEdge;
use crate::visual_graph::VisualGraph;

pub fn collect_highlight_path_ids(graph: &VisualGraph, queries: &[RootQuery]) -> Vec<String> {
    if queries.is_empty() {
        return Vec::new();
    }
    let fwd = build_adjacency(&graph.edges, EdgeEnd::From);
    let bwd = build_adjacency(&graph.edges, EdgeEnd::To);

    let mut set: HashSet<String> = HashSet::new();
    for q in queries {
        match q {
            RootQuery::Single { query, .. } => {
                for id in seed_ids(graph, query) {
                    set.insert(id);
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

    emit_in_walk_order(graph, &set)
}

enum EdgeEnd {
    From,
    To,
}

/// Build a directed adjacency map keyed by the chosen endpoint. The
/// keys / values borrow the edge strings, so the map lives only as
/// long as `edges`.
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

/// Emit the highlighted ids in `iterate_visual_nodes` walk order with
/// duplicates removed, matching `collect_highlight_ids`'s contract so
/// the emitter's byte output stays deterministic.
fn emit_in_walk_order(graph: &VisualGraph, set: &HashSet<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut emitted: HashSet<String> = HashSet::new();
    iterate_visual_nodes(&graph.elements, &mut |node| {
        let id = node.id();
        if set.contains(id) && emitted.insert(id.to_string()) {
            out.push(id.to_string());
        }
    });
    out
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
