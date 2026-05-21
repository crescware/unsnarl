//! Mirrors `ts/src/emitter/mermaid/theme/light-theme.test.ts`. Pins
//! that the light theme has every slot populated and shares the
//! same transparent elkEmptyPlaceholder workaround as the dark theme.

use super::LIGHT_THEME;

#[test]
fn every_class_def_slot_is_populated() {
    assert!(!LIGHT_THEME.boundary_stub.stroke.is_empty());
    assert!(!LIGHT_THEME.boundary_stub.stroke_dasharray.is_empty());
    assert!(!LIGHT_THEME.boundary_stub.color.is_empty());
    assert!(!LIGHT_THEME.var_node.stroke_dasharray.is_empty());
    assert!(!LIGHT_THEME.elk_empty_placeholder.fill.is_empty());
    assert!(!LIGHT_THEME.elk_empty_placeholder.stroke.is_empty());
}

#[test]
fn nest_palette_has_at_least_six_entries_to_keep_wrap_body_brightness_distinct() {
    assert!(LIGHT_THEME.nest_palette.len() >= 6);
}

#[test]
fn elk_empty_placeholder_has_transparent_fill_and_stroke_same_workaround_as_dark_theme() {
    // The placeholder is a layout-only hack — it is not a node. fill
    // and stroke stay transparent so no rectangle is drawn around
    // the label. Text color is left to Mermaid's default so the
    // "No nodes" label stays readable against whichever subgraph
    // background it lands on.
    assert_eq!(LIGHT_THEME.elk_empty_placeholder.fill, "transparent");
    assert_eq!(LIGHT_THEME.elk_empty_placeholder.stroke, "transparent");
}
