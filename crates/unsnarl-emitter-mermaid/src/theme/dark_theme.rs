//! The built-in dark color theme.

use super::color_theme::{
    BoundaryStubColors, ColorTheme, ElkEmptyPlaceholderColors, HighlightColors, NestPaletteEntry,
    VarNodeColors,
};

pub static DARK_THEME: ColorTheme = ColorTheme {
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
    nest_palette: &[
        NestPaletteEntry {
            fill: "#11192a",
            stroke: "transparent",
        },
        NestPaletteEntry {
            fill: "#1a2538",
            stroke: "transparent",
        },
        NestPaletteEntry {
            fill: "#243047",
            stroke: "transparent",
        },
        NestPaletteEntry {
            fill: "#2d3b57",
            stroke: "transparent",
        },
        NestPaletteEntry {
            fill: "#364666",
            stroke: "transparent",
        },
        NestPaletteEntry {
            fill: "#3f5175",
            stroke: "transparent",
        },
    ],
    highlight: HighlightColors {
        fill: "#facc15",
        stroke: "#facc15",
        color: "#0a0a0a",
        edge_stroke: "#facc15",
        edge_stroke_width: "2px",
    },
};

#[cfg(test)]
#[path = "dark_theme_test.rs"]
mod dark_theme_test;
