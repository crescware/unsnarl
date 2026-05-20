//! Breadth-first traversal helper for pruning.
//!
//! Mirrors `ts/src/visual-graph/prune/bfs.ts`.

use std::collections::{HashMap, HashSet};

pub fn bfs(
    starts: &HashSet<String>,
    adj: &HashMap<String, Vec<String>>,
    max_depth: i32,
) -> HashSet<String> {
    let mut reached: HashSet<String> = starts.clone();
    if max_depth <= 0 {
        return reached;
    }
    let mut frontier: HashSet<String> = starts.clone();
    let mut depth = 0;
    while depth < max_depth && !frontier.is_empty() {
        let mut next: HashSet<String> = HashSet::new();
        for id in &frontier {
            if let Some(neighbors) = adj.get(id) {
                for n in neighbors {
                    if !reached.contains(n) {
                        reached.insert(n.clone());
                        next.insert(n.clone());
                    }
                }
            }
        }
        frontier = next;
        depth += 1;
    }
    reached
}

#[cfg(test)]
#[path = "bfs_test.rs"]
mod bfs_test;
