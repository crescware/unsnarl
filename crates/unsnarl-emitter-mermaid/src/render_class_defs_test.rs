use std::collections::HashMap;

use super::render_class_defs;
use crate::theme::{DARK_THEME, LIGHT_THEME};

fn empty_nest_map() -> HashMap<usize, Vec<String>> {
    HashMap::new()
}

#[test]
fn emits_nothing_when_all_id_lists_and_the_nest_map_are_empty() {
    let mut lines: Vec<String> = Vec::new();
    render_class_defs(&[], &[], &empty_nest_map(), &DARK_THEME, &mut lines);
    assert!(lines.is_empty());
}

#[test]
fn emits_the_boundary_stub_class_def_without_a_fill() {
    let mut lines: Vec<String> = Vec::new();
    let stub_ids = vec!["stub_1".to_string(), "stub_2".to_string()];
    render_class_defs(&stub_ids, &[], &empty_nest_map(), &DARK_THEME, &mut lines);
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
    render_class_defs(&[], &var_ids, &empty_nest_map(), &DARK_THEME, &mut lines);
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
    render_class_defs(&stub_ids, &[], &empty_nest_map(), &LIGHT_THEME, &mut lines);
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
    render_class_defs(&[], &[], &nest_map, &DARK_THEME, &mut lines);
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
    render_class_defs(&[], &[], &nest_map, &DARK_THEME, &mut lines);
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
    render_class_defs(&[], &[], &nest_map, &DARK_THEME, &mut lines);
    assert!(!lines.iter().any(|v| v.contains("nestL2")));
    assert!(lines.iter().any(|v| v.contains("nestL1")));
    assert!(lines.iter().any(|v| v.contains("nestL3")));
}

#[test]
fn places_a_function_wrapper_id_alongside_other_subgraphs_in_the_same_palette_slot() {
    let mut lines: Vec<String> = Vec::new();
    let mut nest_map: HashMap<usize, Vec<String>> = HashMap::new();
    nest_map.insert(0, vec!["wrap_s_fn".to_string(), "s_fn".to_string()]);
    render_class_defs(&[], &[], &nest_map, &DARK_THEME, &mut lines);
    assert!(lines.contains(&"  class wrap_s_fn nestL1;".to_string()));
    assert!(lines.contains(&"  class s_fn nestL1;".to_string()));
}
