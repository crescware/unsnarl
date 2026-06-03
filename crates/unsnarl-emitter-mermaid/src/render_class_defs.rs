//! Emits the trailing `classDef` / `class` lines that style boundary
//! stubs, var nodes, per-depth nest fills, and the visible-border
//! treatment for subgraphs whose perimeter is the terminus *or* the
//! origin of an ordinary edge.

use std::collections::{HashMap, HashSet};

use crate::theme::ColorTheme;

pub fn render_class_defs(
    stub_ids: &[String],
    var_ids: &[String],
    nest_class_map: &HashMap<usize, Vec<String>>,
    edge_target_subgraph_ids: &HashSet<String>,
    edge_source_subgraph_ids: &HashSet<String>,
    theme: &ColorTheme,
    lines: &mut Vec<String>,
) {
    if !stub_ids.is_empty() {
        let c = &theme.boundary_stub;
        // No fill: stubs inherit the default Mermaid node background
        // so they share a backdrop with regular nodes rather than
        // letting the parent subgraph fill bleed through.
        lines.push(format!(
            "  classDef boundaryStub stroke:{},stroke-dasharray:{},color:{};",
            c.stroke, c.stroke_dasharray, c.color
        ));
        for id in stub_ids {
            lines.push(format!("  class {id} boundaryStub;"));
        }
    }
    if !var_ids.is_empty() {
        // var-declared Variable nodes carry no edges in the visual
        // graph (their references are filtered out upstream). Render
        // the border dashed so the reader does not mistake them for
        // ordinary nodes that happen to be unconnected.
        lines.push(format!(
            "  classDef varNode stroke-dasharray:{};",
            theme.var_node.stroke_dasharray
        ));
        for id in var_ids {
            lines.push(format!("  class {id} varNode;"));
        }
    }
    emit_nest_class_defs(nest_class_map, theme, lines);
    emit_edge_target_subgraph_class_def(edge_target_subgraph_ids, nest_class_map, theme, lines);
    emit_edge_source_subgraph_class_def(
        edge_source_subgraph_ids,
        edge_target_subgraph_ids,
        nest_class_map,
        theme,
        lines,
    );
}

/// When at least one subgraph is an edge terminus, emit a
/// single `classDef edgeTargetSubgraph` plus per-id `class`
/// assignments. The class is applied *in addition to* the
/// per-depth `nestL<N>` class on the same id; Mermaid merges
/// styles across multiple `class` lines so the fill stays
/// per-depth while the stroke comes from this class. Subgraph ids
/// are iterated in the depth-then-emit-order the `nestL<N>`
/// classDefs already use so the new `class` lines line up under
/// their matching `class … nestL<N>` line, keeping the on-disk
/// output deterministic.
fn emit_edge_target_subgraph_class_def(
    edge_target_subgraph_ids: &HashSet<String>,
    nest_class_map: &HashMap<usize, Vec<String>>,
    theme: &ColorTheme,
    lines: &mut Vec<String>,
) {
    if edge_target_subgraph_ids.is_empty() {
        return;
    }
    lines.push(format!(
        "  classDef edgeTargetSubgraph stroke:{};",
        theme.edge_target_subgraph.stroke
    ));
    let palette_length = theme.nest_palette.len();
    for slot in 0..palette_length {
        let Some(ids) = nest_class_map.get(&slot) else {
            continue;
        };
        for id in ids {
            if edge_target_subgraph_ids.contains(id) {
                lines.push(format!("  class {id} edgeTargetSubgraph;"));
            }
        }
    }
}

/// Mirror of [`emit_edge_target_subgraph_class_def`] for subgraphs
/// that are an edge *origin* rather than a terminus. A subgraph that
/// is already an edge target keeps its `edgeTargetSubgraph` stroke
/// and is skipped here, so the two classes never paint the same id
/// twice and only source-*only* subgraphs receive
/// `edgeSourceSubgraph`. Ids are iterated in the same
/// depth-then-emit order the `nestL<N>` classDefs use so the new
/// `class` lines stay deterministic.
fn emit_edge_source_subgraph_class_def(
    edge_source_subgraph_ids: &HashSet<String>,
    edge_target_subgraph_ids: &HashSet<String>,
    nest_class_map: &HashMap<usize, Vec<String>>,
    theme: &ColorTheme,
    lines: &mut Vec<String>,
) {
    let has_source_only = edge_source_subgraph_ids
        .iter()
        .any(|id| !edge_target_subgraph_ids.contains(id));
    if !has_source_only {
        return;
    }
    lines.push(format!(
        "  classDef edgeSourceSubgraph stroke:{};",
        theme.edge_source_subgraph.stroke
    ));
    let palette_length = theme.nest_palette.len();
    for slot in 0..palette_length {
        let Some(ids) = nest_class_map.get(&slot) else {
            continue;
        };
        for id in ids {
            if edge_source_subgraph_ids.contains(id) && !edge_target_subgraph_ids.contains(id) {
                lines.push(format!("  class {id} edgeSourceSubgraph;"));
            }
        }
    }
}

fn emit_nest_class_defs(
    nest_class_map: &HashMap<usize, Vec<String>>,
    theme: &ColorTheme,
    lines: &mut Vec<String>,
) {
    let palette_length = theme.nest_palette.len();
    if palette_length == 0 {
        return;
    }
    // Iterate slots in ascending palette order so the output
    // ordering is deterministic regardless of insertion order into
    // the map.
    for slot in 0..palette_length {
        let Some(ids) = nest_class_map.get(&slot) else {
            continue;
        };
        if ids.is_empty() {
            continue;
        }
        // The class name uses 1-based palette slots so the
        // user-facing class matches the "depth" the user reasons
        // about (L1 = outermost subgraph).
        let level = slot + 1;
        let c = &theme.nest_palette[slot];
        lines.push(format!(
            "  classDef nestL{level} fill:{},stroke:{};",
            c.fill, c.stroke
        ));
        for id in ids {
            lines.push(format!("  class {id} nestL{level};"));
        }
    }
}

#[cfg(test)]
#[path = "render_class_defs_test.rs"]
mod render_class_defs_test;
