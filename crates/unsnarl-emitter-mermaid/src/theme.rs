//! Color themes for the mermaid emitter.
//!
//! The two built-in themes ([`DARK_THEME`], [`LIGHT_THEME`]) cover
//! the `--color-theme dark` / `--color-theme light` CLI choices; the
//! mapping from CLI value to theme lives in the `unsnarl` crate so
//! this crate does not depend on the CLI layer.

pub mod color_theme;
pub mod dark_theme;
pub mod light_theme;
pub mod nest_palette_index;

pub use color_theme::{
    BoundaryStubColors, ColorTheme, ElkEmptyPlaceholderColors, HighlightColors, NestPaletteEntry,
    VarNodeColors,
};
pub use dark_theme::DARK_THEME;
pub use light_theme::LIGHT_THEME;
pub use nest_palette_index::nest_palette_index;
