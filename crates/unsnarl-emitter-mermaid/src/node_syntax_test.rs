use unsnarl_visual_graph::visual_node::{
    BindingVisualNode, SyntheticNodeKind, SyntheticVisualNode, VisualNode,
};

use super::node_syntax;
use crate::mermaid_fixtures::{base_simple_synthetic, base_write_op};

#[test]
fn write_op_uses_stadium_brackets() {
    let n: VisualNode = SyntheticVisualNode {
        name: "x".to_string(),
        line: 3,
        ..base_write_op()
    }
    .into();
    let got = node_syntax(&n, false);
    assert!(got.starts_with("([\""));
    assert!(got.ends_with("\"])"));
}

#[test]
fn module_sink_uses_double_circle_brackets() {
    let n: VisualNode = SyntheticVisualNode {
        name: "module".to_string(),
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticModuleSink)
    }
    .into();
    assert_eq!(node_syntax(&n, false), "((module))");
}

#[test]
fn if_test_uses_diamond_brackets() {
    let n: VisualNode = SyntheticVisualNode {
        name: "if-test".to_string(),
        line: 5,
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticIfStatementTest)
    }
    .into();
    assert_eq!(node_syntax(&n, false), "{\"if ()<br/>L5\"}");
}

#[test]
fn switch_discriminant_uses_diamond_brackets() {
    let n: VisualNode = SyntheticVisualNode {
        name: "switch-discriminant".to_string(),
        line: 7,
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticSwitchStatementDiscriminant)
    }
    .into();
    assert_eq!(node_syntax(&n, false), "{\"switch ()<br/>L7\"}");
}

#[test]
fn default_kind_uses_square_brackets() {
    let n: VisualNode = BindingVisualNode {
        name: "x".to_string(),
        line: 4,
        ..crate::mermaid_fixtures::base_const_binding()
    }
    .into();
    let got = node_syntax(&n, false);
    assert!(got.starts_with("[\""));
    assert!(got.ends_with("\"]"));
}

#[test]
fn debug_true_threads_node_kind_into_the_inner_label() {
    let n: VisualNode = SyntheticVisualNode {
        name: "if-test".to_string(),
        line: 5,
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticIfStatementTest)
    }
    .into();
    assert_eq!(
        node_syntax(&n, true),
        "{\"if ()<br/>L5<br/>SyntheticIfStatementTest\"}"
    );
}
