use std::collections::{HashMap, HashSet};

use super::render_class_defs;
use crate::theme::{DARK_THEME, LIGHT_THEME};

fn empty_targets() -> HashSet<String> {
    HashSet::new()
}

fn empty_nest_map() -> HashMap<usize, Vec<String>> {
    HashMap::new()
}

#[test]
fn emits_nothing_when_all_id_lists_and_the_nest_map_are_empty() {
    let mut lines: Vec<String> = Vec::new();
    render_class_defs(
        &[],
        &[],
        &empty_nest_map(),
        &empty_targets(),
        &DARK_THEME,
        &mut lines,
    );
    assert!(lines.is_empty());
}

#[test]
fn emits_the_boundary_stub_class_def_without_a_fill() {
    let mut lines: Vec<String> = Vec::new();
    let stub_ids = vec!["stub_1".to_string(), "stub_2".to_string()];
    render_class_defs(
        &stub_ids,
        &[],
        &empty_nest_map(),
        &empty_targets(),
        &DARK_THEME,
        &mut lines,
    );
    assert_eq!(
        lines,
        vec![
            "  classDef boundaryStub stroke:#888,stroke-dasharray:3 3,color:#888;".to_string(),
            "  class stub_1 boundaryStub;".to_string(),
            "  class stub_2 boundaryStub;".to_string(),
        ]
    );
}

#[test]
fn emits_the_var_node_class_def_from_the_dark_theme_with_the_original_dash_pattern() {
    let mut lines: Vec<String> = Vec::new();
    let var_ids = vec!["v_one".to_string(), "v_two".to_string()];
    render_class_defs(
        &[],
        &var_ids,
        &empty_nest_map(),
        &empty_targets(),
        &DARK_THEME,
        &mut lines,
    );
    assert_eq!(
        lines,
        vec![
            "  classDef varNode stroke-dasharray:5 5;".to_string(),
            "  class v_one varNode;".to_string(),
            "  class v_two varNode;".to_string(),
        ]
    );
}

#[test]
fn emits_boundary_stub_and_var_node_together_when_both_lists_are_non_empty() {
    let mut lines: Vec<String> = Vec::new();
    let stub_ids = vec!["stub_1".to_string()];
    let var_ids = vec!["v_one".to_string()];
    render_class_defs(
        &stub_ids,
        &var_ids,
        &empty_nest_map(),
        &empty_targets(),
        &DARK_THEME,
        &mut lines,
    );
    assert_eq!(
        lines,
        vec![
            "  classDef boundaryStub stroke:#888,stroke-dasharray:3 3,color:#888;".to_string(),
            "  class stub_1 boundaryStub;".to_string(),
            "  classDef varNode stroke-dasharray:5 5;".to_string(),
            "  class v_one varNode;".to_string(),
        ]
    );
}

#[test]
fn routes_through_the_supplied_theme_so_a_light_theme_produces_its_own_literals() {
    let mut lines: Vec<String> = Vec::new();
    let stub_ids = vec!["stub_1".to_string()];
    render_class_defs(
        &stub_ids,
        &[],
        &empty_nest_map(),
        &empty_targets(),
        &LIGHT_THEME,
        &mut lines,
    );
    let b = &LIGHT_THEME.boundary_stub;
    let expected = format!(
        "  classDef boundaryStub stroke:{},stroke-dasharray:{},color:{};",
        b.stroke, b.stroke_dasharray, b.color
    );
    assert!(lines.contains(&expected));
}

#[test]
fn emits_per_level_nest_class_defs_in_palette_slot_order_with_one_based_names() {
    let mut lines: Vec<String> = Vec::new();
    let mut nest_map: HashMap<usize, Vec<String>> = HashMap::new();
    nest_map.insert(0, vec!["s_outer".to_string()]);
    nest_map.insert(1, vec!["s_mid".to_string()]);
    nest_map.insert(2, vec!["s_inner".to_string()]);
    render_class_defs(
        &[],
        &[],
        &nest_map,
        &empty_targets(),
        &DARK_THEME,
        &mut lines,
    );
    let p = DARK_THEME.nest_palette;
    let expected: Vec<String> = vec![
        format!(
            "  classDef nestL1 fill:{},stroke:{};",
            p[0].fill, p[0].stroke
        ),
        "  class s_outer nestL1;".to_string(),
        format!(
            "  classDef nestL2 fill:{},stroke:{};",
            p[1].fill, p[1].stroke
        ),
        "  class s_mid nestL2;".to_string(),
        format!(
            "  classDef nestL3 fill:{},stroke:{};",
            p[2].fill, p[2].stroke
        ),
        "  class s_inner nestL3;".to_string(),
    ];
    assert_eq!(lines, expected);
}

#[test]
fn emits_slots_in_ascending_palette_order_regardless_of_insertion_order() {
    let mut lines: Vec<String> = Vec::new();
    let mut nest_map: HashMap<usize, Vec<String>> = HashMap::new();
    nest_map.insert(2, vec!["s_inner".to_string()]);
    nest_map.insert(0, vec!["s_outer".to_string()]);
    nest_map.insert(1, vec!["s_mid".to_string()]);
    render_class_defs(
        &[],
        &[],
        &nest_map,
        &empty_targets(),
        &DARK_THEME,
        &mut lines,
    );
    let headers: Vec<&String> = lines
        .iter()
        .filter(|v| v.contains("classDef nestL"))
        .collect();
    let p = DARK_THEME.nest_palette;
    let expected: Vec<String> = vec![
        format!(
            "  classDef nestL1 fill:{},stroke:{};",
            p[0].fill, p[0].stroke
        ),
        format!(
            "  classDef nestL2 fill:{},stroke:{};",
            p[1].fill, p[1].stroke
        ),
        format!(
            "  classDef nestL3 fill:{},stroke:{};",
            p[2].fill, p[2].stroke
        ),
    ];
    let got: Vec<String> = headers.into_iter().cloned().collect();
    assert_eq!(got, expected);
}

#[test]
fn skips_slots_that_have_no_subgraph_ids() {
    let mut lines: Vec<String> = Vec::new();
    let mut nest_map: HashMap<usize, Vec<String>> = HashMap::new();
    nest_map.insert(0, vec!["s_outer".to_string()]);
    nest_map.insert(2, vec!["s_far".to_string()]);
    render_class_defs(
        &[],
        &[],
        &nest_map,
        &empty_targets(),
        &DARK_THEME,
        &mut lines,
    );
    assert!(!lines.iter().any(|v| v.contains("nestL2")));
    assert!(lines.iter().any(|v| v.contains("nestL1")));
    assert!(lines.iter().any(|v| v.contains("nestL3")));
}

#[test]
fn places_a_function_wrapper_id_alongside_other_subgraphs_in_the_same_palette_slot() {
    let mut lines: Vec<String> = Vec::new();
    let mut nest_map: HashMap<usize, Vec<String>> = HashMap::new();
    nest_map.insert(0, vec!["wrap_s_fn".to_string(), "s_fn".to_string()]);
    render_class_defs(
        &[],
        &[],
        &nest_map,
        &empty_targets(),
        &DARK_THEME,
        &mut lines,
    );
    assert!(lines.contains(&"  class wrap_s_fn nestL1;".to_string()));
    assert!(lines.contains(&"  class s_fn nestL1;".to_string()));
}

#[test]
fn emits_edge_target_subgraph_class_def_and_assignments_when_set_is_non_empty() {
    let mut lines: Vec<String> = Vec::new();
    let mut nest_map: HashMap<usize, Vec<String>> = HashMap::new();
    nest_map.insert(0, vec!["s_outer".to_string()]);
    let targets: HashSet<String> = HashSet::from(["s_outer".to_string()]);
    render_class_defs(&[], &[], &nest_map, &targets, &DARK_THEME, &mut lines);
    assert_eq!(
        lines.last().cloned(),
        Some("  class s_outer edgeTargetSubgraph;".to_string())
    );
    assert!(lines.iter().any(|v| v
        == &format!(
            "  classDef edgeTargetSubgraph stroke:{};",
            DARK_THEME.edge_target_subgraph.stroke
        )));
}

#[test]
fn edge_target_class_def_is_skipped_when_set_is_empty_even_if_nest_map_is_populated() {
    let mut lines: Vec<String> = Vec::new();
    let mut nest_map: HashMap<usize, Vec<String>> = HashMap::new();
    nest_map.insert(0, vec!["s_outer".to_string()]);
    render_class_defs(
        &[],
        &[],
        &nest_map,
        &empty_targets(),
        &DARK_THEME,
        &mut lines,
    );
    assert!(!lines.iter().any(|v| v.contains("edgeTargetSubgraph")));
}

#[test]
fn edge_target_assignments_follow_palette_slot_order_regardless_of_set_iteration_order() {
    let mut lines: Vec<String> = Vec::new();
    let mut nest_map: HashMap<usize, Vec<String>> = HashMap::new();
    nest_map.insert(0, vec!["s_outer".to_string()]);
    nest_map.insert(1, vec!["s_mid".to_string()]);
    nest_map.insert(2, vec!["s_inner".to_string()]);
    let targets: HashSet<String> = HashSet::from([
        "s_inner".to_string(),
        "s_outer".to_string(),
        "s_mid".to_string(),
    ]);
    render_class_defs(&[], &[], &nest_map, &targets, &DARK_THEME, &mut lines);
    let assignments: Vec<&String> = lines
        .iter()
        .filter(|v| v.contains("edgeTargetSubgraph;"))
        .collect();
    assert_eq!(
        assignments,
        vec![
            &"  class s_outer edgeTargetSubgraph;".to_string(),
            &"  class s_mid edgeTargetSubgraph;".to_string(),
            &"  class s_inner edgeTargetSubgraph;".to_string(),
        ]
    );
}

#[test]
fn edge_target_assignments_within_a_single_slot_follow_that_slots_emit_order() {
    // The doc promises "depth-then-emit-order": across slots is the
    // depth half, and WITHIN a slot the assignments follow the Vec's
    // emit order. Three ids share slot 0 in a fixed Vec order; only a
    // subset are edge targets, supplied in a scrambled set. The output
    // must follow the Vec order (wrap_s_fn before s_late), skipping the
    // non-target s_mid, regardless of how the set iterates.
    let mut lines: Vec<String> = Vec::new();
    let mut nest_map: HashMap<usize, Vec<String>> = HashMap::new();
    nest_map.insert(
        0,
        vec![
            "wrap_s_fn".to_string(),
            "s_mid".to_string(),
            "s_late".to_string(),
        ],
    );
    let targets: HashSet<String> = HashSet::from(["s_late".to_string(), "wrap_s_fn".to_string()]);
    render_class_defs(&[], &[], &nest_map, &targets, &DARK_THEME, &mut lines);
    let assignments: Vec<&String> = lines
        .iter()
        .filter(|v| v.contains("edgeTargetSubgraph;"))
        .collect();
    assert_eq!(
        assignments,
        vec![
            &"  class wrap_s_fn edgeTargetSubgraph;".to_string(),
            &"  class s_late edgeTargetSubgraph;".to_string(),
        ]
    );
}

#[test]
fn edge_target_class_lines_line_up_in_the_same_id_order_as_the_nest_class_lines() {
    // The doc's core claim: edge-target `class` lines "line up under
    // their matching `class … nestL<N>` line". With every id an edge
    // target, the id sequence of the `edgeTargetSubgraph` assignments
    // must equal the id sequence of the `nestL<N>` assignments across
    // both depth (slots) and emit order (within a slot).
    let mut lines: Vec<String> = Vec::new();
    let mut nest_map: HashMap<usize, Vec<String>> = HashMap::new();
    nest_map.insert(0, vec!["wrap_s_fn".to_string(), "s_fn".to_string()]);
    nest_map.insert(1, vec!["s_mid".to_string()]);
    nest_map.insert(2, vec!["s_inner_a".to_string(), "s_inner_b".to_string()]);
    let targets: HashSet<String> = HashSet::from([
        "wrap_s_fn".to_string(),
        "s_fn".to_string(),
        "s_mid".to_string(),
        "s_inner_a".to_string(),
        "s_inner_b".to_string(),
    ]);
    render_class_defs(&[], &[], &nest_map, &targets, &DARK_THEME, &mut lines);
    let ids_for = |suffix: &str| -> Vec<String> {
        lines
            .iter()
            .filter(|v| v.ends_with(&format!(" {suffix};")))
            .filter_map(|v| v.split_whitespace().nth(1).map(str::to_string))
            .collect::<Vec<String>>()
    };
    let nest_ids: Vec<String> = lines
        .iter()
        .filter(|v| v.contains(" nestL"))
        .filter(|v| v.contains("  class "))
        .filter_map(|v| v.split_whitespace().nth(1).map(str::to_string))
        .collect();
    assert_eq!(
        nest_ids,
        vec!["wrap_s_fn", "s_fn", "s_mid", "s_inner_a", "s_inner_b"]
    );
    assert_eq!(ids_for("edgeTargetSubgraph"), nest_ids);
}

#[test]
fn edge_target_assignments_skip_ids_outside_the_palette_slots() {
    // Only `s_mapped` has a nest_map entry; `s_unmapped` lives
    // deeper than the palette so it never received a `nestL<N>` slot
    // and consequently must not get an `edgeTargetSubgraph` class
    // assignment either (no per-depth fill to merge with).
    let mut lines: Vec<String> = Vec::new();
    let mut nest_map: HashMap<usize, Vec<String>> = HashMap::new();
    nest_map.insert(0, vec!["s_mapped".to_string()]);
    let targets: HashSet<String> =
        HashSet::from(["s_mapped".to_string(), "s_unmapped".to_string()]);
    render_class_defs(&[], &[], &nest_map, &targets, &DARK_THEME, &mut lines);
    let assignments: Vec<&String> = lines
        .iter()
        .filter(|v| v.contains("edgeTargetSubgraph;"))
        .collect();
    assert_eq!(
        assignments,
        vec![&"  class s_mapped edgeTargetSubgraph;".to_string()]
    );
}
