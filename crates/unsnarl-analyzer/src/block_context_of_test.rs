use unsnarl_ir::scope::BlockContext;
use unsnarl_oxc_parity::AstType;

use crate::testing::{ast_node, entry};

use super::block_context_of;

fn serialize(ctx: &BlockContext) -> serde_json::Value {
    serde_json::to_value(ctx).unwrap()
}

#[test]
fn returns_none_when_parent_is_null() {
    assert!(block_context_of(None, Some("body"), &[]).is_none());
}

#[test]
fn returns_none_when_key_is_null() {
    let parent = ast_node(AstType::IfStatement, 5);
    assert!(block_context_of(Some(&parent), None, &[]).is_none());
}

#[test]
fn returns_parent_type_key_and_start_as_parent_span_offset() {
    let parent = ast_node(AstType::IfStatement, 12);
    let ctx = block_context_of(Some(&parent), Some("consequent"), &[]).expect("Some");
    assert_eq!(
        serialize(&ctx),
        serde_json::json!({
            "kind": "other",
            "parentType": "IfStatement",
            "key": "consequent",
            "parentSpanOffset": 12,
            "ifChainRootOffset": null
        })
    );
}

#[test]
fn falls_back_to_parent_span_offset_zero_when_start_is_zero() {
    let parent = ast_node(AstType::Program, 0);
    let ctx = block_context_of(Some(&parent), Some("body"), &[]).expect("Some");
    assert_eq!(
        serialize(&ctx),
        serde_json::json!({
            "kind": "other",
            "parentType": "Program",
            "key": "body",
            "parentSpanOffset": 0,
            "ifChainRootOffset": null
        })
    );
}

#[test]
fn includes_if_chain_root_offset_when_path_indicates_else_if_chain() {
    use crate::testing::ast_node_with_end;
    let outer = ast_node_with_end(AstType::IfStatement, 5, 100);
    let inner = ast_node_with_end(AstType::IfStatement, 40, 80);
    let path = vec![
        entry(ast_node(AstType::Program, 0), None),
        entry(outer, Some("body")),
        entry(inner.clone(), Some("alternate")),
    ];
    let ctx = block_context_of(Some(&inner), Some("consequent"), &path).expect("Some");
    assert_eq!(
        serialize(&ctx),
        serde_json::json!({
            "kind": "other",
            "parentType": "IfStatement",
            "key": "consequent",
            "parentSpanOffset": 40,
            "ifChainRootOffset": 5
        })
    );
}
