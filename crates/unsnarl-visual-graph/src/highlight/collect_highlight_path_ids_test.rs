use super::{collect_highlight_path_ids, HighlightWarning};

use unsnarl_ir::language::Language;
use unsnarl_root_query::{Direction as QueryDir, ParsedRootQuery, RootQuery};

use crate::direction::Direction;
use crate::prune::{ResolvedAs, RootQueryResolution};
use crate::visual_edge::VisualEdge;
use crate::visual_element::VisualElement;
use crate::visual_graph::VisualGraph;
use crate::visual_node::{BindingVisualNode, SyntheticVisualNode, VisualNode};

fn node(id: &str, name: &str, line: u32) -> VisualNode {
    BindingVisualNode::const_binding(id, name, line).into()
}

fn graph_of(nodes: Vec<VisualNode>, edges: Vec<VisualEdge>) -> VisualGraph {
    VisualGraph::new(
        "x.ts",
        Language::Ts,
        Direction::RL,
        nodes.into_iter().map(VisualElement::Node).collect(),
        edges,
        Vec::new(),
    )
}

/// a -> b -> c, with an off-path branch b -> d, plus an isolated node.
fn chain_graph() -> VisualGraph {
    graph_of(
        vec![
            node("n_a", "a", 1),
            node("n_b", "b", 2),
            node("n_c", "c", 3),
            node("n_d", "d", 4),
            node("n_iso", "iso", 5),
        ],
        vec![
            VisualEdge::new("n_a", "n_b", "read"),
            VisualEdge::new("n_b", "n_c", "read"),
            VisualEdge::new("n_b", "n_d", "read"),
        ],
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
    path_q(name(lhs), name(rhs), &format!("{lhs}..{rhs}"))
}

fn path_q(lhs: ParsedRootQuery, rhs: ParsedRootQuery, raw: &str) -> RootQuery {
    RootQuery::Path {
        lhs,
        rhs,
        raw: raw.to_string(),
    }
}

#[test]
fn empty_queries_return_empty() {
    let sel = collect_highlight_path_ids(&chain_graph(), &[]);
    assert!(sel.ids.is_empty());
    assert!(sel.point_ids.is_empty());
    assert!(sel.resolutions.is_empty());
    assert!(sel.warnings.is_empty());
}

#[test]
fn single_query_paints_only_the_matched_point_and_records_it_as_a_point_id() {
    let sel = collect_highlight_path_ids(
        &chain_graph(),
        &[RootQuery::Single {
            query: name("b"),
            raw: "b".to_string(),
        }],
    );
    assert_eq!(sel.ids, vec!["n_b"]);
    // A point query keeps the radius-1 edge rule, so the id is a point id.
    assert_eq!(sel.point_ids, vec!["n_b"]);
}

#[test]
fn direction_after_paints_the_forward_reachable_set_including_the_seed() {
    let sel = collect_highlight_path_ids(&chain_graph(), &[direction("a", QueryDir::After)]);
    assert_eq!(sel.ids, vec!["n_a", "n_b", "n_c", "n_d"]);
    // Reachability hits are NOT point ids: edges paint both-endpoint only.
    assert!(sel.point_ids.is_empty());
}

#[test]
fn direction_before_paints_the_backward_reachable_set() {
    let sel = collect_highlight_path_ids(&chain_graph(), &[direction("c", QueryDir::Before)]);
    assert_eq!(sel.ids, vec!["n_a", "n_b", "n_c"]);
    assert!(sel.point_ids.is_empty());
}

#[test]
fn direction_context_paints_both_directions() {
    let sel = collect_highlight_path_ids(&chain_graph(), &[direction("b", QueryDir::Context)]);
    assert_eq!(sel.ids, vec!["n_a", "n_b", "n_c", "n_d"]);
}

#[test]
fn path_paints_the_nodes_between_the_two_endpoints_excluding_off_path_branches() {
    let sel = collect_highlight_path_ids(&chain_graph(), &[path("a", "c")]);
    // n_d hangs off n_b but is not on any a<->c route.
    assert_eq!(sel.ids, vec!["n_a", "n_b", "n_c"]);
}

#[test]
fn path_is_direction_independent() {
    let forward = collect_highlight_path_ids(&chain_graph(), &[path("a", "c")]);
    let backward = collect_highlight_path_ids(&chain_graph(), &[path("c", "a")]);
    assert_eq!(forward.ids, backward.ids);
}

#[test]
fn path_with_no_connecting_route_paints_nothing_and_warns() {
    let sel = collect_highlight_path_ids(&chain_graph(), &[path("a", "iso")]);
    assert!(sel.ids.is_empty());
    assert_eq!(
        sel.warnings,
        vec![HighlightWarning::NoPath {
            raw: "a..iso".to_string()
        }]
    );
}

#[test]
fn a_connected_path_does_not_warn_when_a_prior_query_already_painted_its_nodes() {
    // `a..c` paints {a,b,c}; `b..c` then adds no NEW node, yet b->c is a
    // direct edge so the path IS connected. NoPath must key off this
    // path's own connectivity, not the shared accumulator's length —
    // otherwise the overlap triggers a spurious 'no connecting path'.
    let sel = collect_highlight_path_ids(&chain_graph(), &[path("a", "c"), path("b", "c")]);
    assert_eq!(sel.ids, vec!["n_a", "n_b", "n_c"]);
    assert!(sel.warnings.is_empty());
}

#[test]
fn a_direction_whose_seed_matches_nothing_warns_and_contributes_nothing() {
    let sel = collect_highlight_path_ids(&chain_graph(), &[direction("zzz", QueryDir::After)]);
    assert!(sel.ids.is_empty());
    assert_eq!(
        sel.warnings,
        vec![HighlightWarning::NoMatch {
            raw: "zzz..dir".to_string()
        }]
    );
}

#[test]
fn a_path_whose_endpoint_matches_nothing_warns_no_match_not_no_path() {
    // `zzz` matches no node, so the path is NoMatch (an endpoint found
    // nothing) — distinct from NoPath (both endpoints matched but no
    // route connects them, the `a..iso` case). The two warnings carry
    // different wording, so the `Path` NoMatch arm needs coverage of its
    // own, separate from the `Direction` NoMatch and the `Path` NoPath
    // cases.
    let sel = collect_highlight_path_ids(&chain_graph(), &[path("a", "zzz")]);
    assert!(sel.ids.is_empty());
    assert_eq!(
        sel.warnings,
        vec![HighlightWarning::NoMatch {
            raw: "a..zzz".to_string()
        }]
    );
}

#[test]
fn a_point_query_that_matches_nothing_is_silent_like_the_classic_point_highlight() {
    let sel = collect_highlight_path_ids(
        &chain_graph(),
        &[RootQuery::Single {
            query: name("zzz"),
            raw: "zzz".to_string(),
        }],
    );
    assert!(sel.ids.is_empty());
    // A bare point miss has never warned; keep that behavior.
    assert!(sel.warnings.is_empty());
}

#[test]
fn a_point_query_combined_with_a_direction_records_only_the_point_as_a_point_id() {
    let sel = collect_highlight_path_ids(
        &chain_graph(),
        &[
            RootQuery::Single {
                query: name("d"),
                raw: "d".to_string(),
            },
            direction("a", QueryDir::After),
        ],
    );
    // Full set is the forward reach of a (a,b,c,d); only d is a point id.
    assert_eq!(sel.ids, vec!["n_a", "n_b", "n_c", "n_d"]);
    assert_eq!(sel.point_ids, vec!["n_d"]);
}

// A name that appears twice — once upstream, once downstream of the
// seed. A bare `foo..bar` connects both, but a line-range endpoint
// constrains which `bar` participates, so the range chooses the path's
// direction (issue #90: `123:foo..123-9999:bar`).
fn forked_name_graph() -> VisualGraph {
    graph_of(
        vec![
            node("n_up", "bar", 1),
            node("n_foo", "foo", 5),
            node("n_down", "bar", 9),
        ],
        vec![
            VisualEdge::new("n_up", "n_foo", "read"),
            VisualEdge::new("n_foo", "n_down", "read"),
        ],
    )
}

fn range_name(start: u32, end: u32, name: &str) -> ParsedRootQuery {
    ParsedRootQuery::RangeName {
        start: unsnarl_ir::SourceLine(start),
        end: unsnarl_ir::SourceLine(end),
        name: name.to_string(),
        raw: format!("{start}-{end}:{name}"),
    }
}

#[test]
fn an_unconstrained_path_connects_both_occurrences_of_the_endpoint_name() {
    let sel = collect_highlight_path_ids(&forked_name_graph(), &[path("foo", "bar")]);
    assert_eq!(sel.ids, vec!["n_up", "n_foo", "n_down"]);
}

#[test]
fn a_downstream_line_range_endpoint_keeps_only_the_downstream_path() {
    let sel = collect_highlight_path_ids(
        &forked_name_graph(),
        &[path_q(
            name("foo"),
            range_name(6, 99, "bar"),
            "foo..6-99:bar",
        )],
    );
    // Only the line>=6 `bar` participates, so the path runs downward.
    assert_eq!(sel.ids, vec!["n_foo", "n_down"]);
}

#[test]
fn an_upstream_line_range_endpoint_keeps_only_the_upstream_path() {
    let sel = collect_highlight_path_ids(
        &forked_name_graph(),
        &[path_q(name("foo"), range_name(1, 4, "bar"), "foo..1-4:bar")],
    );
    // Only the line<=4 `bar` participates, so the path runs upward.
    assert_eq!(sel.ids, vec!["n_up", "n_foo"]);
}

// A `LineOrName` endpoint (`L7` parses ambiguously to line-7 or name
// "L7") must be resolved up front and the decision reported, exactly
// as an `-r L7` query is — issue #90 routes the `..` endpoints through
// the same resolution log.
fn line_or_name(line: u32, name: &str) -> ParsedRootQuery {
    ParsedRootQuery::LineOrName {
        line: unsnarl_ir::SourceLine(line),
        name: name.to_string(),
        raw: name.to_string(),
    }
}

#[test]
fn a_line_or_name_endpoint_is_resolved_and_recorded_in_the_resolution_log() {
    // A binding literally named `L7` makes the name interpretation
    // matchable, so the ambiguity resolves to Name.
    let graph = graph_of(
        vec![node("n_l7", "L7", 2), node("n_x", "x", 3)],
        vec![VisualEdge::new("n_l7", "n_x", "read")],
    );
    let sel = collect_highlight_path_ids(
        &graph,
        &[RootQuery::Direction {
            lhs: line_or_name(7, "L7"),
            dir: QueryDir::After,
            level: None,
            raw: "L7..+a".to_string(),
        }],
    );
    assert_eq!(sel.ids, vec!["n_l7", "n_x"]);
    assert_eq!(
        sel.resolutions,
        vec![RootQueryResolution {
            raw: "L7".to_string(),
            line: unsnarl_ir::SourceLine(7),
            name: "L7".to_string(),
            resolved_as: ResolvedAs::Name,
        }]
    );
}

fn write_op_node(id: &str, name: &str, line: u32) -> VisualNode {
    SyntheticVisualNode::write_reference(id, name, line).into()
}

fn return_use_node(id: &str, name: &str, line: u32) -> VisualNode {
    SyntheticVisualNode::return_argument_reference(id, name, line).into()
}

// Highlight diverges from `-r/--roots` on purpose: pruning's name matcher
// skips `WriteReference` / `ReturnArgumentReference` on a bare name query,
// but a point highlight paints every place the identifier appears,
// use-sites included. This pins that divergence at the collector level
// (it used to live in the now-removed `collect_highlight_ids`).
#[test]
fn a_point_name_query_paints_write_and_return_use_sites_unlike_minus_r() {
    let graph = graph_of(
        vec![
            node("n_decl", "counter", 1),
            write_op_node("n_write", "counter", 2),
            return_use_node("n_return", "counter", 3),
            node("n_other", "other", 4),
        ],
        Vec::new(),
    );
    let sel = collect_highlight_path_ids(
        &graph,
        &[RootQuery::Single {
            query: name("counter"),
            raw: "counter".to_string(),
        }],
    );
    assert_eq!(sel.ids, vec!["n_decl", "n_write", "n_return"]);
    // A point query records every hit as a point id (radius-1 edge rule).
    assert_eq!(sel.point_ids, vec!["n_decl", "n_write", "n_return"]);
}
