use super::collect_highlight_path_ids;

use unsnarl_ir::language::Language;
use unsnarl_root_query::{Direction as QueryDir, ParsedRootQuery, RootQuery};

use crate::direction::Direction;
use crate::visual_edge::VisualEdge;
use crate::visual_element::VisualElement;
use crate::visual_graph::VisualGraph;
use crate::visual_node::{BindingVisualNode, VisualNode};

fn node(id: &str, name: &str, line: u32) -> VisualNode {
    BindingVisualNode::const_binding(id, name, line).into()
}

/// a -> b -> c, with an off-path branch b -> d, plus an isolated node.
fn chain_graph() -> VisualGraph {
    let nodes = vec![
        node("n_a", "a", 1),
        node("n_b", "b", 2),
        node("n_c", "c", 3),
        node("n_d", "d", 4),
        node("n_iso", "iso", 5),
    ];
    let edges = vec![
        VisualEdge::new("n_a", "n_b", "read"),
        VisualEdge::new("n_b", "n_c", "read"),
        VisualEdge::new("n_b", "n_d", "read"),
    ];
    VisualGraph::new(
        "x.ts",
        Language::Ts,
        Direction::RL,
        nodes.into_iter().map(VisualElement::Node).collect(),
        edges,
        Vec::new(),
    )
}

fn name(s: &str) -> ParsedRootQuery {
    ParsedRootQuery::Name {
        name: s.to_string(),
        raw: s.to_string(),
    }
}

fn direction(seed: &str, dir: QueryDir) -> RootQuery {
    RootQuery::Direction {
        lhs: name(seed),
        dir,
        level: None,
        raw: format!("{seed}..dir"),
    }
}

fn path(lhs: &str, rhs: &str) -> RootQuery {
    RootQuery::Path {
        lhs: name(lhs),
        rhs: name(rhs),
        raw: format!("{lhs}..{rhs}"),
    }
}

#[test]
fn empty_queries_return_empty() {
    assert!(collect_highlight_path_ids(&chain_graph(), &[]).is_empty());
}

#[test]
fn single_query_paints_only_the_matched_point() {
    let ids = collect_highlight_path_ids(
        &chain_graph(),
        &[RootQuery::Single {
            query: name("b"),
            raw: "b".to_string(),
        }],
    );
    assert_eq!(ids, vec!["n_b"]);
}

#[test]
fn direction_after_paints_the_forward_reachable_set_including_the_seed() {
    let ids = collect_highlight_path_ids(&chain_graph(), &[direction("a", QueryDir::After)]);
    // Walk order over the element tree.
    assert_eq!(ids, vec!["n_a", "n_b", "n_c", "n_d"]);
}

#[test]
fn direction_before_paints_the_backward_reachable_set() {
    let ids = collect_highlight_path_ids(&chain_graph(), &[direction("c", QueryDir::Before)]);
    assert_eq!(ids, vec!["n_a", "n_b", "n_c"]);
}

#[test]
fn direction_context_paints_both_directions() {
    let ids = collect_highlight_path_ids(&chain_graph(), &[direction("b", QueryDir::Context)]);
    assert_eq!(ids, vec!["n_a", "n_b", "n_c", "n_d"]);
}

#[test]
fn path_paints_the_nodes_between_the_two_endpoints_excluding_off_path_branches() {
    let ids = collect_highlight_path_ids(&chain_graph(), &[path("a", "c")]);
    // n_d hangs off n_b but is not on any a<->c route.
    assert_eq!(ids, vec!["n_a", "n_b", "n_c"]);
}

#[test]
fn path_is_direction_independent() {
    let forward = collect_highlight_path_ids(&chain_graph(), &[path("a", "c")]);
    let backward = collect_highlight_path_ids(&chain_graph(), &[path("c", "a")]);
    assert_eq!(forward, backward);
}

#[test]
fn path_with_no_connecting_route_paints_nothing() {
    let ids = collect_highlight_path_ids(&chain_graph(), &[path("a", "iso")]);
    assert!(ids.is_empty());
}
