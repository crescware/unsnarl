//! The built-in light color theme.
//!
//! Mirrors `ts/src/emitter/mermaid/theme/light-theme.ts`.

use super::color_theme::{
    BoundaryStubColors, ColorTheme, ElkEmptyPlaceholderColors, HighlightColors, NestPaletteEntry,
    VarNodeColors,
};

pub static LIGHT_THEME: ColorTheme = ColorTheme {
    boundary_stub: BoundaryStubColors {
        stroke: "#555",
        stroke_dasharray: "3 3",
        color: "#555",
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
            fill: "#f4f7fb",
            stroke: "transparent",
        },
        NestPaletteEntry {
            fill: "#e8eff7",
            stroke: "transparent",
        },
        NestPaletteEntry {
            fill: "#dce6f3",
            stroke: "transparent",
        },
        NestPaletteEntry {
            fill: "#d1ddef",
            stroke: "transparent",
        },
        NestPaletteEntry {
            fill: "#c5d4eb",
            stroke: "transparent",
        },
        NestPaletteEntry {
            fill: "#b9cbe7",
            stroke: "transparent",
        },
    ],
    highlight: HighlightColors {
        fill: "#fde047",
        stroke: "#ca8a04",
        color: "#0a0a0a",
        edge_stroke: "#ca8a04",
        edge_stroke_width: "2px",
    },
};

#[cfg(test)]
#[path = "light_theme_test.rs"]
mod light_theme_test;
