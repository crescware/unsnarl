//! Mirrors `ts/src/emitter/mermaid/render-pruning-comment.test.ts`.

use unsnarl_visual_graph::visual_graph_pruning::{PruningRoot, VisualGraphPruning};

use super::render_pruning_comment;
use crate::testing::base_graph;

#[test]
fn does_nothing_when_graph_pruning_is_none() {
    let mut lines: Vec<String> = Vec::new();
    render_pruning_comment(&base_graph(), &mut lines);
    assert!(lines.is_empty());
}

#[test]
fn emits_a_single_comment_summarising_roots_ancestors_and_descendants() {
    let mut g = base_graph();
    g.pruning = Some(VisualGraphPruning {
        roots: vec![
            PruningRoot {
                query: "L5".to_string(),
                matched: 1,
            },
            PruningRoot {
                query: "L9".to_string(),
                matched: 2,
            },
        ],
        ancestors: 3,
        descendants: 4,
    });
    let mut lines: Vec<String> = Vec::new();
    render_pruning_comment(&g, &mut lines);
    assert_eq!(
        lines,
        vec!["  %% pruning roots L5=1 L9=2 ancestors=3 descendants=4".to_string()]
    );
}

#[test]
fn appends_a_warning_line_for_each_zero_match_root() {
    let mut g = base_graph();
    g.pruning = Some(VisualGraphPruning {
        roots: vec![
            PruningRoot {
                query: "L1".to_string(),
                matched: 0,
            },
            PruningRoot {
                query: "L9".to_string(),
                matched: 1,
            },
            PruningRoot {
                query: "missing".to_string(),
                matched: 0,
            },
        ],
        ancestors: 0,
        descendants: 0,
    });
    let mut lines: Vec<String> = Vec::new();
    render_pruning_comment(&g, &mut lines);
    assert!(lines.contains(&"  %% pruning warning query L1 matched 0 roots".to_string()));
    assert!(lines.contains(&"  %% pruning warning query missing matched 0 roots".to_string()));
    assert!(!lines
        .iter()
        .any(|v| v.contains("warning") && v.contains("L9")));
}

#[test]
fn empty_roots_list_still_emits_the_summary_line() {
    let mut g = base_graph();
    g.pruning = Some(VisualGraphPruning {
        roots: Vec::new(),
        ancestors: 0,
        descendants: 0,
    });
    let mut lines: Vec<String> = Vec::new();
    render_pruning_comment(&g, &mut lines);
    assert_eq!(lines.len(), 1);
}
