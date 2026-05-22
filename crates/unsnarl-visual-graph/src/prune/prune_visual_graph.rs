//! Core pruning orchestration.
//!
//! Takes a base [`VisualGraph`] and a list of resolved queries,
//! walks the graph to determine the surviving node / edge set, and
//! assembles a new [`VisualGraph`] plus per-query match counts.

use std::collections::{HashMap, HashSet};

use unsnarl_root_query::ParsedRootQuery;

use crate::boundary_edge_direction::{BoundaryEdgeDirectionIn, BoundaryEdgeDirectionOut};
use crate::prune::bfs::bfs;
use crate::prune::build_adjacency::build_adjacency;
use crate::prune::build_parent_map::build_parent_map;
use crate::prune::collect_ids::collect_ids;
use crate::prune::collect_node_ids::collect_node_ids;
use crate::prune::iterate_visual_nodes::iterate_visual_nodes;
use crate::prune::iterate_visual_subgraphs::iterate_visual_subgraphs;
use crate::prune::node_matches_query::node_matches_query;
use crate::prune::prune_options::{PerQueryMatch, PruneOptions, PruneResult};
use crate::prune::rebuild_elements::rebuild_elements;
use crate::visual_boundary_edge::VisualBoundaryEdge;
use crate::visual_edge::VisualEdge;
use crate::visual_graph::{VisualGraph, VisualGraphSource};
use crate::visual_graph_pruning::{PruningRoot, VisualGraphPruning};

enum Bucket {
    Out { inside: String },
    In { inside: String, labels: Vec<String> },
}

pub fn prune_visual_graph(graph: &VisualGraph, options: &PruneOptions) -> PruneResult {
    if options.roots.is_empty() {
        return PruneResult {
            graph: clone_graph(graph),
            per_query: Vec::new(),
            root_ids: Vec::new(),
        };
    }

    let mut per_query: Vec<PerQueryMatch> = options
        .roots
        .iter()
        .map(|q| PerQueryMatch {
            query: clone_query(q),
            matched: 0,
        })
        .collect();
    // Two parallel containers: a Vec preserves the
    // `iterate_visual_nodes` walk order while the HashSet keeps
    // lookups O(1) during the subgraph sweep and during BFS.
    let mut root_ids: Vec<String> = Vec::new();
    let mut root_ids_set: HashSet<String> = HashSet::new();

    iterate_visual_nodes(&graph.elements, &mut |node| {
        for (i, q) in options.roots.iter().enumerate() {
            if node_matches_query(node, q) {
                if root_ids_set.insert(node.id().to_string()) {
                    root_ids.push(node.id().to_string());
                }
                per_query[i].matched += 1;
            }
        }
    });

    // A bare line query whose number is the start line of a subgraph
    // sweeps every node in that subgraph into the root set.
    for (i, q) in options.roots.iter().enumerate() {
        let ParsedRootQuery::Line { line: q_line, .. } = q else {
            continue;
        };
        iterate_visual_subgraphs(&graph.elements, &mut |sg| {
            if sg.line() != q_line.0 {
                return;
            }
            let mut added: u32 = 0;
            for id in collect_node_ids(sg.elements()) {
                if root_ids_set.insert(id.clone()) {
                    root_ids.push(id);
                    added += 1;
                }
            }
            if added > 0 {
                per_query[i].matched += added;
            }
        });
    }

    let adj = build_adjacency(&graph.edges);
    // BFS only as far as the user asked. The outermost edges that
    // point beyond this radius are surfaced separately as
    // boundaryEdges so renderers can hint at "more context exists in
    // this direction" without pulling the next generation of nodes
    // into the diagram.
    let inner_d = bfs(&root_ids_set, &adj.out_edges, options.descendants as i32);
    let inner_a = bfs(&root_ids_set, &adj.in_edges, options.ancestors as i32);
    let mut reachable: HashSet<String> = root_ids_set
        .iter()
        .chain(inner_d.iter())
        .chain(inner_a.iter())
        .cloned()
        .collect();

    // Boundary edges are "more graph beyond here" hints. They are not
    // about counting individual outgoing edges -- one inside node with
    // 100 cut neighbors should still produce a single hint, not 100. So
    // collapse on (inside, direction); for "in" we additionally union
    // the labels (split by comma so "read,call" + "read" yields
    // {call, read}). The bucket list is kept in insertion order (the
    // order keys were first observed during the edge walk) so the
    // emitted boundaryEdges slice stays stable against the IR parity
    // baselines.
    let mut bucket_order: Vec<String> = Vec::new();
    let mut buckets: HashMap<String, Bucket> = HashMap::new();

    if options.descendants > 0 {
        for e in &graph.edges {
            if !reachable.contains(&e.from) || reachable.contains(&e.to) {
                continue;
            }
            let key = format!("out|{}", e.from);
            if !buckets.contains_key(&key) {
                buckets.insert(
                    key.clone(),
                    Bucket::Out {
                        inside: e.from.clone(),
                    },
                );
                bucket_order.push(key);
            }
        }
    }
    if options.ancestors > 0 {
        for e in &graph.edges {
            if !reachable.contains(&e.to) || reachable.contains(&e.from) {
                continue;
            }
            let key = format!("in|{}", e.to);
            if !buckets.contains_key(&key) {
                buckets.insert(
                    key.clone(),
                    Bucket::In {
                        inside: e.to.clone(),
                        labels: Vec::new(),
                    },
                );
                bucket_order.push(key.clone());
            }
            if let Some(Bucket::In { labels, .. }) = buckets.get_mut(&key) {
                for part in e.label.split(',') {
                    if !labels.iter().any(|l| l == part) {
                        labels.push(part.to_string());
                    }
                }
            }
        }
    }

    let mut boundary_edges: Vec<VisualBoundaryEdge> = Vec::new();
    for key in bucket_order {
        let Some(bucket) = buckets.remove(&key) else {
            continue;
        };
        match bucket {
            Bucket::Out { inside } => {
                boundary_edges.push(VisualBoundaryEdge::Out {
                    inside,
                    direction: BoundaryEdgeDirectionOut::Out,
                });
            }
            Bucket::In { inside, labels } => {
                let mut sorted = labels.clone();
                sorted.sort();
                let label = sorted.join(",");
                boundary_edges.push(VisualBoundaryEdge::In {
                    inside,
                    direction: BoundaryEdgeDirectionIn::In,
                    label,
                });
            }
        }
    }

    let parent_of = build_parent_map(&graph.elements);
    let initial: Vec<String> = reachable.iter().cloned().collect();
    for id in initial {
        let mut cur = parent_of.get(&id).cloned();
        while let Some(parent_id) = cur {
            if reachable.contains(&parent_id) {
                break;
            }
            reachable.insert(parent_id.clone());
            cur = parent_of.get(&parent_id).cloned();
        }
    }

    let new_elements = rebuild_elements(&graph.elements, &reachable);
    let survivors = collect_ids(&new_elements);
    let new_edges: Vec<VisualEdge> = graph
        .edges
        .iter()
        .filter(|v| survivors.contains(&v.from) && survivors.contains(&v.to))
        .cloned()
        .collect();
    let surviving_boundary: Vec<VisualBoundaryEdge> = boundary_edges
        .into_iter()
        .filter(|v| match v {
            VisualBoundaryEdge::Out { inside, .. } | VisualBoundaryEdge::In { inside, .. } => {
                survivors.contains(inside)
            }
        })
        .collect();

    let pruned = VisualGraph {
        version: graph.version,
        source: VisualGraphSource {
            path: graph.source.path.clone(),
            language: graph.source.language,
        },
        direction: graph.direction,
        elements: new_elements,
        edges: new_edges,
        boundary_edges: surviving_boundary,
        pruning: Some(VisualGraphPruning {
            roots: per_query
                .iter()
                .map(|p| PruningRoot {
                    query: query_raw(&p.query).to_string(),
                    matched: p.matched,
                })
                .collect(),
            descendants: options.descendants,
            ancestors: options.ancestors,
        }),
    };

    PruneResult {
        graph: pruned,
        per_query,
        root_ids,
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

fn clone_graph(graph: &VisualGraph) -> VisualGraph {
    VisualGraph {
        version: graph.version,
        source: VisualGraphSource {
            path: graph.source.path.clone(),
            language: graph.source.language,
        },
        direction: graph.direction,
        elements: graph.elements.clone(),
        edges: graph.edges.clone(),
        boundary_edges: graph.boundary_edges.clone(),
        pruning: graph.pruning.clone(),
    }
}

#[cfg(test)]
#[path = "prune_visual_graph_test.rs"]
mod prune_visual_graph_test;
