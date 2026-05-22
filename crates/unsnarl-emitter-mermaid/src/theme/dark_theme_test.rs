//! Pins the per-slot literals of the built-in dark theme so an
//! accidental rename / re-tint surfaces here.

use super::DARK_THEME;

#[test]
fn boundary_stub_keeps_the_original_dark_mode_literals() {
    assert_eq!(DARK_THEME.boundary_stub.stroke, "#888");
    assert_eq!(DARK_THEME.boundary_stub.stroke_dasharray, "3 3");
    assert_eq!(DARK_THEME.boundary_stub.color, "#888");
}

#[test]
fn var_node_keeps_the_original_dash_pattern() {
    assert_eq!(DARK_THEME.var_node.stroke_dasharray, "5 5");
}

#[test]
fn elk_empty_placeholder_keeps_the_transparent_fill_stroke_literals() {
    // fill / stroke stay transparent so the placeholder leaves no
    // rectangle around the "No nodes" label. Text color is
    // intentionally omitted so Mermaid's default (which adapts to
    // the background) keeps the label readable.
    assert_eq!(DARK_THEME.elk_empty_placeholder.fill, "transparent");
    assert_eq!(DARK_THEME.elk_empty_placeholder.stroke, "transparent");
}

#[test]
fn nest_palette_has_at_least_six_entries_to_keep_wrap_body_brightness_distinct() {
    // Each function consumes two adjacent slots (wrapper at N, body
    // at N+1) so the wrapper and body read as distinct brightness
    // levels. Six entries trade cycle-distance for per-step contrast;
    // this test guards against accidental shrinkage that would lose
    // contrast.
    assert!(DARK_THEME.nest_palette.len() >= 6);
}
