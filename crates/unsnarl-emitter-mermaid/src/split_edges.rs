//! Splits the edge list into the body bucket and the import bucket.
//!
//! Mirrors `ts/src/emitter/mermaid/split-edges.ts`. The TS port
//! returns an object with two arrays; the Rust port returns a
//! tuple so callers destructure with the same one-line pattern.

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
