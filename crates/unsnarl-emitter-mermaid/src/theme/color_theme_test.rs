//! The test below builds a minimal `ColorTheme` value to act as a
//! compile-time fence — if a field is ever renamed, this expression
//! stops compiling and the test fails to build.

use super::{
    BoundaryStubColors, ColorTheme, EdgeSourceSubgraphColors, EdgeTargetSubgraphColors,
    ElkEmptyPlaceholderColors, HighlightColors, NestPaletteEntry, VarNodeColors,
};

static MIN_THEME: ColorTheme = ColorTheme {
    boundary_stub: BoundaryStubColors {
        stroke: "#888",
        stroke_dasharray: "3 3",
        color: "#888",
    },
    var_node: VarNodeColors {
        stroke_dasharray: "5 5",
    },
    elk_empty_placeholder: ElkEmptyPlaceholderColors {
        fill: "transparent",
        stroke: "transparent",
    },
    nest_palette: &[NestPaletteEntry {
        fill: "#111",
        stroke: "#222",
    }],
    edge_target_subgraph: EdgeTargetSubgraphColors { stroke: "#333" },
    edge_source_subgraph: EdgeSourceSubgraphColors { stroke: "#444" },
    highlight: HighlightColors {
        fill: "#ff0",
        stroke: "#cc0",
        color: "#000",
        edge_stroke: "#cc0",
        edge_stroke_width: "2px",
    },
};

#[test]
fn a_hand_built_theme_value_type_checks_against_color_theme() {
    assert_eq!(MIN_THEME.nest_palette.len(), 1);
}
