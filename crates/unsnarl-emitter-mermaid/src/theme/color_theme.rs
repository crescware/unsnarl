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

/// Stroke applied as a *second* class to subgraphs that appear as
/// the terminus of at least one ordinary edge in the rendered
/// graph (e.g. a `CallProxy` subgraph receiving `read,call` from
/// its callee binding). Subgraphs that no edge terminates on keep
/// the per-depth `nestL<N>` class's transparent stroke and remain
/// border-less, so the visible border becomes a positive signal of
/// "an edge actually lands here".
pub struct EdgeTargetSubgraphColors {
    pub stroke: &'static str,
}

/// Stroke applied as a *second* class to subgraphs that appear as
/// the *origin* (`from`) of at least one ordinary edge — the mirror
/// of [`EdgeTargetSubgraphColors`] for the opposite endpoint, so an
/// arrow leaving a subgraph draws the same visible border as one
/// arriving. A subgraph that is already an edge target keeps its
/// `edgeTargetSubgraph` stroke and is not styled again here, so the
/// two classes never paint the same id twice.
pub struct EdgeSourceSubgraphColors {
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
    pub edge_target_subgraph: EdgeTargetSubgraphColors,
    pub edge_source_subgraph: EdgeSourceSubgraphColors,
    pub highlight: HighlightColors,
}

#[cfg(test)]
#[path = "color_theme_test.rs"]
mod color_theme_test;
