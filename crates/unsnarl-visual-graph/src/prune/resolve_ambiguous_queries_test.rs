use super::*;

use unsnarl_ir::SourceLine;

use crate::prune::test_helpers::{const_binding_node_with_end, graph_of, write_op_node};
use crate::visual_element::VisualElement;
use crate::visual_node::VisualNode;

fn variable_node(name: &str, line: u32) -> VisualNode {
    const_binding_node_with_end(&format!("n-{name}-{line}"), name, line, None)
}

fn writeop_named(name: &str, line: u32) -> VisualNode {
    write_op_node(&format!("w-{name}-{line}"), name, line)
}

fn line_or_name(raw: &str, line: u32) -> ParsedRootQuery {
    ParsedRootQuery::LineOrName {
        line: SourceLine(line),
        name: raw.to_string(),
        raw: raw.to_string(),
    }
}

fn line_query(line: u32, raw: &str) -> ParsedRootQuery {
    ParsedRootQuery::Line {
        line: SourceLine(line),
        raw: raw.to_string(),
    }
}

fn name_query(name: &str) -> ParsedRootQuery {
    ParsedRootQuery::Name {
        name: name.to_string(),
        raw: name.to_string(),
    }
}

#[test]
fn returns_input_untouched_when_no_line_or_name_is_present() {
    let g = graph_of(vec![VisualElement::Node(variable_node("foo", 1))], vec![]);
    let queries = vec![line_query(5, "5"), name_query("foo")];
    let result = resolve_ambiguous_queries(&g, &queries);
    assert_eq!(result.resolved, queries);
    assert!(result.resolutions.is_empty());
}

#[test]
fn silently_treats_line_or_name_as_line_when_no_l_prefix_identifier_exists() {
    let g = graph_of(
        vec![
            VisualElement::Node(variable_node("foo", 1)),
            VisualElement::Node(variable_node("bar", 3)),
        ],
        vec![],
    );
    let result = resolve_ambiguous_queries(&g, &[line_or_name("L12", 12)]);
    assert_eq!(result.resolved, vec![line_query(12, "L12")]);
    assert!(result.resolutions.is_empty());
}

#[test]
fn resolves_to_name_when_an_exact_match_exists_with_a_resolution_log() {
    let g = graph_of(
        vec![
            VisualElement::Node(variable_node("L12", 7)),
            VisualElement::Node(variable_node("other", 9)),
        ],
        vec![],
    );
    let result = resolve_ambiguous_queries(&g, &[line_or_name("L12", 12)]);
    assert_eq!(result.resolved, vec![name_query("L12")]);
    assert_eq!(
        result.resolutions,
        vec![RootQueryResolution {
            raw: "L12".to_string(),
            line: SourceLine(12),
            name: "L12".to_string(),
            resolved_as: ResolvedAs::Name,
        }]
    );
}

#[test]
fn resolves_to_line_when_other_l_n_identifiers_exist_but_no_exact_match() {
    let g = graph_of(
        vec![
            VisualElement::Node(variable_node("l5", 1)),
            VisualElement::Node(variable_node("l99", 3)),
        ],
        vec![],
    );
    let result = resolve_ambiguous_queries(&g, &[line_or_name("L12", 12)]);
    assert_eq!(result.resolved, vec![line_query(12, "L12")]);
    assert_eq!(
        result.resolutions,
        vec![RootQueryResolution {
            raw: "L12".to_string(),
            line: SourceLine(12),
            name: "L12".to_string(),
            resolved_as: ResolvedAs::Line,
        }]
    );
}

#[test]
fn name_lookup_is_case_sensitive() {
    let g = graph_of(vec![VisualElement::Node(variable_node("l1", 1))], vec![]);
    let result = resolve_ambiguous_queries(&g, &[line_or_name("L1", 1)]);
    assert_eq!(result.resolved, vec![line_query(1, "L1")]);
    assert_eq!(
        result.resolutions,
        vec![RootQueryResolution {
            raw: "L1".to_string(),
            line: SourceLine(1),
            name: "L1".to_string(),
            resolved_as: ResolvedAs::Line,
        }]
    );
}

#[test]
fn preserves_order_across_a_mixed_array() {
    let g = graph_of(
        vec![
            VisualElement::Node(variable_node("L12", 4)),
            VisualElement::Node(variable_node("l5", 6)),
        ],
        vec![],
    );
    let queries = vec![
        line_query(1, "1"),
        line_or_name("L12", 12),
        name_query("x"),
        line_or_name("L99", 99),
    ];
    let result = resolve_ambiguous_queries(&g, &queries);
    assert_eq!(
        result.resolved,
        vec![
            line_query(1, "1"),
            name_query("L12"),
            name_query("x"),
            line_query(99, "L99"),
        ]
    );
    assert_eq!(
        result.resolutions,
        vec![
            RootQueryResolution {
                raw: "L12".to_string(),
                line: SourceLine(12),
                name: "L12".to_string(),
                resolved_as: ResolvedAs::Name,
            },
            RootQueryResolution {
                raw: "L99".to_string(),
                line: SourceLine(99),
                name: "L99".to_string(),
                resolved_as: ResolvedAs::Line,
            },
        ]
    );
}

#[test]
fn emits_one_resolution_entry_per_line_or_name_occurrence() {
    let g = graph_of(vec![VisualElement::Node(variable_node("L12", 1))], vec![]);
    let result = resolve_ambiguous_queries(&g, &[line_or_name("L12", 12), line_or_name("L12", 12)]);
    assert_eq!(
        result.resolutions,
        vec![
            RootQueryResolution {
                raw: "L12".to_string(),
                line: SourceLine(12),
                name: "L12".to_string(),
                resolved_as: ResolvedAs::Name,
            },
            RootQueryResolution {
                raw: "L12".to_string(),
                line: SourceLine(12),
                name: "L12".to_string(),
                resolved_as: ResolvedAs::Name,
            },
        ]
    );
}

#[test]
fn ignores_name_query_excluded_kinds_when_collecting_matchable_names() {
    let g = graph_of(
        vec![
            VisualElement::Node(writeop_named("L12", 1)),
            VisualElement::Node(variable_node("foo", 3)),
        ],
        vec![],
    );
    let result = resolve_ambiguous_queries(&g, &[line_or_name("L12", 12)]);
    assert_eq!(result.resolved, vec![line_query(12, "L12")]);
    assert!(result.resolutions.is_empty());
}
