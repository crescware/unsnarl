//! Emits the dashed boundary-stub triplet for each pruned-away
//! neighbor.
//!
//! Mirrors `ts/src/emitter/mermaid/render-boundary-edges.ts`. The
//! Rust port walks the same `boundary_edges` list and pushes the
//! same `boundary_stub_N` ids; the surface used by `render_class_defs`
//! to attach the dashed-circle class.

use unsnarl_visual_graph::visual_boundary_edge::VisualBoundaryEdge;
use unsnarl_visual_graph::visual_graph::VisualGraph;

pub fn render_boundary_edges(
    graph: &VisualGraph,
    lines: &mut Vec<String>,
    stub_ids: &mut Vec<String>,
) {
    if graph.boundary_edges.is_empty() {
        return;
    }
    // Pruning detected one or more neighbors past the requested
    // radius. Mermaid cannot draw a truly dangling edge, so each
    // boundary edge gets a faint stub node "(...)" attached via a
    // dashed arrow. The stub stands in for "more graph keeps going
    // beyond here". The label question follows the edge semantics
    // `from -label-> to`, where the label describes the action `to`
    // performs on `from`:
    //
    // - "out" (`inside -> stub`): the actor is the stub, which is
    //   unknown, so we cannot honestly attach a label.
    // - "in"  (`stub -> inside`): the actor is the kept inside
    //   node, so we keep the original label.
    let mut stub_counter = 0u32;
    for be in &graph.boundary_edges {
        stub_counter += 1;
        let stub_id = format!("boundary_stub_{stub_counter}");
        stub_ids.push(stub_id.clone());
        // ASCII "..." instead of U+2026 -- some Mermaid renderers
        // stumble on multibyte glyphs inside node shape syntax.
        lines.push(format!("  {stub_id}((...))"));
        match be {
            VisualBoundaryEdge::Out { inside, .. } => {
                lines.push(format!("  {inside} -.-> {stub_id}"));
            }
            VisualBoundaryEdge::In { inside, label, .. } => {
                lines.push(format!("  {stub_id} -.->|{label}| {inside}"));
            }
        }
    }
}
