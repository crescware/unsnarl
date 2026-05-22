//! Sibling tests for [`make_variable_node`].

use unsnarl_ir::serialized::serialized_definition::{
    ImportBindingDefaultDef, ImportBindingNamedDef, SerializedDefinition, SimpleDefType,
};
use unsnarl_ir::serialized::SerializedVariable;
use unsnarl_oxc_parity::{AstType, VariableDeclarationKind};

use super::make_variable_node;
use crate::builder::testing::{
    base_def, base_serialized_variable, base_simple_def, definition_name, definition_node,
    reference_id, scope_id, span, span_offset_line, variable_id,
};
use crate::visual_node::{BindingExtras, BindingNodeKind, VisualNode};

fn variable_with_def(
    id: &str,
    name: &str,
    identifiers_span: Option<unsnarl_ir::primitive::Span>,
    def: SerializedDefinition,
) -> SerializedVariable {
    SerializedVariable::new(
        variable_id(id),
        name.to_string(),
        scope_id("s"),
        identifiers_span.into_iter().collect(),
        Vec::new(),
        vec![def],
    )
}

fn variable_def_with_init(
    declaration_kind: VariableDeclarationKind,
    init_type: AstType,
) -> SerializedDefinition {
    SerializedDefinition::Variable(
        unsnarl_ir::serialized::serialized_definition::VariableDef::new(
            definition_name("x", span(0)),
            definition_node(AstType::Identifier, span(0)),
            None,
            Some(definition_node(init_type, span(0))),
            declaration_kind,
        ),
    )
}

#[test]
fn let_declared_variable_produces_a_let_binding_kind_node() {
    let v = SerializedVariable::new(
        variable_id("v1"),
        "x".to_string(),
        scope_id("s"),
        vec![span_offset_line(0, 2)],
        Vec::new(),
        vec![base_def(VariableDeclarationKind::Let)],
    );
    let node = make_variable_node(&v);
    let VisualNode::Binding(b) = &node else {
        panic!("expected binding node");
    };
    assert_eq!(b.id, "n_v1");
    assert_eq!(b.name, "x");
    assert_eq!(b.line, 2);
    assert!(!b.is_jsx_element);
    assert!(matches!(b.kind, BindingNodeKind::LetBinding));
    let BindingExtras::Variable { init_is_function } = &b.extras else {
        panic!("expected Variable extras");
    };
    assert!(!init_is_function);
}

#[test]
fn falls_back_to_def_name_span_line_when_identifiers_is_empty() {
    let def = SerializedDefinition::Variable(
        unsnarl_ir::serialized::serialized_definition::VariableDef::new(
            definition_name("x", span_offset_line(0, 7)),
            definition_node(AstType::Identifier, span(0)),
            None,
            None,
            VariableDeclarationKind::Let,
        ),
    );
    let v = variable_with_def("v", "x", None, def);
    let VisualNode::Binding(b) = make_variable_node(&v) else {
        panic!("expected binding");
    };
    assert_eq!(b.line, 7);
}

#[test]
fn implicit_global_variable_forces_line_zero() {
    let v = SerializedVariable::new(
        variable_id("v"),
        "Math".to_string(),
        scope_id("s"),
        vec![span_offset_line(0, 4)],
        Vec::new(),
        vec![base_simple_def(SimpleDefType::ImplicitGlobalVariable)],
    );
    let VisualNode::Binding(b) = make_variable_node(&v) else {
        panic!("expected binding");
    };
    assert!(matches!(b.kind, BindingNodeKind::SyntheticImplicitGlobal));
    assert_eq!(b.line, 0);
}

#[test]
fn arrow_function_expression_init_yields_init_is_function_true() {
    let v = variable_with_def(
        "v",
        "x",
        Some(span(0)),
        variable_def_with_init(
            VariableDeclarationKind::Let,
            AstType::ArrowFunctionExpression,
        ),
    );
    let VisualNode::Binding(b) = make_variable_node(&v) else {
        panic!("expected binding");
    };
    let BindingExtras::Variable { init_is_function } = b.extras else {
        panic!("expected Variable extras");
    };
    assert!(init_is_function);
}

#[test]
fn function_expression_init_yields_init_is_function_true() {
    let v = variable_with_def(
        "v",
        "x",
        Some(span(0)),
        variable_def_with_init(VariableDeclarationKind::Let, AstType::FunctionExpression),
    );
    let VisualNode::Binding(b) = make_variable_node(&v) else {
        panic!("expected binding");
    };
    let BindingExtras::Variable { init_is_function } = b.extras else {
        panic!("expected Variable extras");
    };
    assert!(init_is_function);
}

#[test]
fn non_function_init_yields_init_is_function_false() {
    let v = variable_with_def(
        "v",
        "x",
        Some(span(0)),
        variable_def_with_init(VariableDeclarationKind::Let, AstType::Literal),
    );
    let VisualNode::Binding(b) = make_variable_node(&v) else {
        panic!("expected binding");
    };
    let BindingExtras::Variable { init_is_function } = b.extras else {
        panic!("expected Variable extras");
    };
    assert!(!init_is_function);
}

#[test]
fn var_is_emitted_as_var_binding_node() {
    let mut v = base_serialized_variable();
    v.defs = vec![base_def(VariableDeclarationKind::Var)];
    let VisualNode::Binding(b) = make_variable_node(&v) else {
        panic!("expected binding");
    };
    assert!(matches!(b.kind, BindingNodeKind::VarBinding));
}

#[test]
fn const_is_emitted_as_const_binding_node() {
    let mut v = base_serialized_variable();
    v.defs = vec![base_def(VariableDeclarationKind::Const)];
    let VisualNode::Binding(b) = make_variable_node(&v) else {
        panic!("expected binding");
    };
    assert!(matches!(b.kind, BindingNodeKind::ConstBinding));
}

#[test]
fn let_is_emitted_as_let_binding_node() {
    let v = base_serialized_variable();
    let VisualNode::Binding(b) = make_variable_node(&v) else {
        panic!("expected binding");
    };
    assert!(matches!(b.kind, BindingNodeKind::LetBinding));
}

#[test]
fn named_import_binding_propagates_imported_name() {
    let def = SerializedDefinition::ImportBindingNamed(ImportBindingNamedDef::new(
        definition_name("renamed", span(0)),
        definition_node(AstType::Identifier, span(0)),
        None,
        "original".to_string(),
        "./mod.js".to_string(),
    ));
    let v = SerializedVariable::new(
        variable_id("v"),
        "renamed".to_string(),
        scope_id("s"),
        vec![span(0)],
        Vec::new(),
        vec![def],
    );
    // Suppress the unused-variable warning for the helper alias.
    let _ = reference_id;
    let VisualNode::Binding(b) = make_variable_node(&v) else {
        panic!("expected binding");
    };
    assert!(matches!(b.kind, BindingNodeKind::NamedImportBinding));
    let BindingExtras::NamedImport { imported_name } = &b.extras else {
        panic!("expected NamedImport extras");
    };
    assert_eq!(imported_name, "original");
}

#[test]
fn default_import_binding_has_no_imported_name_field() {
    let def = SerializedDefinition::ImportBindingDefault(ImportBindingDefaultDef::new(
        definition_name("x", span(0)),
        definition_node(AstType::Identifier, span(0)),
        None,
        "./mod.js".to_string(),
    ));
    let v = SerializedVariable::new(
        variable_id("v"),
        "x".to_string(),
        scope_id("s"),
        vec![span(0)],
        Vec::new(),
        vec![def],
    );
    let VisualNode::Binding(b) = make_variable_node(&v) else {
        panic!("expected binding");
    };
    assert!(matches!(b.kind, BindingNodeKind::DefaultImportBinding));
    assert!(matches!(b.extras, BindingExtras::None {}));
}
