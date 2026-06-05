//! Reachability collector for the `-H` / `--highlight` path /
//! direction grammar (issue #90).
//!
//! Given a list of [`RootQuery`] (point / path / direction), returns a
//! [`HighlightSelection`]: the ids to highlight, the point-query
//! subset, the `LineOrName` resolutions of the `..` endpoints, and any
//! no-match / no-path warnings. Reachability is computed over the
//! VisualGraph's `edges` (post-pruning) — the drawn graph, not the IR
//! (issue #90 decision 1). A path is therefore a path *in the diagram*;
//! `pruning` that removed an element removes it from the reachable set
//! for free, and nothing is ever painted outside the displayed graph.
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
//! Every `..` endpoint's `LineOrName` ambiguity is resolved up front in
//! a single pass; the resolutions ride out in [`HighlightSelection`] so
//! the caller can list them in the same Notice log `-r` uses.
//!
//! Empty results (an endpoint that matches nothing, or a `Path` with no
//! connecting route) are reported as [`HighlightWarning`]s and
//! contribute nothing, leaving the exit code untouched (decision 3).
//! The warnings are *returned* for the CLI layer to print, not written
//! to stderr from this library crate.

use std::collections::{HashMap, HashSet};

use unsnarl_root_query::{Direction, ParsedRootQuery, RootQuery};

use crate::highlight::node_matches_highlight_query::node_matches_highlight_query;
use crate::prune::iterate_visual_nodes::iterate_visual_nodes;
use crate::prune::{resolve_ambiguous_queries, RootQueryResolution};
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
    /// `LineOrName` resolutions for the `..` endpoints, in endpoint
    /// order. The caller merges these into the resolution notice log so
    /// a `10:foo` endpoint reports its line-vs-name decision exactly the
    /// way an `-r 10:foo` query already does.
    pub resolutions: Vec<RootQueryResolution>,
    /// No-match / no-path warnings, returned for the CLI layer to print
    /// on stderr (the library never writes to stderr itself).
    pub warnings: Vec<HighlightWarning>,
}

/// A highlight query that selected nothing worth painting.
#[derive(Debug, PartialEq)]
pub enum HighlightWarning {
    /// An endpoint of a `Direction` / `Path` query matched no node.
    NoMatch { raw: String },
    /// A `Path` query's two endpoints both matched, but no directed
    /// route connects them in either direction.
    NoPath { raw: String },
}

pub fn collect_highlight_path_ids(
    graph: &VisualGraph,
    queries: &[RootQuery],
) -> HighlightSelection {
    if queries.is_empty() {
        return HighlightSelection {
            ids: Vec::new(),
            point_ids: Vec::new(),
            resolutions: Vec::new(),
            warnings: Vec::new(),
        };
    }

    // Resolve every endpoint's `LineOrName` ambiguity in one pass, in
    // query order. `resolve_ambiguous_queries` decides each ambiguity
    // from the graph alone, so a batched resolve yields the same
    // per-endpoint result a per-endpoint resolve would — but produces a
    // single resolution log and walks the graph once.
    let endpoints = collect_endpoints(queries);
    let resolve = resolve_ambiguous_queries(graph, &endpoints);
    let resolved = resolve.resolved;
    let resolutions = resolve.resolutions;

    let fwd = build_adjacency(&graph.edges, EdgeEnd::From);
    let bwd = build_adjacency(&graph.edges, EdgeEnd::To);

    let mut set: HashSet<String> = HashSet::new();
    let mut point_set: HashSet<String> = HashSet::new();
    let mut warnings: Vec<HighlightWarning> = Vec::new();
    // `resolved` is parallel to `endpoints`, which was built by walking
    // `queries` in order: Single / Direction contribute one endpoint,
    // Path contributes two. The cursor advances in the same rhythm.
    let mut cursor = 0usize;
    for q in queries {
        match q {
            RootQuery::Single { .. } => {
                let seeds = seed_ids(graph, &resolved[cursor]);
                cursor += 1;
                for id in seeds {
                    set.insert(id.clone());
                    point_set.insert(id);
                }
            }
            RootQuery::Direction { dir, raw, .. } => {
                let seeds = seed_ids(graph, &resolved[cursor]);
                cursor += 1;
                if seeds.is_empty() {
                    warnings.push(HighlightWarning::NoMatch { raw: raw.clone() });
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
            RootQuery::Path { raw, .. } => {
                let l = seed_ids(graph, &resolved[cursor]);
                let r = seed_ids(graph, &resolved[cursor + 1]);
                cursor += 2;
                if l.is_empty() || r.is_empty() {
                    warnings.push(HighlightWarning::NoMatch { raw: raw.clone() });
                    continue;
                }
                let lf = reach(&fwd, &l);
                let lb = reach(&bwd, &l);
                let rf = reach(&fwd, &r);
                let rb = reach(&bwd, &r);
                // Connectivity is whether THIS path's intersection
                // produced any node, tracked locally — not whether the
                // shared accumulator grew. A path whose nodes a prior
                // query already painted leaves the accumulator's length
                // unchanged yet is genuinely connected, so a length
                // check would warn `NoPath` falsely.
                let mut connected = false;
                for id in &lf {
                    if rb.contains(id) {
                        set.insert(id.clone());
                        connected = true;
                    }
                }
                for id in &rf {
                    if lb.contains(id) {
                        set.insert(id.clone());
                        connected = true;
                    }
                }
                if !connected {
                    warnings.push(HighlightWarning::NoPath { raw: raw.clone() });
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
    HighlightSelection {
        ids,
        point_ids,
        resolutions,
        warnings,
    }
}

/// Flatten every query's endpoint(s) into one list, in query order, so
/// the batched resolve and the per-query walk index it identically.
fn collect_endpoints(queries: &[RootQuery]) -> Vec<ParsedRootQuery> {
    let mut endpoints: Vec<ParsedRootQuery> = Vec::new();
    for q in queries {
        match q {
            RootQuery::Single { query, .. } => endpoints.push(query.clone()),
            RootQuery::Direction { lhs, .. } => endpoints.push(lhs.clone()),
            RootQuery::Path { lhs, rhs, .. } => {
                endpoints.push(lhs.clone());
                endpoints.push(rhs.clone());
            }
        }
    }
    endpoints
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

/// Return every visible node id the (already-resolved) endpoint query
/// matches. Resolution happened once in [`collect_highlight_path_ids`],
/// so this is a pure match walk.
fn seed_ids(graph: &VisualGraph, query: &ParsedRootQuery) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    iterate_visual_nodes(&graph.elements, &mut |node| {
        if node_matches_highlight_query(node, query) {
            ids.push(node.id().to_string());
        }
    });
    ids
}

/// Emit the highlighted ids in element-tree walk order with duplicates
/// removed. Walks *every* element (nodes and subgraphs), so a subgraph
/// or non-root-candidate node in the set is emitted too. For point
/// queries the set holds only matched root-candidate nodes, so this
/// reproduces the classic point-highlight walk order byte-for-byte.
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

#[cfg(test)]
#[path = "collect_highlight_path_ids_test.rs"]
mod collect_highlight_path_ids_test;
