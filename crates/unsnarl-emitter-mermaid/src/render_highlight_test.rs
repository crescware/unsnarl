use super::render_highlight;
use crate::theme::DARK_THEME;

#[test]
fn writes_nothing_when_no_ids_are_highlighted() {
    let mut lines: Vec<String> = Vec::new();
    render_highlight(&[], &[], &DARK_THEME, &mut lines);
    assert!(lines.is_empty());
}

#[test]
fn emits_one_style_line_per_highlighted_node() {
    let ids = vec!["n_a".to_string()];
    let mut lines: Vec<String> = Vec::new();
    render_highlight(&ids, &[], &DARK_THEME, &mut lines);
    let h = &DARK_THEME.highlight;
    assert_eq!(
        lines,
        vec![format!(
            "  style n_a fill:{},stroke:{},color:{};",
            h.fill, h.stroke, h.color
        )]
    );
}

#[test]
fn emits_a_single_link_style_line_covering_every_highlighted_edge_index() {
    let ids = vec!["n_a".to_string()];
    let mut lines: Vec<String> = Vec::new();
    render_highlight(&ids, &[0, 2, 3], &DARK_THEME, &mut lines);
    let h = &DARK_THEME.highlight;
    let last = lines.last().expect("last line present");
    assert_eq!(
        last,
        &format!(
            "  linkStyle 0,2,3 stroke:{},stroke-width:{};",
            h.edge_stroke, h.edge_stroke_width
        )
    );
}

#[test]
fn skips_the_link_style_line_when_no_edge_indices_are_supplied() {
    let ids = vec!["n_a".to_string()];
    let mut lines: Vec<String> = Vec::new();
    render_highlight(&ids, &[], &DARK_THEME, &mut lines);
    assert!(!lines.iter().any(|v| v.starts_with("  linkStyle")));
}
