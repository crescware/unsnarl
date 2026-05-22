//! Predicate selecting nodes that are emitted in the trailing
//! "synthetic node" block rather than at the top of the diagram.

use unsnarl_visual_graph::node_kind::NodeKind;
use unsnarl_visual_graph::visual_node::VisualNode;

pub fn renders_in_synthetic_block(n: &VisualNode) -> bool {
    matches!(
        n.kind(),
        NodeKind::SyntheticModuleSink
            | NodeKind::SyntheticModuleSource
            | NodeKind::SyntheticImportIntermediate
            | NodeKind::SyntheticExpressionStatement
    )
}

#[cfg(test)]
#[path = "renders_in_synthetic_block_test.rs"]
mod renders_in_synthetic_block_test;
