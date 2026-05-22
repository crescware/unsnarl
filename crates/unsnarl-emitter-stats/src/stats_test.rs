//! Constructs the `VisualGraph` directly so the test stays inside
//! the `unsnarl-emitter-stats` crate (which intentionally does not
//! depend on the parser / analyzer crates). `render_stats` is the
//! private renderer that `StatsEmitter::emit` delegates to once it
//! has a graph in hand, so calling it lets the tests pin the same
//! observable behaviour.

use unsnarl_emitter::Emitter;
use unsnarl_ir::language::Language;
use unsnarl_ir::serialized::serialized_ir::SerializedIrVersion;
use unsnarl_visual_graph::boundary_edge_direction::BoundaryEdgeDirectionOut;
use unsnarl_visual_graph::direction::Direction;
use unsnarl_visual_graph::visual_boundary_edge::VisualBoundaryEdge;
use unsnarl_visual_graph::visual_edge::VisualEdge;
use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_element_type::NodeTypeTag;
use unsnarl_visual_graph::visual_graph::{VisualGraph, VisualGraphSource};
use unsnarl_visual_graph::visual_node::{
    BindingExtras, BindingNodeKind, BindingVisualNode, VisualNode,
};

use super::{render_stats, StatsEmitter};

fn node(id: &str, name: &str, line: u32, unused: bool) -> VisualElement {
    VisualElement::Node(VisualNode::Binding(BindingVisualNode {
        r#type: NodeTypeTag::Node,
        id: id.to_string(),
        name: name.to_string(),
        line,
        end_line: None,
        is_jsx_element: false,
        unused,
        kind: BindingNodeKind::ConstBinding,
        extras: BindingExtras::Variable {
            init_is_function: false,
        },
    }))
}

fn edge(from: &str, to: &str) -> VisualEdge {
    VisualEdge {
        from: from.to_string(),
        to: to.to_string(),
        label: "read".to_string(),
    }
}

fn graph(
    elements: Vec<VisualElement>,
    edges: Vec<VisualEdge>,
    boundary_edges: Vec<VisualBoundaryEdge>,
) -> VisualGraph {
    VisualGraph {
        version: SerializedIrVersion(1),
        source: VisualGraphSource {
            path: "x.ts".to_string(),
            language: Language::Ts,
        },
        direction: Direction::TB,
        elements,
        edges,
        boundary_edges,
        pruning: None,
    }
}

#[test]
fn identifies_as_stats_with_tsv_content_type() {
    let e = StatsEmitter;
    assert_eq!(e.format(), "stats");
    assert_eq!(e.content_type(), "text/tab-separated-values");
}

#[test]
fn emits_one_tsv_row_per_node_followed_by_a_total_summary() {
    // Mirrors "const a = 1;\nconst b = a;\n": one read edge from a
    // to b; b is unused (nothing reads it).
    let g = graph(
        vec![node("a", "a", 1, false), node("b", "b", 2, true)],
        vec![edge("a", "b")],
        vec![],
    );
    let out = render_stats(&g);
    let lines: Vec<&str> = out.trim_end_matches('\n').split('\n').collect();
    assert_eq!(
        lines,
        vec!["1\t0\tx.ts:1 a", "0\t1\tx.ts:2 unused b", "1\t1\t2 total"]
    );
}

#[test]
fn zero_edge_nodes_report_zero_zero() {
    // Mirrors "const a = 1;\n": a single declaration, nothing reads
    // it, so the row reports 0 descendants / 0 ancestors and the
    // node is unused.
    let g = graph(vec![node("a", "a", 1, true)], vec![], vec![]);
    let out = render_stats(&g);
    let first = out.split('\n').next().expect("at least one row");
    assert_eq!(first, "0\t0\tx.ts:1 unused a");
}

#[test]
fn renders_question_mark_for_the_direction_touched_by_a_boundary_edge() {
    // Mirrors the pruned "const a = 1; const b = a; const c = b;
    // const d = c;" scenario where pruning keeps only {a, b} and
    // clips after b — b's outbound side becomes "?" instead of 0.
    // (b is read by c in the source so it's NOT marked unused.)
    let g = graph(
        vec![node("a", "a", 1, false), node("b", "b", 2, false)],
        vec![edge("a", "b")],
        vec![VisualBoundaryEdge::Out {
            inside: "b".to_string(),
            direction: BoundaryEdgeDirectionOut::Out,
        }],
    );
    let out = render_stats(&g);
    let lines: Vec<&str> = out.trim_end_matches('\n').split('\n').collect();
    assert_eq!(lines[0], "1\t0\tx.ts:1 a");
    assert_eq!(lines[1], "?\t1\tx.ts:2 b");
    assert_eq!(lines[2], "?\t1\t2 total");
}

#[test]
fn output_is_newline_terminated_for_shell_friendly_piping() {
    let g = graph(vec![node("a", "a", 1, true)], vec![], vec![]);
    assert!(render_stats(&g).ends_with('\n'));
}

#[test]
fn rows_are_sorted_by_line_ascending() {
    // Build the elements in a deliberately out-of-order shape (1, 3,
    // 2) so the test fails if `render_stats` accidentally relies on
    // the input order instead of running its own sort.
    let g = graph(
        vec![
            node("a", "a", 1, false),
            node("c", "c", 3, true),
            node("b", "b", 2, true),
        ],
        vec![edge("a", "b"), edge("c", "b")],
        vec![],
    );
    let out = render_stats(&g);
    let rows: Vec<&str> = out
        .trim_end_matches('\n')
        .split('\n')
        .filter(|r| !r.ends_with(" total"))
        .collect();
    let line_numbers: Vec<u32> = rows
        .iter()
        .map(|row| {
            // ".../foo.ts:<N> name..."
            let after_colon = row.split(":").nth(1).expect("path:line");
            let n: u32 = after_colon
                .split(' ')
                .next()
                .expect("number before name")
                .parse()
                .expect("line number");
            n
        })
        .collect();
    let mut sorted = line_numbers.clone();
    sorted.sort();
    assert_eq!(line_numbers, sorted);
}

#[test]
fn preserves_source_order_for_nodes_that_share_a_line_stable_sort() {
    // a and b both live on line 1; the stable line-only sort must
    // keep them in their original IR order so editors jumping to the
    // row for `a` don't accidentally land on `b` first.
    let g = graph(
        vec![node("a", "a", 1, true), node("b", "b", 1, true)],
        vec![],
        vec![],
    );
    let out = render_stats(&g);
    let rows: Vec<&str> = out
        .trim_end_matches('\n')
        .split('\n')
        .filter(|r| !r.ends_with(" total"))
        .collect();
    assert_eq!(rows, vec!["0\t0\tx.ts:1 unused a", "0\t0\tx.ts:1 unused b"]);
}
