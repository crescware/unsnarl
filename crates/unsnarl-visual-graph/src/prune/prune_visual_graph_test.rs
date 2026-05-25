use super::*;

use std::collections::HashSet;

use unsnarl_ir::SourceLine;

use crate::prune::collect_ids::collect_ids;
use crate::prune::test_helpers::{
    const_binding_node_with_end, function_subgraph, graph_of, if_else_container_subgraph,
    return_subgraph, return_use_node, write_op_node,
};
use crate::visual_boundary_edge::VisualBoundaryEdge;
use crate::visual_edge::VisualEdge;
use crate::visual_element::VisualElement;
use crate::visual_node::VisualNode;

fn node_at(id: &str, name: &str, line: u32) -> VisualNode {
    const_binding_node_with_end(id, name, line, None)
}

fn node_with_end(id: &str, name: &str, line: u32, end: u32) -> VisualNode {
    const_binding_node_with_end(id, name, line, Some(end))
}

fn edge(from: &str, to: &str, label: &str) -> VisualEdge {
    VisualEdge::new(from, to, label)
}

fn raw_line(n: u32) -> ParsedRootQuery {
    ParsedRootQuery::Line {
        line: SourceLine(n),
        raw: n.to_string(),
    }
}

fn raw_line_name(n: u32, name: &str) -> ParsedRootQuery {
    ParsedRootQuery::LineName {
        line: SourceLine(n),
        name: name.to_string(),
        raw: format!("{n}:{name}"),
    }
}

fn raw_name(name: &str) -> ParsedRootQuery {
    ParsedRootQuery::Name {
        name: name.to_string(),
        raw: name.to_string(),
    }
}

fn raw_range(s: u32, e: u32) -> ParsedRootQuery {
    ParsedRootQuery::Range {
        start: SourceLine(s),
        end: SourceLine(e),
        raw: format!("{s}-{e}"),
    }
}

fn ids_in_order(graph: &VisualGraph) -> Vec<String> {
    graph.elements.iter().map(|v| v.id().to_string()).collect()
}

fn flatten_ids(graph: &VisualGraph) -> HashSet<String> {
    collect_ids(&graph.elements)
}

#[test]
fn returns_the_graph_unchanged_when_no_roots_are_provided() {
    let g = graph_of(
        vec![
            VisualElement::Node(node_at("a", "a", 1)),
            VisualElement::Node(node_at("b", "b", 2)),
        ],
        vec![edge("a", "b", "read")],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![],
            descendants: 5,
            ancestors: 5,
        },
    );
    assert_eq!(ids_in_order(&r.graph), vec!["a", "b"]);
    assert!(r.per_query.is_empty());
}

#[test]
fn matches_by_line_and_keeps_only_the_root_when_n_is_zero() {
    let g = graph_of(
        vec![
            VisualElement::Node(node_at("a", "a", 1)),
            VisualElement::Node(node_at("b", "b", 2)),
            VisualElement::Node(node_at("c", "c", 3)),
        ],
        vec![edge("a", "b", "read"), edge("b", "c", "read")],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line(2)],
            descendants: 0,
            ancestors: 0,
        },
    );
    assert_eq!(ids_in_order(&r.graph), vec!["b"]);
    assert!(r.graph.edges.is_empty());
    assert_eq!(r.per_query[0].matched, 1);
}

#[test]
fn expands_descendants_by_n_hops_with_unlabeled_outbound_boundary_hint() {
    let g = graph_of(
        vec![
            VisualElement::Node(node_at("a", "a", 1)),
            VisualElement::Node(node_at("b", "b", 2)),
            VisualElement::Node(node_at("c", "c", 3)),
            VisualElement::Node(node_at("d", "d", 4)),
        ],
        vec![
            edge("a", "b", "read"),
            edge("b", "c", "read"),
            edge("c", "d", "read"),
        ],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line(1)],
            descendants: 2,
            ancestors: 0,
        },
    );
    let mut got = ids_in_order(&r.graph);
    got.sort();
    assert_eq!(got, vec!["a", "b", "c"]);
    assert_eq!(r.graph.boundary_edges.len(), 1);
    match &r.graph.boundary_edges[0] {
        VisualBoundaryEdge::Out { inside, .. } => assert_eq!(inside, "c"),
        _ => panic!("expected out direction"),
    }
}

#[test]
fn expands_ancestors_by_n_hops_with_labeled_inbound_boundary_hint() {
    let g = graph_of(
        vec![
            VisualElement::Node(node_at("a", "a", 1)),
            VisualElement::Node(node_at("b", "b", 2)),
            VisualElement::Node(node_at("c", "c", 3)),
            VisualElement::Node(node_at("d", "d", 4)),
        ],
        vec![
            edge("a", "b", "read"),
            edge("b", "c", "read"),
            edge("c", "d", "read"),
        ],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line(4)],
            descendants: 0,
            ancestors: 2,
        },
    );
    let mut got = ids_in_order(&r.graph);
    got.sort();
    assert_eq!(got, vec!["b", "c", "d"]);
    assert_eq!(r.graph.boundary_edges.len(), 1);
    match &r.graph.boundary_edges[0] {
        VisualBoundaryEdge::In { inside, label, .. } => {
            assert_eq!(inside, "b");
            assert_eq!(label, "read");
        }
        _ => panic!("expected in direction"),
    }
}

#[test]
fn collapses_repeated_boundary_entries_on_same_inside_and_direction_out() {
    let g = graph_of(
        vec![
            VisualElement::Node(node_at("c", "c", 1)),
            VisualElement::Node(node_at("d", "d", 2)),
        ],
        vec![
            edge("c", "d", "read"),
            edge("d", "e", "read"),
            edge("d", "f", "read"),
        ],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line_name(1, "c")],
            descendants: 1,
            ancestors: 0,
        },
    );
    assert_eq!(r.graph.boundary_edges.len(), 1);
}

#[test]
fn merges_in_direction_labels_into_sorted_deduplicated_comma_list() {
    let g = graph_of(
        vec![
            VisualElement::Node(node_at("A", "a", 5)),
            VisualElement::Node(node_at("M", "m", 4)),
        ],
        vec![
            edge("M", "A", "read"),
            edge("X", "M", "read,call"),
            edge("Y", "M", "read"),
            edge("Z", "M", "set"),
        ],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line_name(5, "a")],
            descendants: 0,
            ancestors: 1,
        },
    );
    assert_eq!(r.graph.boundary_edges.len(), 1);
    match &r.graph.boundary_edges[0] {
        VisualBoundaryEdge::In { inside, label, .. } => {
            assert_eq!(inside, "M");
            assert_eq!(label, "call,read,set");
        }
        _ => panic!("expected in direction"),
    }
}

#[test]
fn descendants_zero_stays_strict_no_boundary_peek_emitted() {
    let g = graph_of(
        vec![
            VisualElement::Node(node_at("a", "a", 1)),
            VisualElement::Node(node_at("b", "b", 2)),
        ],
        vec![edge("a", "b", "read")],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line(1)],
            descendants: 0,
            ancestors: 0,
        },
    );
    assert_eq!(ids_in_order(&r.graph), vec!["a"]);
    assert!(r.graph.boundary_edges.is_empty());
}

#[test]
fn retains_the_parent_subgraph_wrapping_a_kept_node() {
    let inner = node_at("inner", "x", 5);
    let sg = function_subgraph("sg1", 4, vec![VisualElement::Node(inner)]);
    let outer = node_at("outer", "y", 1);
    let g = graph_of(
        vec![VisualElement::Node(outer), VisualElement::Subgraph(sg)],
        vec![],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line_name(5, "x")],
            descendants: 0,
            ancestors: 0,
        },
    );
    assert_eq!(ids_in_order(&r.graph), vec!["sg1"]);
    let VisualElement::Subgraph(s) = &r.graph.elements[0] else {
        panic!("expected subgraph");
    };
    assert_eq!(s.elements().len(), 1);
    assert_eq!(s.elements()[0].id(), "inner");
}

#[test]
fn drops_empty_subgraphs() {
    let lonely = function_subgraph("sg2", 10, vec![VisualElement::Node(node_at("z", "z", 11))]);
    let g = graph_of(
        vec![
            VisualElement::Node(node_at("a", "a", 1)),
            VisualElement::Subgraph(lonely),
        ],
        vec![],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line(1)],
            descendants: 0,
            ancestors: 0,
        },
    );
    assert_eq!(ids_in_order(&r.graph), vec!["a"]);
}

#[test]
fn subgraph_ids_serve_as_bfs_endpoints_but_kept_only_when_inside_survives() {
    let g = graph_of(
        vec![
            VisualElement::Node(node_at("flag", "flag", 1)),
            VisualElement::Subgraph(if_else_container_subgraph(
                "cont_if",
                3,
                vec![VisualElement::Node(node_at("wr1", "set", 4))],
            )),
            VisualElement::Node(node_at("result", "result", 10)),
        ],
        vec![
            edge("flag", "cont_if", "read"),
            edge("cont_if", "wr1", "branch"),
            edge("wr1", "result", "read"),
        ],
    );

    let tight = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line_name(1, "flag")],
            descendants: 1,
            ancestors: 0,
        },
    );
    assert_eq!(ids_in_order(&tight.graph), vec!["flag"]);

    let wider = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line_name(1, "flag")],
            descendants: 2,
            ancestors: 0,
        },
    );
    let wider_ids = flatten_ids(&wider.graph);
    assert!(wider_ids.contains("flag"));
    assert!(wider_ids.contains("cont_if"));
    assert!(wider_ids.contains("wr1"));
}

#[test]
fn counts_per_query_matches_and_reports_zero_when_nothing_matches() {
    let g = graph_of(
        vec![
            VisualElement::Node(node_at("a", "foo", 1)),
            VisualElement::Node(node_at("b", "bar", 2)),
        ],
        vec![],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_name("foo"), raw_name("nope")],
            descendants: 0,
            ancestors: 0,
        },
    );
    assert_eq!(r.per_query[0].matched, 1);
    assert_eq!(r.per_query[1].matched, 0);
    let pruning = r.graph.pruning.as_ref().expect("pruning attached");
    assert_eq!(pruning.roots.len(), 2);
    assert_eq!(pruning.roots[0].query, "foo");
    assert_eq!(pruning.roots[0].matched, 1);
    assert_eq!(pruning.roots[1].query, "nope");
    assert_eq!(pruning.roots[1].matched, 0);
}

#[test]
fn emits_an_empty_graph_when_every_query_misses() {
    let g = graph_of(
        vec![
            VisualElement::Node(node_at("a", "foo", 1)),
            VisualElement::Node(node_at("b", "bar", 2)),
        ],
        vec![edge("a", "b", "read")],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_name("nope")],
            descendants: 5,
            ancestors: 5,
        },
    );
    assert!(r.graph.elements.is_empty());
    assert!(r.graph.edges.is_empty());
    let pruning = r.graph.pruning.as_ref().expect("pruning attached");
    assert_eq!(pruning.roots.len(), 1);
    assert_eq!(pruning.roots[0].query, "nope");
    assert_eq!(pruning.roots[0].matched, 0);
}

#[test]
fn name_only_query_matches_across_scopes_multiple_hits() {
    let g = graph_of(
        vec![
            VisualElement::Node(node_at("outer_counter", "counter", 1)),
            VisualElement::Subgraph(function_subgraph(
                "fn",
                5,
                vec![VisualElement::Node(node_at("inner_counter", "counter", 6))],
            )),
        ],
        vec![],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_name("counter")],
            descendants: 0,
            ancestors: 0,
        },
    );
    let ids = flatten_ids(&r.graph);
    assert!(ids.contains("outer_counter"));
    assert!(ids.contains("inner_counter"));
    assert!(ids.contains("fn"));
    assert_eq!(r.per_query[0].matched, 2);
}

#[test]
fn range_query_covers_all_lines_in_the_inclusive_range() {
    let g = graph_of(
        vec![
            VisualElement::Node(node_at("a", "a", 9)),
            VisualElement::Node(node_at("b", "b", 11)),
            VisualElement::Node(node_at("c", "c", 13)),
            VisualElement::Node(node_at("d", "d", 14)),
        ],
        vec![],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_range(9, 13)],
            descendants: 0,
            ancestors: 0,
        },
    );
    assert_eq!(ids_in_order(&r.graph), vec!["a", "b", "c"]);
}

#[test]
fn line_query_matches_return_use_at_that_line_directly() {
    let decl_a = node_at("n_scope_0_a_6", "a", 1);
    let use_a = return_use_node("ret_use_ref_0", "a", 11, None);
    let ret = return_subgraph("sg_return", 10, vec![VisualElement::Node(use_a)]);
    let g = graph_of(
        vec![VisualElement::Node(decl_a), VisualElement::Subgraph(ret)],
        vec![edge("n_scope_0_a_6", "ret_use_ref_0", "read")],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line(11)],
            descendants: 0,
            ancestors: 0,
        },
    );
    let mut got: Vec<String> = flatten_ids(&r.graph).into_iter().collect();
    got.sort();
    assert_eq!(got, vec!["ret_use_ref_0", "sg_return"]);
    assert_eq!(r.per_query[0].matched, 1);
}

#[test]
fn ancestors_one_reaches_the_declaration_from_a_return_use_root() {
    let decl_a = node_at("n_scope_0_a_6", "a", 1);
    let use_a = return_use_node("ret_use_ref_0", "a", 11, None);
    let ret = return_subgraph("sg_return", 10, vec![VisualElement::Node(use_a)]);
    let g = graph_of(
        vec![VisualElement::Node(decl_a), VisualElement::Subgraph(ret)],
        vec![edge("n_scope_0_a_6", "ret_use_ref_0", "read")],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line(11)],
            descendants: 0,
            ancestors: 1,
        },
    );
    let ids = flatten_ids(&r.graph);
    assert!(ids.contains("n_scope_0_a_6"));
    assert!(ids.contains("ret_use_ref_0"));
}

#[test]
fn jsx_return_use_spanning_multiple_lines_matched_anywhere_in_span() {
    let use_a = return_use_node("ret_use_ref_0", "a", 11, Some(23));
    let ret = return_subgraph("sg_return", 10, vec![VisualElement::Node(use_a)]);
    let g = graph_of(vec![VisualElement::Subgraph(ret)], vec![]);
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line(23)],
            descendants: 0,
            ancestors: 0,
        },
    );
    let ids = flatten_ids(&r.graph);
    assert!(ids.contains("ret_use_ref_0"));
    assert_eq!(r.per_query[0].matched, 1);
}

#[test]
fn write_op_is_also_a_root_candidate() {
    let wr = write_op_node("wr_ref_0", "x", 5);
    let g = graph_of(vec![VisualElement::Node(wr)], vec![]);
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line(5)],
            descendants: 0,
            ancestors: 0,
        },
    );
    assert_eq!(ids_in_order(&r.graph), vec!["wr_ref_0"]);
    assert_eq!(r.per_query[0].matched, 1);
}

#[test]
fn name_queries_skip_write_op_and_return_use() {
    let decl = node_at("n_decl_foo", "foo", 1);
    let wr = write_op_node("wr_foo", "foo", 5);
    let ret = return_use_node("ret_foo", "foo", 11, None);
    let g = graph_of(
        vec![
            VisualElement::Node(decl),
            VisualElement::Node(wr),
            VisualElement::Node(ret),
        ],
        vec![],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_name("foo")],
            descendants: 0,
            ancestors: 0,
        },
    );
    assert_eq!(ids_in_order(&r.graph), vec!["n_decl_foo"]);
    assert_eq!(r.per_query[0].matched, 1);
}

#[test]
fn line_name_still_matches_write_op_and_return_use_at_requested_line() {
    let wr = write_op_node("wr_foo", "foo", 5);
    let ret = return_use_node("ret_foo", "foo", 11, None);
    let g = graph_of(
        vec![VisualElement::Node(wr), VisualElement::Node(ret)],
        vec![],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line_name(11, "foo")],
            descendants: 0,
            ancestors: 0,
        },
    );
    assert_eq!(ids_in_order(&r.graph), vec!["ret_foo"]);
    assert_eq!(r.per_query[0].matched, 1);
}

#[test]
fn bare_line_query_matching_subgraph_start_sweeps_every_node_inside() {
    let inner = node_at("inner_a", "a", 11);
    let outer_only = node_at("outside", "z", 50);
    let sg = return_subgraph("sg_return", 10, vec![VisualElement::Node(inner)]);
    let g = graph_of(
        vec![VisualElement::Subgraph(sg), VisualElement::Node(outer_only)],
        vec![],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line(10)],
            descendants: 0,
            ancestors: 0,
        },
    );
    let flat = flatten_ids(&r.graph);
    assert!(flat.contains("inner_a"));
    assert!(!flat.contains("outside"));
    assert_eq!(r.per_query[0].matched, 1);
}

#[test]
fn range_query_never_auto_pulls_subgraph_body() {
    let inner = node_at("inner_a", "a", 11);
    let sg = return_subgraph("sg_return", 10, vec![VisualElement::Node(inner)]);
    let g = graph_of(vec![VisualElement::Subgraph(sg)], vec![]);
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_range(10, 11)],
            descendants: 0,
            ancestors: 0,
        },
    );
    let flat = flatten_ids(&r.graph);
    assert!(flat.contains("inner_a"));
    assert_eq!(r.per_query[0].matched, 1);
}

#[test]
fn line_query_not_at_subgraph_start_falls_back_to_per_node_matching() {
    let inner = node_at("inner_a", "a", 11);
    let sg = return_subgraph("sg_return", 10, vec![VisualElement::Node(inner)]);
    let g = graph_of(vec![VisualElement::Subgraph(sg)], vec![]);
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line(11)],
            descendants: 0,
            ancestors: 0,
        },
    );
    let flat = flatten_ids(&r.graph);
    assert!(flat.contains("inner_a"));
    assert!(flat.contains("sg_return"));
    assert_eq!(r.per_query[0].matched, 1);
}

#[test]
fn node_end_line_window_matches_query_within() {
    let ranged = node_with_end("a", "a", 11, 23);
    let g = graph_of(vec![VisualElement::Node(ranged)], vec![]);
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line(20)],
            descendants: 0,
            ancestors: 0,
        },
    );
    assert_eq!(ids_in_order(&r.graph), vec!["a"]);
}

#[test]
fn line_just_past_end_line_does_not_match() {
    let ranged = node_with_end("a", "a", 11, 23);
    let g = graph_of(vec![VisualElement::Node(ranged)], vec![]);
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line(24)],
            descendants: 0,
            ancestors: 0,
        },
    );
    assert!(r.graph.elements.is_empty());
}

#[test]
fn range_query_overlapping_node_window_matches_once() {
    let ranged = node_with_end("a", "a", 11, 23);
    let g = graph_of(vec![VisualElement::Node(ranged)], vec![]);
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_range(20, 30)],
            descendants: 0,
            ancestors: 0,
        },
    );
    assert_eq!(ids_in_order(&r.graph), vec!["a"]);
    assert_eq!(r.per_query[0].matched, 1);
}

#[test]
fn both_direction_context_emits_both_side_boundary_hints() {
    let g = graph_of(
        vec![
            VisualElement::Node(node_at("a", "a", 1)),
            VisualElement::Node(node_at("b", "b", 2)),
            VisualElement::Node(node_at("c", "c", 3)),
            VisualElement::Node(node_at("d", "d", 4)),
            VisualElement::Node(node_at("e", "e", 5)),
        ],
        vec![
            edge("a", "b", "read"),
            edge("b", "c", "read"),
            edge("c", "d", "read"),
            edge("d", "e", "read"),
        ],
    );
    let r = prune_visual_graph(
        &g,
        &PruneOptions {
            roots: vec![raw_line(3)],
            descendants: 1,
            ancestors: 1,
        },
    );
    let mut got = ids_in_order(&r.graph);
    got.sort();
    assert_eq!(got, vec!["b", "c", "d"]);
    assert_eq!(r.graph.boundary_edges.len(), 2);
    let mut out_found = false;
    let mut in_found = false;
    for be in &r.graph.boundary_edges {
        match be {
            VisualBoundaryEdge::Out { inside, .. } if inside == "d" => out_found = true,
            VisualBoundaryEdge::In { inside, label, .. } if inside == "b" && label == "read" => {
                in_found = true
            }
            _ => {}
        }
    }
    assert!(out_found);
    assert!(in_found);
}
