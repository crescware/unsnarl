//! Maps a 1-based subgraph depth to a 0-based palette index by
//! cycling.
//!
//! Mirrors `ts/src/emitter/mermaid/theme/nest-palette-index.ts`. The
//! TS function throws on invalid inputs; the Rust port mirrors that
//! with `panic!` so a mis-wired emitter fails loudly rather than
//! silently picking the wrong color slot.

pub fn nest_palette_index(depth: u32, palette_length: usize) -> usize {
    if palette_length == 0 {
        panic!("paletteLength must be > 0");
    }
    if depth < 1 {
        panic!("depth must be >= 1");
    }
    ((depth as usize) - 1) % palette_length
}

#[cfg(test)]
#[path = "nest_palette_index_test.rs"]
mod nest_palette_index_test;
