//! Both dagre and elk strategy suites land here because they share
//! a single enum.

use crate::theme::{DARK_THEME, LIGHT_THEME};

use super::{EmptySubgraphContext, MermaidStrategy};

// --- Dagre ---

#[test]
fn dagre_emits_no_preamble_lines_default_renderer() {
    assert_eq!(MermaidStrategy::Dagre.preamble_lines(), &[] as &[&str]);
}

#[test]
fn dagre_trailer_lines_returns_an_empty_list_regardless_of_placeholder_ids() {
    assert!(MermaidStrategy::Dagre
        .trailer_lines(&[], &DARK_THEME)
        .is_empty());
    let ids: Vec<String> = vec!["elk_empty_a".to_string(), "elk_empty_b".to_string()];
    assert!(MermaidStrategy::Dagre
        .trailer_lines(&ids, &DARK_THEME)
        .is_empty());
}

#[test]
fn dagre_empty_subgraph_placeholder_always_returns_none() {
    // dagre sizes empty clusters correctly on its own, so no
    // placeholder is needed regardless of how the subgraph is wired
    // to the rest of the graph.
    let result = MermaidStrategy::Dagre.empty_subgraph_placeholder(EmptySubgraphContext {
        subgraph_id: "s_scope_42",
        indent: "    ",
    });
    assert!(result.is_none());
}

// --- Elk ---

#[test]
fn elk_trailer_lines_returns_an_empty_list_when_there_are_no_placeholder_ids() {
    assert!(MermaidStrategy::Elk
        .trailer_lines(&[], &DARK_THEME)
        .is_empty());
}

#[test]
fn elk_trailer_lines_emits_classdef_from_dark_theme_and_one_class_line_per_id() {
    let ids: Vec<String> = vec!["elk_empty_a".to_string(), "elk_empty_b".to_string()];
    let out = MermaidStrategy::Elk.trailer_lines(&ids, &DARK_THEME);
    assert_eq!(
        out[0],
        "  classDef elkEmptyPlaceholder fill:transparent,stroke:transparent;"
    );
    assert!(out.contains(&"  class elk_empty_a elkEmptyPlaceholder;".to_string()));
    assert!(out.contains(&"  class elk_empty_b elkEmptyPlaceholder;".to_string()));
}

#[test]
fn elk_trailer_lines_routes_through_the_supplied_theme_so_placeholder_picks_up_theme_literals() {
    let ids: Vec<String> = vec!["elk_empty_x".to_string()];
    let out = MermaidStrategy::Elk.trailer_lines(&ids, &LIGHT_THEME);
    let expected = format!(
        "  classDef elkEmptyPlaceholder fill:{},stroke:{};",
        LIGHT_THEME.elk_empty_placeholder.fill, LIGHT_THEME.elk_empty_placeholder.stroke
    );
    assert_eq!(out[0], expected);
}

#[test]
fn elk_trailer_lines_does_not_emit_a_color_segment_in_the_elk_empty_placeholder_classdef() {
    // The placeholder is a layout marker, not a node. The classDef
    // must not pin a color literal because doing so would either
    // hide the "No nodes" label (color:transparent) or force a
    // hard-coded color that fights the active Mermaid theme.
    let ids: Vec<String> = vec!["elk_empty_x".to_string()];
    let out = MermaidStrategy::Elk.trailer_lines(&ids, &DARK_THEME);
    assert!(!out[0].contains("color:"));
}

#[test]
fn elk_empty_subgraph_placeholder_returns_a_placeholder_line_and_id_for_any_empty_subgraph() {
    let result = MermaidStrategy::Elk
        .empty_subgraph_placeholder(EmptySubgraphContext {
            subgraph_id: "s_scope_42",
            indent: "    ",
        })
        .expect("elk strategy always emits a placeholder");
    assert_eq!(result.line, r#"    elk_empty_s_scope_42["No nodes"]"#);
    assert_eq!(result.placeholder_id, "elk_empty_s_scope_42");
}

#[test]
fn elk_empty_subgraph_placeholder_propagates_the_indent_prefix_verbatim() {
    let result = MermaidStrategy::Elk
        .empty_subgraph_placeholder(EmptySubgraphContext {
            subgraph_id: "x",
            indent: "\t\t",
        })
        .expect("elk strategy always emits a placeholder");
    assert_eq!(result.line, "\t\telk_empty_x[\"No nodes\"]");
}

#[test]
fn elk_empty_subgraph_placeholder_id_is_derived_solely_from_the_subgraph_id() {
    let result = MermaidStrategy::Elk
        .empty_subgraph_placeholder(EmptySubgraphContext {
            subgraph_id: "cont_if_scope_0_99",
            indent: "  ",
        })
        .expect("elk strategy always emits a placeholder");
    assert_eq!(result.placeholder_id, "elk_empty_cont_if_scope_0_99");
}
