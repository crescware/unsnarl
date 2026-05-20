//! Build forward / reverse adjacency maps from a `VisualEdge` list.
//!
//! Mirrors `ts/src/visual-graph/prune/build-adjacency.ts`.

use std::collections::HashMap;

use crate::prune::push_to::push_to;
use crate::visual_edge::VisualEdge;

pub struct Adjacency {
    pub out_edges: HashMap<String, Vec<String>>,
    pub in_edges: HashMap<String, Vec<String>>,
}

pub fn build_adjacency(edges: &[VisualEdge]) -> Adjacency {
    let mut out_edges: HashMap<String, Vec<String>> = HashMap::new();
    let mut in_edges: HashMap<String, Vec<String>> = HashMap::new();
    for e in edges {
        push_to(&mut out_edges, &e.from, e.to.clone());
        push_to(&mut in_edges, &e.to, e.from.clone());
    }
    Adjacency {
        out_edges,
        in_edges,
    }
}

#[cfg(test)]
#[path = "build_adjacency_test.rs"]
mod build_adjacency_test;
