use unsnarl_visual_graph::node_kind::NodeKind;
use unsnarl_visual_graph::visual_node::{BindingNodeKind, SyntheticNodeKind, VisualNode};

use super::renders_in_synthetic_block;
use crate::mermaid_fixtures::{
    base_const_binding, base_import_binding_default, base_import_binding_named,
    base_import_binding_namespace, base_let_binding, base_simple_binding, base_simple_synthetic,
    base_var_binding, base_write_op,
};

fn node_of_kind(kind: NodeKind) -> VisualNode {
    match kind {
        NodeKind::ConstBinding => VisualNode::Binding(base_const_binding()),
        NodeKind::LetBinding => VisualNode::Binding(base_let_binding()),
        NodeKind::VarBinding => VisualNode::Binding(base_var_binding()),
        NodeKind::WriteReference => VisualNode::Synthetic(base_write_op()),
        NodeKind::DefaultImportBinding => VisualNode::Binding(base_import_binding_default()),
        NodeKind::NamedImportBinding => VisualNode::Binding(base_import_binding_named("imported")),
        NodeKind::NamespaceImportBinding => VisualNode::Binding(base_import_binding_namespace()),
        // Remaining BindingNodeKind variants
        NodeKind::FunctionDeclaration => {
            VisualNode::Binding(base_simple_binding(BindingNodeKind::FunctionDeclaration))
        }
        NodeKind::ClassDeclaration => {
            VisualNode::Binding(base_simple_binding(BindingNodeKind::ClassDeclaration))
        }
        NodeKind::FormalParameter => {
            VisualNode::Binding(base_simple_binding(BindingNodeKind::FormalParameter))
        }
        NodeKind::CatchParameter => {
            VisualNode::Binding(base_simple_binding(BindingNodeKind::CatchParameter))
        }
        NodeKind::SyntheticImplicitGlobal => VisualNode::Binding(base_simple_binding(
            BindingNodeKind::SyntheticImplicitGlobal,
        )),
        // Remaining SyntheticNodeKind variants
        NodeKind::ReturnArgumentReference => VisualNode::Synthetic(base_simple_synthetic(
            SyntheticNodeKind::ReturnArgumentReference,
        )),
        NodeKind::ThrowArgumentReference => VisualNode::Synthetic(base_simple_synthetic(
            SyntheticNodeKind::ThrowArgumentReference,
        )),
        NodeKind::SyntheticIfStatementTest => VisualNode::Synthetic(base_simple_synthetic(
            SyntheticNodeKind::SyntheticIfStatementTest,
        )),
        NodeKind::SyntheticSwitchStatementDiscriminant => VisualNode::Synthetic(
            base_simple_synthetic(SyntheticNodeKind::SyntheticSwitchStatementDiscriminant),
        ),
        NodeKind::SyntheticWhileStatementTest => VisualNode::Synthetic(base_simple_synthetic(
            SyntheticNodeKind::SyntheticWhileStatementTest,
        )),
        NodeKind::SyntheticDoWhileStatementTest => VisualNode::Synthetic(base_simple_synthetic(
            SyntheticNodeKind::SyntheticDoWhileStatementTest,
        )),
        NodeKind::SyntheticForStatementHeader => VisualNode::Synthetic(base_simple_synthetic(
            SyntheticNodeKind::SyntheticForStatementHeader,
        )),
        NodeKind::SyntheticForInStatementHeader => VisualNode::Synthetic(base_simple_synthetic(
            SyntheticNodeKind::SyntheticForInStatementHeader,
        )),
        NodeKind::SyntheticForOfStatementHeader => VisualNode::Synthetic(base_simple_synthetic(
            SyntheticNodeKind::SyntheticForOfStatementHeader,
        )),
        NodeKind::SyntheticModuleSink => VisualNode::Synthetic(base_simple_synthetic(
            SyntheticNodeKind::SyntheticModuleSink,
        )),
        NodeKind::SyntheticModuleSource => VisualNode::Synthetic(base_simple_synthetic(
            SyntheticNodeKind::SyntheticModuleSource,
        )),
        NodeKind::SyntheticImportIntermediate => VisualNode::Synthetic(base_simple_synthetic(
            SyntheticNodeKind::SyntheticImportIntermediate,
        )),
        NodeKind::SyntheticExpressionStatement => VisualNode::Synthetic(base_simple_synthetic(
            SyntheticNodeKind::SyntheticExpressionStatement,
        )),
        NodeKind::SyntheticBeyondDepth => VisualNode::Synthetic(base_simple_synthetic(
            SyntheticNodeKind::SyntheticBeyondDepth,
        )),
    }
}

#[track_caller]
fn check(kind: NodeKind, expected: bool) {
    let n = node_of_kind(kind);
    assert_eq!(
        renders_in_synthetic_block(&n),
        expected,
        "kind={:?}",
        kind.as_str()
    );
}

#[test]
fn synthetic_module_sink_renders_in_synthetic_block() {
    check(NodeKind::SyntheticModuleSink, true);
}

#[test]
fn synthetic_module_source_renders_in_synthetic_block() {
    check(NodeKind::SyntheticModuleSource, true);
}

#[test]
fn synthetic_import_intermediate_renders_in_synthetic_block() {
    check(NodeKind::SyntheticImportIntermediate, true);
}

#[test]
fn synthetic_expression_statement_renders_in_synthetic_block() {
    check(NodeKind::SyntheticExpressionStatement, true);
}

#[test]
fn var_binding_does_not_render_in_synthetic_block() {
    check(NodeKind::VarBinding, false);
}

#[test]
fn const_binding_does_not_render_in_synthetic_block() {
    check(NodeKind::ConstBinding, false);
}

#[test]
fn let_binding_does_not_render_in_synthetic_block() {
    check(NodeKind::LetBinding, false);
}

#[test]
fn function_declaration_does_not_render_in_synthetic_block() {
    check(NodeKind::FunctionDeclaration, false);
}

#[test]
fn class_declaration_does_not_render_in_synthetic_block() {
    check(NodeKind::ClassDeclaration, false);
}

#[test]
fn formal_parameter_does_not_render_in_synthetic_block() {
    check(NodeKind::FormalParameter, false);
}

#[test]
fn catch_parameter_does_not_render_in_synthetic_block() {
    check(NodeKind::CatchParameter, false);
}

#[test]
fn named_import_binding_does_not_render_in_synthetic_block() {
    check(NodeKind::NamedImportBinding, false);
}

#[test]
fn default_import_binding_does_not_render_in_synthetic_block() {
    check(NodeKind::DefaultImportBinding, false);
}

#[test]
fn namespace_import_binding_does_not_render_in_synthetic_block() {
    check(NodeKind::NamespaceImportBinding, false);
}

#[test]
fn synthetic_implicit_global_does_not_render_in_synthetic_block() {
    check(NodeKind::SyntheticImplicitGlobal, false);
}

#[test]
fn write_reference_does_not_render_in_synthetic_block() {
    check(NodeKind::WriteReference, false);
}

#[test]
fn return_argument_reference_does_not_render_in_synthetic_block() {
    check(NodeKind::ReturnArgumentReference, false);
}

#[test]
fn throw_argument_reference_does_not_render_in_synthetic_block() {
    check(NodeKind::ThrowArgumentReference, false);
}
