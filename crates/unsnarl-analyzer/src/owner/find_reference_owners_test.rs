use unsnarl_oxc_parity::AstType;

use crate::analyzer_fixtures::{ast_node, ast_node_with_end};
use crate::path_entry::PathEntry;

use super::{locate_reference_owner_slot, OwnerLookup};

#[test]
fn empty_path_returns_none() {
    let lookup = locate_reference_owner_slot(&[]);
    assert!(matches!(lookup, OwnerLookup::None));
}

#[test]
fn variable_declarator_on_path_is_reported() {
    let path = vec![
        PathEntry::new(ast_node(AstType::Program, 0), None),
        PathEntry::new(
            ast_node_with_end(AstType::VariableDeclaration, 0, 20),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::VariableDeclarator, 4, 18),
            Some("declarations"),
        ),
        PathEntry::new(ast_node_with_end(AstType::Identifier, 4, 5), Some("id")),
    ];
    let lookup = locate_reference_owner_slot(&path);
    let idx = match lookup {
        OwnerLookup::VariableDeclarator { path_index } => path_index,
        _ => panic!("expected VariableDeclarator owner slot"),
    };
    assert_eq!(idx, 2);
}

#[test]
fn assignment_expression_on_path_is_reported() {
    let path = vec![
        PathEntry::new(ast_node(AstType::Program, 0), None),
        PathEntry::new(
            ast_node_with_end(AstType::ExpressionStatement, 0, 10),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::AssignmentExpression, 0, 9),
            Some("expression"),
        ),
        PathEntry::new(ast_node_with_end(AstType::Identifier, 0, 1), Some("left")),
    ];
    let lookup = locate_reference_owner_slot(&path);
    let idx = match lookup {
        OwnerLookup::AssignmentExpression { path_index } => path_index,
        _ => panic!("expected AssignmentExpression owner slot"),
    };
    assert_eq!(idx, 2);
}

#[test]
fn function_declaration_boundary_stops_search() {
    let path = vec![
        PathEntry::new(ast_node(AstType::Program, 0), None),
        PathEntry::new(
            ast_node_with_end(AstType::FunctionDeclaration, 0, 50),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::BlockStatement, 12, 50),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::Identifier, 13, 16),
            Some("expression"),
        ),
    ];
    let lookup = locate_reference_owner_slot(&path);
    assert!(matches!(lookup, OwnerLookup::Boundary));
}

#[test]
fn arrow_function_expression_boundary_stops_search() {
    let path = vec![
        PathEntry::new(ast_node(AstType::Program, 0), None),
        PathEntry::new(
            ast_node_with_end(AstType::ArrowFunctionExpression, 0, 50),
            None,
        ),
        PathEntry::new(ast_node_with_end(AstType::Identifier, 5, 8), Some("body")),
    ];
    let lookup = locate_reference_owner_slot(&path);
    assert!(matches!(lookup, OwnerLookup::Boundary));
}

#[test]
fn class_declaration_boundary_stops_search() {
    let path = vec![
        PathEntry::new(ast_node(AstType::Program, 0), None),
        PathEntry::new(
            ast_node_with_end(AstType::ClassDeclaration, 0, 50),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::PropertyDefinition, 10, 40),
            Some("body"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::Identifier, 15, 18),
            Some("value"),
        ),
    ];
    let lookup = locate_reference_owner_slot(&path);
    assert!(matches!(lookup, OwnerLookup::Boundary));
}

#[test]
fn inner_boundary_wins_over_outer_variable_declarator() {
    // The reference sits inside an inner arrow function, which itself
    // sits inside a VariableDeclarator's init. The boundary stops the
    // search before the declarator is reached.
    let path = vec![
        PathEntry::new(ast_node(AstType::Program, 0), None),
        PathEntry::new(
            ast_node_with_end(AstType::VariableDeclarator, 0, 50),
            Some("declarations"),
        ),
        PathEntry::new(
            ast_node_with_end(AstType::ArrowFunctionExpression, 10, 50),
            Some("init"),
        ),
        PathEntry::new(ast_node_with_end(AstType::Identifier, 25, 30), Some("body")),
    ];
    let lookup = locate_reference_owner_slot(&path);
    assert!(matches!(lookup, OwnerLookup::Boundary));
}

#[test]
fn path_without_interesting_entries_returns_none() {
    let path = vec![
        PathEntry::new(ast_node(AstType::Program, 0), None),
        PathEntry::new(ast_node_with_end(AstType::IfStatement, 0, 30), Some("body")),
        PathEntry::new(ast_node_with_end(AstType::Identifier, 5, 8), Some("test")),
    ];
    let lookup = locate_reference_owner_slot(&path);
    assert!(matches!(lookup, OwnerLookup::None));
}

#[test]
fn nearest_variable_declarator_wins_over_outer_assignment() {
    // Inside the rhs of an outer AssignmentExpression sits a
    // VariableDeclarator (rare in real code but possible via
    // intermediate constructs). The nearest entry wins.
    let path = vec![
        PathEntry::new(ast_node(AstType::Program, 0), None),
        PathEntry::new(
            ast_node_with_end(AstType::AssignmentExpression, 0, 50),
            None,
        ),
        PathEntry::new(
            ast_node_with_end(AstType::VariableDeclarator, 10, 40),
            Some("right"),
        ),
        PathEntry::new(ast_node_with_end(AstType::Identifier, 14, 18), Some("id")),
    ];
    let lookup = locate_reference_owner_slot(&path);
    let idx = match lookup {
        OwnerLookup::VariableDeclarator { path_index } => path_index,
        _ => panic!("expected VariableDeclarator owner slot"),
    };
    assert_eq!(idx, 2);
}
