use unsnarl_visual_graph::visual_subgraph::{OwnedVisualSubgraph, VisualSubgraph};

use super::line_range_label;
use crate::testing::base_function_subgraph;

fn subgraph(line: u32, end_line: Option<u32>) -> VisualSubgraph {
    OwnedVisualSubgraph {
        line,
        end_line,
        ..base_function_subgraph()
    }
    .into()
}

#[test]
fn single_line_when_end_line_is_none() {
    assert_eq!(line_range_label(&subgraph(5, None)), "L5");
}

#[test]
fn single_line_when_end_line_equals_line() {
    assert_eq!(line_range_label(&subgraph(5, Some(5))), "L5");
}

#[test]
fn range_when_end_line_differs_from_line() {
    assert_eq!(line_range_label(&subgraph(5, Some(10))), "L5-10");
}

#[test]
fn range_with_adjacent_lines() {
    assert_eq!(line_range_label(&subgraph(1, Some(2))), "L1-2");
}
