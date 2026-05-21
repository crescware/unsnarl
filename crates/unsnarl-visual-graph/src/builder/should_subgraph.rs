//! Mirrors `ts/src/visual-graph/builder/should-subgraph.ts`.

use unsnarl_ir::serialized::SerializedScope;

use super::is_class_subgraph::is_class_subgraph;
use super::is_control_subgraph::is_control_subgraph;
use super::is_function_subgraph::is_function_subgraph;

pub fn should_subgraph(scope: &SerializedScope) -> bool {
    is_function_subgraph(scope) || is_class_subgraph(scope) || is_control_subgraph(scope)
}

#[cfg(test)]
#[path = "should_subgraph_test.rs"]
mod should_subgraph_test;
