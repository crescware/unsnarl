//! `ColorTheme`: every CSS-style value the mermaid emitter splices
//! into `classDef` / `style` / `linkStyle` lines.
//!
//! Every field is `&'static str` so concrete themes can sit in
//! `static` storage and be referenced by `&'static ColorTheme`
//! from the emitter.

pub struct BoundaryStubColors {
    pub stroke: &'static str,
    pub stroke_dasharray: &'static str,
    pub color: &'static str,
}

pub struct VarNodeColors {
    pub stroke_dasharray: &'static str,
}

pub struct ElkEmptyPlaceholderColors {
    pub fill: &'static str,
    pub stroke: &'static str,
}

pub struct NestPaletteEntry {
    pub fill: &'static str,
    pub stroke: &'static str,
}

pub struct HighlightColors {
    pub fill: &'static str,
    pub stroke: &'static str,
    pub color: &'static str,
    pub edge_stroke: &'static str,
    pub edge_stroke_width: &'static str,
}

pub struct ColorTheme {
    pub boundary_stub: BoundaryStubColors,
    pub var_node: VarNodeColors,
    pub elk_empty_placeholder: ElkEmptyPlaceholderColors,
    /// Cycled per subgraph depth (1-based). Empty is invalid: every
    /// theme must provide at least one palette entry so cycling has
    /// a target.
    pub nest_palette: &'static [NestPaletteEntry],
    pub highlight: HighlightColors,
}

#[cfg(test)]
#[path = "color_theme_test.rs"]
mod color_theme_test;
