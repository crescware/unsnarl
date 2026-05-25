use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::scope::BlockContext;
use unsnarl_oxc_parity::AstType;

use crate::path_entry::PathEntry;
use crate::testing::ast_node;

use super::block_context_of;

fn serialize(ctx: &BlockContext) -> serde_json::Value {
    serde_json::to_value(ctx).unwrap()
}

#[test]
fn returns_none_when_parent_is_null() {
    assert!(block_context_of(None, Some("body"), &[], &SourceIndex::build("")).is_none());
}

#[test]
fn returns_none_when_key_is_null() {
    let parent = ast_node(AstType::IfStatement, 5);
    assert!(block_context_of(Some(&parent), None, &[], &SourceIndex::build("")).is_none());
}

#[test]
fn returns_parent_type_key_and_start_as_parent_span_offset() {
    let parent = ast_node(AstType::IfStatement, 12);
    // 14 bytes of ASCII so byte 12 is in-range and UTF-16 conversion
    // is a no-op for this position.
    let raw = "let x = 1; if";
    let ctx = block_context_of(
        Some(&parent),
        Some("consequent"),
        &[],
        &SourceIndex::build(raw),
    )
    .expect("Some");
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
    let ctx =
        block_context_of(Some(&parent), Some("body"), &[], &SourceIndex::build("")).expect("Some");
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
        PathEntry::new(ast_node(AstType::Program, 0), None),
        PathEntry::new(outer, Some("body")),
        PathEntry::new(inner.clone(), Some("alternate")),
    ];
    // Pad to >= 100 bytes of ASCII so the byte offsets pre-converted
    // through `span_from_offset` are in-range and map 1:1 to UTF-16.
    let raw = " ".repeat(100);
    let ctx = block_context_of(
        Some(&inner),
        Some("consequent"),
        &path,
        &SourceIndex::build(&raw),
    )
    .expect("Some");
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

#[test]
fn parent_span_offset_is_in_utf16_code_units_when_source_contains_non_ascii() {
    // `parent.span.start` is a UTF-8 byte offset from `oxc_parser`. A
    // leading em-dash (`—`, 3 UTF-8 bytes / 1 UTF-16 code unit) shifts
    // the byte position of a token below it by +2 (3 - 1) vs UTF-16.
    // `if` starts at byte 7 (chars: `/`, `/`, ` `, `—`, `\n`, then
    // `if`); in UTF-16 the equivalent code-unit offset is 5. The
    // serialised `parentSpanOffset` must use the UTF-16 value per
    // the IR contract.
    let raw = "// —\nif (true) {}\n";
    let parent = crate::testing::ast_node(AstType::IfStatement, 7);
    let ctx = block_context_of(
        Some(&parent),
        Some("consequent"),
        &[],
        &SourceIndex::build(raw),
    )
    .expect("Some");
    assert_eq!(
        serialize(&ctx),
        serde_json::json!({
            "kind": "other",
            "parentType": "IfStatement",
            "key": "consequent",
            "parentSpanOffset": 5,
            "ifChainRootOffset": null
        })
    );
}

#[test]
fn if_chain_root_offset_is_in_utf16_code_units_when_source_contains_non_ascii() {
    use crate::testing::ast_node_with_end;
    // Same trick: shift everything past a leading em-dash so the
    // byte/UTF-16 difference is observable on both `parentSpanOffset`
    // and `ifChainRootOffset`.
    let raw = "// —\nif (a) {} else if (b) {} else {}\n";
    // Outer IfStatement at byte 7 (UTF-16 offset 5, after the
    // em-dash). Inner IfStatement nested in the `alternate` slot at
    // byte 22 (UTF-16 offset 20).
    let outer = ast_node_with_end(AstType::IfStatement, 7, 200);
    let inner = ast_node_with_end(AstType::IfStatement, 22, 100);
    let path = vec![
        PathEntry::new(crate::testing::ast_node(AstType::Program, 0), None),
        PathEntry::new(outer, Some("body")),
        PathEntry::new(inner.clone(), Some("alternate")),
    ];
    let ctx = block_context_of(
        Some(&inner),
        Some("consequent"),
        &path,
        &SourceIndex::build(raw),
    )
    .expect("Some");
    let v = serialize(&ctx);
    assert_eq!(v["parentSpanOffset"], serde_json::json!(20));
    assert_eq!(v["ifChainRootOffset"], serde_json::json!(5));
}
