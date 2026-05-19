use unsnarl_ir::reference::PredicateContainer;
use unsnarl_ir::SourceOffset;
use unsnarl_oxc_parity::{AstType, PredicateContainerType};

use crate::testing::{ast_node, entry};

use super::find_predicate_container;

fn expect_container(
    actual: Option<PredicateContainer>,
    expected_type: PredicateContainerType,
    expected_offset: u32,
) {
    let PredicateContainer { r#type, offset } = actual.expect("expected a predicate container");
    assert!(
        std::mem::discriminant(&r#type) == std::mem::discriminant(&expected_type),
        "predicate container type mismatch"
    );
    assert_eq!(offset, SourceOffset(expected_offset));
}

#[test]
fn returns_none_for_empty_path_and_null_parent() {
    let res = find_predicate_container(None, None, None, &[]);
    assert!(res.is_none());
}

#[test]
fn matches_single_if_test_reference_returns_if_offset() {
    let if_stmt = ast_node(AstType::IfStatement, 10);
    let test_expr = ast_node(AstType::BinaryExpression, 14);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(if_stmt, Some("body")),
        entry(test_expr, Some("test")),
    ];
    let res = find_predicate_container(
        Some(&AstType::BinaryExpression),
        Some(14),
        Some("left"),
        &path,
    );
    expect_container(res, PredicateContainerType::IfStatement, 10);
}

#[test]
fn inner_if_offset_for_else_if_test_reference() {
    let outer = ast_node(AstType::IfStatement, 10);
    let inner = ast_node(AstType::IfStatement, 40);
    let inner_test = ast_node(AstType::BinaryExpression, 44);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(outer, Some("body")),
        entry(inner, Some("alternate")),
        entry(inner_test, Some("test")),
    ];
    let res = find_predicate_container(
        Some(&AstType::BinaryExpression),
        Some(44),
        Some("left"),
        &path,
    );
    expect_container(res, PredicateContainerType::IfStatement, 40);
}

#[test]
fn matches_switch_discriminant_reference_returns_switch_offset() {
    let switch_stmt = ast_node(AstType::SwitchStatement, 10);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(switch_stmt, Some("body")),
    ];
    let res = find_predicate_container(
        Some(&AstType::SwitchStatement),
        Some(10),
        Some("discriminant"),
        &path,
    );
    expect_container(res, PredicateContainerType::SwitchStatement, 10);
}

#[test]
fn returns_none_for_reference_outside_any_test_or_discriminant() {
    let if_stmt = ast_node(AstType::IfStatement, 10);
    let consequent = ast_node(AstType::BlockStatement, 20);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(if_stmt, Some("body")),
        entry(consequent, Some("consequent")),
    ];
    let res = find_predicate_container(
        Some(&AstType::BlockStatement),
        Some(20),
        Some("body"),
        &path,
    );
    assert!(res.is_none());
}

#[test]
fn falls_back_to_parent_start_when_path_is_empty_for_if() {
    let res = find_predicate_container(Some(&AstType::IfStatement), Some(99), Some("test"), &[]);
    expect_container(res, PredicateContainerType::IfStatement, 99);
}

#[test]
fn matches_while_test_reference_returns_while_offset() {
    let while_stmt = ast_node(AstType::WhileStatement, 20);
    let test_expr = ast_node(AstType::BinaryExpression, 27);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(while_stmt, Some("body")),
        entry(test_expr, Some("test")),
    ];
    let res = find_predicate_container(
        Some(&AstType::BinaryExpression),
        Some(27),
        Some("left"),
        &path,
    );
    expect_container(res, PredicateContainerType::WhileStatement, 20);
}

#[test]
fn while_only_matches_when_cur_key_is_test_not_body() {
    let while_stmt = ast_node(AstType::WhileStatement, 20);
    let in_body = ast_node(AstType::BlockStatement, 40);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(while_stmt, Some("body")),
        entry(in_body, Some("body")),
    ];
    let res = find_predicate_container(
        Some(&AstType::BlockStatement),
        Some(40),
        Some("expression"),
        &path,
    );
    assert!(res.is_none());
}

#[test]
fn matches_do_while_test_reference_returns_do_while_offset() {
    let do_stmt = ast_node(AstType::DoWhileStatement, 30);
    let test_expr = ast_node(AstType::BinaryExpression, 60);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(do_stmt, Some("body")),
        entry(test_expr, Some("test")),
    ];
    let res = find_predicate_container(
        Some(&AstType::BinaryExpression),
        Some(60),
        Some("left"),
        &path,
    );
    expect_container(res, PredicateContainerType::DoWhileStatement, 30);
}

#[test]
fn for_statement_matches_when_cur_key_resolves_to_init_test_or_update() {
    for key in ["init", "test", "update"] {
        let for_stmt = ast_node(AstType::ForStatement, 40);
        let in_header = ast_node(AstType::BinaryExpression, 45);
        let path = vec![
            entry(ast_node(AstType::Program, 0), None),
            entry(for_stmt, Some("body")),
            entry(in_header, Some(key)),
        ];
        let res = find_predicate_container(
            Some(&AstType::BinaryExpression),
            Some(45),
            Some("left"),
            &path,
        );
        expect_container(res, PredicateContainerType::ForStatement, 40);
    }
}

#[test]
fn for_statement_does_not_match_when_cur_key_resolves_to_non_header_slot() {
    let for_stmt = ast_node(AstType::ForStatement, 40);
    let in_body = ast_node(AstType::BlockStatement, 60);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(for_stmt, Some("body")),
        entry(in_body, Some("body")),
    ];
    let res = find_predicate_container(
        Some(&AstType::BlockStatement),
        Some(60),
        Some("expression"),
        &path,
    );
    assert!(res.is_none());
}

#[test]
fn for_of_statement_matches_when_cur_key_resolves_to_left_or_right() {
    for key in ["left", "right"] {
        let for_of = ast_node(AstType::ForOfStatement, 50);
        let in_header = ast_node(AstType::BinaryExpression, 58);
        let path = vec![
            entry(ast_node(AstType::Program, 0), None),
            entry(for_of, Some("body")),
            entry(in_header, Some(key)),
        ];
        let res = find_predicate_container(
            Some(&AstType::BinaryExpression),
            Some(58),
            Some("name"),
            &path,
        );
        expect_container(res, PredicateContainerType::ForOfStatement, 50);
    }
}

#[test]
fn for_in_statement_matches_when_cur_key_resolves_to_left_or_right() {
    for key in ["left", "right"] {
        let for_in = ast_node(AstType::ForInStatement, 70);
        let in_header = ast_node(AstType::BinaryExpression, 78);
        let path = vec![
            entry(ast_node(AstType::Program, 0), None),
            entry(for_in, Some("body")),
            entry(in_header, Some(key)),
        ];
        let res = find_predicate_container(
            Some(&AstType::BinaryExpression),
            Some(78),
            Some("name"),
            &path,
        );
        expect_container(res, PredicateContainerType::ForInStatement, 70);
    }
}

#[test]
fn for_of_does_not_match_when_cur_key_resolves_to_body() {
    let for_of = ast_node(AstType::ForOfStatement, 50);
    let in_body = ast_node(AstType::BlockStatement, 80);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(for_of, Some("body")),
        entry(in_body, Some("body")),
    ];
    let res = find_predicate_container(
        Some(&AstType::BlockStatement),
        Some(80),
        Some("expression"),
        &path,
    );
    assert!(res.is_none());
}

#[test]
fn falls_back_to_parent_start_for_while_test_when_path_empty() {
    let res = find_predicate_container(Some(&AstType::WhileStatement), Some(88), Some("test"), &[]);
    expect_container(res, PredicateContainerType::WhileStatement, 88);
}

#[test]
fn falls_back_to_parent_start_for_do_while_test_when_path_empty() {
    let res = find_predicate_container(
        Some(&AstType::DoWhileStatement),
        Some(77),
        Some("test"),
        &[],
    );
    expect_container(res, PredicateContainerType::DoWhileStatement, 77);
}

#[test]
fn falls_back_to_parent_start_for_for_statement_when_path_empty() {
    for key in ["init", "test", "update"] {
        let res = find_predicate_container(Some(&AstType::ForStatement), Some(55), Some(key), &[]);
        expect_container(res, PredicateContainerType::ForStatement, 55);
    }
}

#[test]
fn falls_back_to_parent_start_for_for_of_when_path_empty() {
    for key in ["left", "right"] {
        let res =
            find_predicate_container(Some(&AstType::ForOfStatement), Some(66), Some(key), &[]);
        expect_container(res, PredicateContainerType::ForOfStatement, 66);
    }
}

#[test]
fn falls_back_to_parent_start_for_for_in_when_path_empty() {
    for key in ["left", "right"] {
        let res =
            find_predicate_container(Some(&AstType::ForInStatement), Some(77), Some(key), &[]);
        expect_container(res, PredicateContainerType::ForInStatement, 77);
    }
}
