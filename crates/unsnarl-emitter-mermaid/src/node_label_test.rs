use unsnarl_visual_graph::visual_node::{
    BindingNodeKind, BindingVisualNode, SyntheticNodeKind, SyntheticVisualNode, VisualNode,
};

use super::node_label;
use crate::testing::{base_const_binding, base_simple_binding, base_simple_synthetic};

#[test]
fn if_test_emits_if_with_line_only() {
    let n: VisualNode = SyntheticVisualNode {
        name: "ignored".to_string(),
        line: 3,
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticIfStatementTest)
    }
    .into();
    assert_eq!(node_label(&n, false), "if ()<br/>L3");
}

#[test]
fn switch_discriminant_emits_switch_with_line() {
    let n: VisualNode = SyntheticVisualNode {
        name: "ignored".to_string(),
        line: 6,
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticSwitchStatementDiscriminant)
    }
    .into();
    assert_eq!(node_label(&n, false), "switch ()<br/>L6");
}

#[test]
fn while_test_emits_while_with_line() {
    let n: VisualNode = SyntheticVisualNode {
        name: "ignored".to_string(),
        line: 5,
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticWhileStatementTest)
    }
    .into();
    assert_eq!(node_label(&n, false), "while ()<br/>L5");
}

#[test]
fn do_while_test_emits_do_while_with_line() {
    let n: VisualNode = SyntheticVisualNode {
        name: "ignored".to_string(),
        line: 7,
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticDoWhileStatementTest)
    }
    .into();
    assert_eq!(node_label(&n, false), "do while ()<br/>L7");
}

#[test]
fn for_test_emits_for_with_line() {
    let n: VisualNode = SyntheticVisualNode {
        name: "ignored".to_string(),
        line: 4,
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticForStatementHeader)
    }
    .into();
    assert_eq!(node_label(&n, false), "for ()<br/>L4");
}

#[test]
fn module_sink_shortcuts_to_the_literal_module() {
    let n: VisualNode = SyntheticVisualNode {
        name: "ignored".to_string(),
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticModuleSink)
    }
    .into();
    assert_eq!(node_label(&n, false), "module");
}

#[test]
fn implicit_global_variable_renders_without_a_line_suffix() {
    let n: VisualNode = BindingVisualNode {
        name: "Math".to_string(),
        line: 0,
        ..base_simple_binding(BindingNodeKind::SyntheticImplicitGlobal)
    }
    .into();
    assert_eq!(node_label(&n, false), "global Math");
}

#[test]
fn expression_statement_renders_the_head_followed_by_the_statement_line() {
    let n: VisualNode = SyntheticVisualNode {
        name: "console.log()".to_string(),
        line: 7,
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticExpressionStatement)
    }
    .into();
    assert_eq!(node_label(&n, false), "console.log()<br/>L7");
}

#[test]
fn appends_the_line_range_as_a_single_line() {
    let n: VisualNode = BindingVisualNode {
        name: "x".to_string(),
        line: 7,
        ..base_const_binding()
    }
    .into();
    assert_eq!(node_label(&n, false), "x<br/>L7");
}

#[test]
fn appends_the_line_range_when_end_line_differs_from_line() {
    let n: VisualNode = BindingVisualNode {
        name: "x".to_string(),
        line: 7,
        end_line: Some(9),
        ..base_const_binding()
    }
    .into();
    assert_eq!(node_label(&n, false), "x<br/>L7-9");
}

#[test]
fn collapses_to_a_single_line_when_end_line_equals_line() {
    let n: VisualNode = BindingVisualNode {
        name: "x".to_string(),
        line: 4,
        end_line: Some(4),
        ..base_const_binding()
    }
    .into();
    assert_eq!(node_label(&n, false), "x<br/>L4");
}

#[test]
fn prefixes_with_unused_when_node_unused_is_true() {
    let n: VisualNode = BindingVisualNode {
        name: "x".to_string(),
        line: 2,
        unused: true,
        ..base_const_binding()
    }
    .into();
    assert_eq!(node_label(&n, false), "unused x<br/>L2");
}

#[test]
fn unused_prefix_is_suppressed_when_unused_is_false() {
    let n_false: VisualNode = BindingVisualNode {
        name: "x".to_string(),
        unused: false,
        ..base_const_binding()
    }
    .into();
    assert_eq!(node_label(&n_false, false), "x<br/>L1");
    let n_default: VisualNode = BindingVisualNode {
        name: "x".to_string(),
        ..base_const_binding()
    }
    .into();
    assert_eq!(node_label(&n_default, false), "x<br/>L1");
}

#[test]
fn debug_true_appends_node_kind_to_the_standard_label() {
    let n: VisualNode = BindingVisualNode {
        name: "x".to_string(),
        line: 7,
        ..base_const_binding()
    }
    .into();
    assert_eq!(node_label(&n, true), "x<br/>L7<br/>ConstBinding");
}

#[test]
fn debug_true_appends_node_kind_to_the_if_test_anchor_label() {
    let n: VisualNode = SyntheticVisualNode {
        name: "ignored".to_string(),
        line: 3,
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticIfStatementTest)
    }
    .into();
    assert_eq!(
        node_label(&n, true),
        "if ()<br/>L3<br/>SyntheticIfStatementTest"
    );
}

#[test]
fn debug_true_appends_node_kind_to_module_sink_even_when_the_base_label_has_no_line() {
    let n: VisualNode = SyntheticVisualNode {
        name: "ignored".to_string(),
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticModuleSink)
    }
    .into();
    assert_eq!(node_label(&n, true), "module<br/>SyntheticModuleSink");
}

#[test]
fn debug_true_appends_node_kind_to_implicit_global_variable() {
    let n: VisualNode = BindingVisualNode {
        name: "Math".to_string(),
        line: 0,
        ..base_simple_binding(BindingNodeKind::SyntheticImplicitGlobal)
    }
    .into();
    assert_eq!(
        node_label(&n, true),
        "global Math<br/>SyntheticImplicitGlobal"
    );
}
