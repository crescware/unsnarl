//! Splits the edge list into the body bucket and the import bucket.
//!
//! Returns a `(body, imports)` tuple so callers destructure with a
//! one-line pattern.

use std::collections::HashSet;

use unsnarl_visual_graph::visual_edge::VisualEdge;

pub fn split_edges<'a>(
    edges: &'a [VisualEdge],
    import_source_ids: &HashSet<String>,
) -> (Vec<&'a VisualEdge>, Vec<&'a VisualEdge>) {
    let mut body: Vec<&'a VisualEdge> = Vec::new();
    let mut imports: Vec<&'a VisualEdge> = Vec::new();
    for e in edges {
        if import_source_ids.contains(&e.from) {
            imports.push(e);
        } else {
            body.push(e);
        }
    }
    (body, imports)
}

#[cfg(test)]
#[path = "split_edges_test.rs"]
mod split_edges_test;
