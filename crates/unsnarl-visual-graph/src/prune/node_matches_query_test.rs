use super::*;

use crate::prune::prune_fixtures::{const_binding_node_with_end, return_use_node, write_op_node};

fn const_binding(line: u32, name: &str) -> VisualNode {
    const_binding_node_with_end("n1", name, line, None)
}

fn const_binding_ranged(line: u32, end_line: u32, name: &str) -> VisualNode {
    const_binding_node_with_end("n1", name, line, Some(end_line))
}

#[test]
fn line_matches_when_line_falls_within_range() {
    let q = ParsedRootQuery::Line {
        line: SourceLine(5),
        raw: "5".to_string(),
    };
    assert!(node_matches_query(&const_binding(5, "x"), &q));
    assert!(!node_matches_query(&const_binding(4, "x"), &q));
    let q6 = ParsedRootQuery::Line {
        line: SourceLine(6),
        raw: "6".to_string(),
    };
    assert!(node_matches_query(&const_binding_ranged(5, 7, "x"), &q6));
    let q8 = ParsedRootQuery::Line {
        line: SourceLine(8),
        raw: "8".to_string(),
    };
    assert!(!node_matches_query(&const_binding_ranged(5, 7, "x"), &q8));
}

#[test]
fn line_name_additionally_requires_exact_name_match() {
    let q = ParsedRootQuery::LineName {
        line: SourceLine(5),
        name: "x".to_string(),
        raw: "5:x".to_string(),
    };
    assert!(node_matches_query(&const_binding(5, "x"), &q));
    assert!(!node_matches_query(&const_binding(5, "y"), &q));
}

#[test]
fn range_overlaps_node_line_range() {
    let q = ParsedRootQuery::Range {
        start: SourceLine(4),
        end: SourceLine(6),
        raw: "4-6".to_string(),
    };
    assert!(node_matches_query(&const_binding(5, "x"), &q));
    assert!(!node_matches_query(&const_binding(7, "x"), &q));
    assert!(node_matches_query(&const_binding_ranged(1, 4, "x"), &q));
}

#[test]
fn range_name_additionally_requires_exact_name_match() {
    let q = ParsedRootQuery::RangeName {
        start: SourceLine(4),
        end: SourceLine(6),
        name: "x".to_string(),
        raw: "4-6:x".to_string(),
    };
    assert!(node_matches_query(&const_binding(5, "x"), &q));
    assert!(!node_matches_query(&const_binding(5, "y"), &q));
}

#[test]
fn name_matches_except_for_excluded_use_site_kinds() {
    let q = ParsedRootQuery::Name {
        name: "x".to_string(),
        raw: "x".to_string(),
    };
    assert!(node_matches_query(&const_binding(5, "x"), &q));
    assert!(!node_matches_query(&write_op_node("n1", "x", 5), &q));
    assert!(!node_matches_query(
        &return_use_node("n1", "x", 5, None),
        &q
    ));
}
