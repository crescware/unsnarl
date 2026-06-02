use unsnarl_oxc_parity::VariableDeclarationKind;
use unsnarl_visual_graph::visual_node::{
    BindingExtras, BindingNodeKind, BindingVisualNode, SyntheticExtras, SyntheticNodeKind,
    SyntheticVisualNode, VisualNode,
};

use super::node_head;
use crate::mermaid_fixtures::{
    base_await_using_binding, base_const_binding, base_import_binding_default,
    base_import_binding_named, base_import_binding_namespace, base_let_binding,
    base_simple_binding, base_simple_synthetic, base_using_binding, base_var_binding,
    base_write_op,
};

#[test]
fn jsx_element_wraps_the_escaped_name_in_angle_brackets_ignoring_kind() {
    let n: VisualNode = BindingVisualNode {
        kind: BindingNodeKind::FunctionDeclaration,
        extras: BindingExtras::None {},
        name: "Foo".to_string(),
        is_jsx_element: true,
        ..base_const_binding()
    }
    .into();
    assert_eq!(node_head(&n), "&lt;Foo&gt;");
}

#[test]
fn function_declaration_formats_as_name_with_parens() {
    let n: VisualNode = BindingVisualNode {
        name: "foo".to_string(),
        ..base_simple_binding(BindingNodeKind::FunctionDeclaration)
    }
    .into();
    assert_eq!(node_head(&n), "foo()");
}

#[test]
fn class_declaration_formats_as_class_name() {
    let n: VisualNode = BindingVisualNode {
        name: "Foo".to_string(),
        ..base_simple_binding(BindingNodeKind::ClassDeclaration)
    }
    .into();
    assert_eq!(node_head(&n), "class Foo");
}

#[test]
fn catch_parameter_formats_as_catch_name() {
    let n: VisualNode = BindingVisualNode {
        name: "err".to_string(),
        ..base_simple_binding(BindingNodeKind::CatchParameter)
    }
    .into();
    assert_eq!(node_head(&n), "catch err");
}

#[test]
fn synthetic_implicit_global_formats_as_global_name() {
    let n: VisualNode = BindingVisualNode {
        name: "global1".to_string(),
        ..base_simple_binding(BindingNodeKind::SyntheticImplicitGlobal)
    }
    .into();
    assert_eq!(node_head(&n), "global global1");
}

#[test]
fn synthetic_import_intermediate_formats_as_import_name() {
    let n: VisualNode = SyntheticVisualNode {
        name: "named".to_string(),
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticImportIntermediate)
    }
    .into();
    assert_eq!(node_head(&n), "import named");
}

#[test]
fn synthetic_expression_statement_uses_name_only() {
    let n: VisualNode = SyntheticVisualNode {
        name: "console.log()".to_string(),
        ..base_simple_synthetic(SyntheticNodeKind::SyntheticExpressionStatement)
    }
    .into();
    assert_eq!(node_head(&n), "console.log()");
}

#[test]
fn renamed_named_import_keeps_the_local_name_only() {
    let n: VisualNode = BindingVisualNode {
        name: "renamed".to_string(),
        ..base_import_binding_named("original")
    }
    .into();
    assert_eq!(node_head(&n), "renamed");
}

#[test]
fn non_renamed_named_import_gets_import_prefix() {
    let n: VisualNode = BindingVisualNode {
        name: "foo".to_string(),
        ..base_import_binding_named("foo")
    }
    .into();
    assert_eq!(node_head(&n), "import foo");
}

#[test]
fn default_import_gets_import_prefix() {
    let n: VisualNode = BindingVisualNode {
        name: "Foo".to_string(),
        ..base_import_binding_default()
    }
    .into();
    assert_eq!(node_head(&n), "import Foo");
}

#[test]
fn namespace_import_gets_import_prefix() {
    let n: VisualNode = BindingVisualNode {
        name: "ns".to_string(),
        ..base_import_binding_namespace()
    }
    .into();
    assert_eq!(node_head(&n), "import ns");
}

#[test]
fn write_op_with_declaration_kind_let_prepends_let() {
    let n: VisualNode = SyntheticVisualNode {
        name: "x".to_string(),
        extras: SyntheticExtras::WriteOp {
            declaration_kind: Some(VariableDeclarationKind::Let),
        },
        ..base_write_op()
    }
    .into();
    assert_eq!(node_head(&n), "let x");
}

#[test]
fn write_op_with_declaration_kind_const_has_no_prefix() {
    let n: VisualNode = SyntheticVisualNode {
        name: "x".to_string(),
        extras: SyntheticExtras::WriteOp {
            declaration_kind: Some(VariableDeclarationKind::Const),
        },
        ..base_write_op()
    }
    .into();
    assert_eq!(node_head(&n), "x");
}

#[test]
fn write_op_without_declaration_kind_has_no_prefix() {
    let n: VisualNode = SyntheticVisualNode {
        name: "x".to_string(),
        ..base_write_op()
    }
    .into();
    assert_eq!(node_head(&n), "x");
}

#[test]
fn const_binding_initialized_with_a_function_uses_paren_format() {
    let n: VisualNode = BindingVisualNode {
        name: "f".to_string(),
        extras: BindingExtras::Variable {
            init_is_function: true,
        },
        ..base_const_binding()
    }
    .into();
    assert_eq!(node_head(&n), "f()");
}

#[test]
fn let_binding_prepends_let() {
    let n: VisualNode = BindingVisualNode {
        name: "x".to_string(),
        ..base_let_binding()
    }
    .into();
    assert_eq!(node_head(&n), "let x");
}

#[test]
fn const_binding_has_no_prefix() {
    let n: VisualNode = BindingVisualNode {
        name: "x".to_string(),
        ..base_const_binding()
    }
    .into();
    assert_eq!(node_head(&n), "x");
}

#[test]
fn using_binding_prepends_using() {
    let n: VisualNode = BindingVisualNode {
        name: "x".to_string(),
        ..base_using_binding()
    }
    .into();
    assert_eq!(node_head(&n), "using x");
}

#[test]
fn await_using_binding_prepends_await_using() {
    let n: VisualNode = BindingVisualNode {
        name: "x".to_string(),
        ..base_await_using_binding()
    }
    .into();
    assert_eq!(node_head(&n), "await using x");
}

#[test]
fn using_binding_initialized_with_a_function_uses_paren_format() {
    let n: VisualNode = BindingVisualNode {
        name: "f".to_string(),
        extras: BindingExtras::Variable {
            init_is_function: true,
        },
        ..base_using_binding()
    }
    .into();
    assert_eq!(node_head(&n), "f()");
}

#[test]
fn var_declared_var_binding_prepends_var() {
    let n: VisualNode = BindingVisualNode {
        name: "x".to_string(),
        ..base_var_binding()
    }
    .into();
    assert_eq!(node_head(&n), "var x");
}

#[test]
fn init_is_function_wins_over_the_var_prefix() {
    let n: VisualNode = BindingVisualNode {
        name: "f".to_string(),
        extras: BindingExtras::Variable {
            init_is_function: true,
        },
        ..base_var_binding()
    }
    .into();
    assert_eq!(node_head(&n), "f()");
}

#[test]
fn return_use_falls_through_to_the_default_formatting() {
    let n: VisualNode = SyntheticVisualNode {
        name: "x".to_string(),
        ..base_simple_synthetic(SyntheticNodeKind::ReturnArgumentReference)
    }
    .into();
    assert_eq!(node_head(&n), "x");
}
