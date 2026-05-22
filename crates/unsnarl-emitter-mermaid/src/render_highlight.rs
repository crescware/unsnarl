//! Emits inline `style` rows for highlighted nodes plus a single
//! `linkStyle` row for any edges that touch them.
//!
//! Inline `style` wins against `classDef` declarations applied via
//! `class`, which is why we don't try to define a "highlightNode"
//! class -- a class would lose to anything mermaid applied earlier
//! in the diagram source.
//!
//! `highlight_ids` is a slice (not a `HashSet`) because the emitted
//! `style` rows must be in element-tree walk order. The upstream
//! `collect_highlight_ids` already produces ids in that order;
//! `HashSet` has unspecified iteration order so it cannot be used
//! to carry them through.

use crate::theme::ColorTheme;

pub fn render_highlight(
    highlight_ids: &[String],
    highlight_edge_indices: &[usize],
    theme: &ColorTheme,
    lines: &mut Vec<String>,
) {
    if highlight_ids.is_empty() {
        return;
    }
    let h = &theme.highlight;
    for id in highlight_ids {
        lines.push(format!(
            "  style {id} fill:{},stroke:{},color:{};",
            h.fill, h.stroke, h.color
        ));
    }
    if !highlight_edge_indices.is_empty() {
        let list = highlight_edge_indices
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",");
        lines.push(format!(
            "  linkStyle {list} stroke:{},stroke-width:{};",
            h.edge_stroke, h.edge_stroke_width
        ));
    }
}

#[cfg(test)]
#[path = "render_highlight_test.rs"]
mod render_highlight_test;
