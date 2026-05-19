use unsnarl_oxc_parity::AstType;

use crate::testing::{ast_node, ast_node_with_end, entry};

use super::if_chain_root_offset;

#[test]
fn returns_none_when_parent_is_null() {
    assert_eq!(if_chain_root_offset(None, Some("consequent"), &[]), None);
}

#[test]
fn returns_none_when_parent_is_not_an_if_statement() {
    let block = ast_node(AstType::BlockStatement, 0);
    assert_eq!(
        if_chain_root_offset(Some(&block), Some("consequent"), &[]),
        None
    );
}

#[test]
fn returns_none_for_keys_other_than_consequent_or_alternate() {
    let parent = ast_node(AstType::IfStatement, 10);
    assert_eq!(if_chain_root_offset(Some(&parent), Some("test"), &[]), None);
}

#[test]
fn returns_none_for_standalone_if() {
    let parent = ast_node_with_end(AstType::IfStatement, 10, 30);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(parent.clone(), Some("body")),
    ];
    assert_eq!(
        if_chain_root_offset(Some(&parent), Some("consequent"), &path),
        None
    );
}

#[test]
fn returns_outer_offset_for_inner_if_consequent_one_step() {
    let outer = ast_node_with_end(AstType::IfStatement, 10, 100);
    let inner = ast_node_with_end(AstType::IfStatement, 40, 80);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(outer, Some("body")),
        entry(inner.clone(), Some("alternate")),
    ];
    assert_eq!(
        if_chain_root_offset(Some(&inner), Some("consequent"), &path),
        Some(10)
    );
}

#[test]
fn returns_outer_offset_for_inner_if_alternate_one_step() {
    let outer = ast_node_with_end(AstType::IfStatement, 10, 100);
    let inner = ast_node_with_end(AstType::IfStatement, 40, 80);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(outer, Some("body")),
        entry(inner.clone(), Some("alternate")),
    ];
    assert_eq!(
        if_chain_root_offset(Some(&inner), Some("alternate"), &path),
        Some(10)
    );
}

#[test]
fn walks_back_through_multiple_chained_alternates() {
    let outermost = ast_node_with_end(AstType::IfStatement, 5, 200);
    let middle = ast_node_with_end(AstType::IfStatement, 40, 180);
    let innermost = ast_node_with_end(AstType::IfStatement, 80, 150);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(outermost, Some("body")),
        entry(middle, Some("alternate")),
        entry(innermost.clone(), Some("alternate")),
    ];
    assert_eq!(
        if_chain_root_offset(Some(&innermost), Some("consequent"), &path),
        Some(5)
    );
}

#[test]
fn does_not_walk_past_a_non_if_statement_ancestor() {
    let outer = ast_node_with_end(AstType::IfStatement, 10, 200);
    let inner_block = ast_node_with_end(AstType::BlockStatement, 40, 100);
    let inner = ast_node_with_end(AstType::IfStatement, 45, 90);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(outer, Some("body")),
        entry(inner_block, Some("alternate")),
        entry(inner.clone(), Some("body")),
    ];
    assert_eq!(
        if_chain_root_offset(Some(&inner), Some("consequent"), &path),
        None
    );
}

#[test]
fn does_not_walk_when_current_if_sits_in_consequent_slot_of_outer() {
    let outer = ast_node_with_end(AstType::IfStatement, 10, 200);
    let cons_block = ast_node_with_end(AstType::BlockStatement, 20, 90);
    let inner = ast_node_with_end(AstType::IfStatement, 25, 70);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(outer, Some("body")),
        entry(cons_block, Some("consequent")),
        entry(inner.clone(), Some("body")),
    ];
    assert_eq!(
        if_chain_root_offset(Some(&inner), Some("consequent"), &path),
        None
    );
}

#[test]
fn falls_back_to_zero_when_chain_top_has_zero_start() {
    let outer = ast_node_with_end(AstType::IfStatement, 0, 100);
    let inner = ast_node_with_end(AstType::IfStatement, 40, 80);
    let path = vec![
        entry(outer, Some("body")),
        entry(inner.clone(), Some("alternate")),
    ];
    assert_eq!(
        if_chain_root_offset(Some(&inner), Some("consequent"), &path),
        Some(0)
    );
}
