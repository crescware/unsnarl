//! Sibling tests for [`describe_subgraph`].

use std::collections::HashMap;

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::serialized_scope::SerializedBlock;
use unsnarl_ir::serialized::SerializedVariable;
use unsnarl_oxc_parity::AstType;

use super::describe_subgraph;
use crate::builder::builder_fixtures::{
    base_serialized_scope, base_serialized_variable, case_clause_block_context, scope_id,
    span_offset_line, variable_id,
};
use crate::direction::Direction;
use crate::visual_subgraph::{
    ControlExtras, ControlSubgraphKind, OwnedExtras, OwnedSubgraphKind, VisualSubgraph,
};

fn block(
    r#type: AstType,
    span_offset: u32,
    span_line: u32,
    end_offset: u32,
    end_line: u32,
) -> SerializedBlock {
    SerializedBlock {
        r#type,
        span: span_offset_line(span_offset, span_line),
        end_span: span_offset_line(end_offset, end_line),
    }
}

#[test]
fn function_subgraph_returns_kind_function_with_owner_metadata() {
    let mut scope = base_serialized_scope("fnScope");
    scope.r#type = ScopeType::Function;
    scope.block = block(AstType::FunctionDeclaration, 0, 5, 50, 10);

    let owner = SerializedVariable::new(
        variable_id("ownerVar"),
        "myFn".to_string(),
        scope_id("s"),
        vec![span_offset_line(0, 5)],
        Vec::new(),
        Vec::new(),
    );
    let mut owners: HashMap<String, String> = HashMap::new();
    owners.insert("fnScope".to_string(), "ownerVar".to_string());
    let mut variables: HashMap<&str, &SerializedVariable> = HashMap::new();
    variables.insert("ownerVar", &owner);

    let sg = describe_subgraph(
        &scope,
        &owners,
        &variables,
        &unsnarl_ir::primitive::SourceIndex::build(""),
    );
    let VisualSubgraph::Owned(s) = sg else {
        panic!("expected owned subgraph");
    };
    assert_eq!(s.id, "s_fnScope");
    assert!(matches!(s.kind, OwnedSubgraphKind::Function));
    assert!(matches!(s.direction, Direction::RL));
    assert_eq!(s.line, 5);
    assert_eq!(s.end_line, Some(10));
    assert!(s.elements.is_empty());
    let OwnedExtras::Function {
        owner_node_id,
        owner_name,
    } = s.extras
    else {
        panic!("expected Function extras");
    };
    assert_eq!(owner_node_id.as_deref(), Some("n_ownerVar"));
    assert_eq!(owner_name, "myFn");
}

#[test]
fn function_subgraph_falls_back_to_scope_block_span_line_when_owner_has_no_identifiers() {
    let mut scope = base_serialized_scope("fn");
    scope.r#type = ScopeType::Function;
    scope.block = block(AstType::FunctionDeclaration, 0, 7, 20, 9);
    let owner = SerializedVariable::new(
        variable_id("o"),
        "f".to_string(),
        scope_id("s"),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let mut owners: HashMap<String, String> = HashMap::new();
    owners.insert("fn".to_string(), "o".to_string());
    let mut variables: HashMap<&str, &SerializedVariable> = HashMap::new();
    variables.insert("o", &owner);
    let VisualSubgraph::Owned(s) = describe_subgraph(
        &scope,
        &owners,
        &variables,
        &unsnarl_ir::primitive::SourceIndex::build(""),
    ) else {
        panic!("expected owned");
    };
    assert_eq!(s.line, 7);
}

#[test]
fn function_subgraph_without_owner_var_renders_anonymous() {
    let mut scope = base_serialized_scope("fn");
    scope.r#type = ScopeType::Function;
    let VisualSubgraph::Owned(s) = describe_subgraph(
        &scope,
        &HashMap::new(),
        &HashMap::new(),
        &unsnarl_ir::primitive::SourceIndex::build(""),
    ) else {
        panic!("expected owned");
    };
    assert!(matches!(s.kind, OwnedSubgraphKind::Function));
    let OwnedExtras::Function {
        owner_node_id,
        owner_name,
    } = s.extras
    else {
        panic!("expected Function extras");
    };
    assert_eq!(owner_node_id, None);
    assert_eq!(owner_name, "");
}

#[test]
fn control_subgraph_for() {
    let mut scope = base_serialized_scope("ctrl");
    scope.r#type = ScopeType::For;
    scope.block = block(AstType::BlockStatement, 0, 1, 10, 3);
    let VisualSubgraph::Control(s) = describe_subgraph(
        &scope,
        &HashMap::new(),
        &HashMap::new(),
        &unsnarl_ir::primitive::SourceIndex::build(""),
    ) else {
        panic!("expected control");
    };
    assert!(matches!(s.kind, ControlSubgraphKind::For));
    assert_eq!(s.id, "s_ctrl");
    assert_eq!(s.line, 1);
    assert_eq!(s.end_line, Some(3));
}

#[test]
fn control_subgraph_catch() {
    let mut scope = base_serialized_scope("ctrl");
    scope.r#type = ScopeType::Catch;
    scope.block = block(AstType::BlockStatement, 0, 1, 10, 3);
    let VisualSubgraph::Control(s) = describe_subgraph(
        &scope,
        &HashMap::new(),
        &HashMap::new(),
        &unsnarl_ir::primitive::SourceIndex::build(""),
    ) else {
        panic!("expected control");
    };
    assert!(matches!(s.kind, ControlSubgraphKind::Catch));
}

#[test]
fn control_subgraph_switch() {
    let mut scope = base_serialized_scope("ctrl");
    scope.r#type = ScopeType::Switch;
    scope.block = block(AstType::BlockStatement, 0, 1, 10, 3);
    let VisualSubgraph::Control(s) = describe_subgraph(
        &scope,
        &HashMap::new(),
        &HashMap::new(),
        &unsnarl_ir::primitive::SourceIndex::build(""),
    ) else {
        panic!("expected control");
    };
    assert!(matches!(s.kind, ControlSubgraphKind::Switch));
}

#[test]
fn case_subgraph_captures_case_test_from_block_context() {
    let mut scope = base_serialized_scope("case1");
    scope.r#type = ScopeType::Block;
    scope.block = block(AstType::BlockStatement, 0, 1, 10, 2);
    scope.block_context = Some(case_clause_block_context(
        AstType::SwitchStatement,
        "cases",
        0,
        Some("x === 1"),
    ));
    let VisualSubgraph::Control(s) = describe_subgraph(
        &scope,
        &HashMap::new(),
        &HashMap::new(),
        &unsnarl_ir::primitive::SourceIndex::build(""),
    ) else {
        panic!("expected control");
    };
    assert!(matches!(s.kind, ControlSubgraphKind::Case));
    let ControlExtras::Case { case_test } = &s.extras else {
        panic!("expected Case extras");
    };
    assert_eq!(case_test.as_deref(), Some("x === 1"));
}

#[test]
fn case_subgraph_keeps_case_test_null_when_default() {
    let mut scope = base_serialized_scope("case-default");
    scope.r#type = ScopeType::Block;
    scope.block_context = Some(case_clause_block_context(
        AstType::SwitchStatement,
        "cases",
        0,
        None,
    ));
    let VisualSubgraph::Control(s) = describe_subgraph(
        &scope,
        &HashMap::new(),
        &HashMap::new(),
        &unsnarl_ir::primitive::SourceIndex::build(""),
    ) else {
        panic!("expected control");
    };
    let ControlExtras::Case { case_test } = &s.extras else {
        panic!("expected Case extras");
    };
    assert_eq!(case_test.as_deref(), None);
}

#[test]
fn plain_block_scope_renders_as_generic_block() {
    let scope = base_serialized_scope("plain");
    let VisualSubgraph::Control(s) = describe_subgraph(
        &scope,
        &HashMap::new(),
        &HashMap::new(),
        &unsnarl_ir::primitive::SourceIndex::build(""),
    ) else {
        panic!("expected control");
    };
    assert!(matches!(s.kind, ControlSubgraphKind::Block));
}

#[test]
fn class_scope_with_class_name_binding_picks_inner_identifier() {
    let mut scope = base_serialized_scope("clsScope");
    scope.r#type = ScopeType::Class;
    scope.block = block(AstType::ClassExpression, 0, 2, 30, 4);
    scope.variables = vec![variable_id("innerNameVar")];
    let inner = SerializedVariable::new(
        variable_id("innerNameVar"),
        "Foo".to_string(),
        scope_id("s"),
        vec![span_offset_line(0, 2)],
        Vec::new(),
        Vec::new(),
    );
    let _ = base_serialized_variable; // suppress unused warning in scope-local imports
    let mut variables: HashMap<&str, &SerializedVariable> = HashMap::new();
    variables.insert("innerNameVar", &inner);

    let VisualSubgraph::Owned(s) = describe_subgraph(
        &scope,
        &HashMap::new(),
        &variables,
        &unsnarl_ir::primitive::SourceIndex::build(""),
    ) else {
        panic!("expected owned");
    };
    assert_eq!(s.id, "s_clsScope");
    assert!(matches!(s.kind, OwnedSubgraphKind::Class));
    assert!(matches!(s.direction, Direction::RL));
    assert_eq!(s.line, 2);
    assert_eq!(s.end_line, Some(4));
    let OwnedExtras::Class { class_name } = s.extras else {
        panic!("expected Class extras");
    };
    assert_eq!(class_name.as_deref(), Some("Foo"));
}

#[test]
fn class_scope_with_no_variables_yields_class_name_null() {
    let mut scope = base_serialized_scope("anon");
    scope.r#type = ScopeType::Class;
    scope.block = block(AstType::ClassExpression, 0, 1, 10, 1);
    scope.variables = Vec::new();
    let VisualSubgraph::Owned(s) = describe_subgraph(
        &scope,
        &HashMap::new(),
        &HashMap::new(),
        &unsnarl_ir::primitive::SourceIndex::build(""),
    ) else {
        panic!("expected owned");
    };
    let OwnedExtras::Class { class_name } = s.extras else {
        panic!("expected Class extras");
    };
    assert_eq!(class_name, None);
}

#[test]
#[should_panic]
fn panics_when_scope_is_module() {
    let mut scope = base_serialized_scope("mod");
    scope.r#type = ScopeType::Module;
    let _ = describe_subgraph(
        &scope,
        &HashMap::new(),
        &HashMap::new(),
        &unsnarl_ir::primitive::SourceIndex::build(""),
    );
}
