//! Emits the trailing `classDef` / `class` lines that style boundary
//! stubs, var nodes, and per-depth nest fills.

use std::collections::HashMap;

use crate::theme::ColorTheme;

pub fn render_class_defs(
    stub_ids: &[String],
    var_ids: &[String],
    nest_class_map: &HashMap<usize, Vec<String>>,
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
