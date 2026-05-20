//! Appends a subgraph id to its 1-based depth's palette slot in
//! `RenderState.nest_class_map`.
//!
//! Mirrors `ts/src/emitter/mermaid/record-nest-slot.ts`. Idempotent
//! on empty palettes (no-op when the theme defines an empty nest
//! palette, which is invalid but defended against to keep the
//! renderer robust). Used by both the function-wrapper and the
//! plain-subgraph paths so they share the same depth bookkeeping.

use crate::render_state::RenderState;
use crate::theme::nest_palette_index;

pub fn record_nest_slot(state: &mut RenderState<'_>, subgraph_id: &str, depth: u32) {
    let palette_length = state.theme.nest_palette.len();
    if palette_length == 0 {
        return;
    }
    let slot = nest_palette_index(depth, palette_length);
    state
        .nest_class_map
        .entry(slot)
        .or_default()
        .push(subgraph_id.to_string());
}
