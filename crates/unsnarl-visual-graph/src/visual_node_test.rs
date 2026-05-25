//! Locks down the JSON field order produced by each of the two
//! `VisualNode` shapes (the binding shape from `make_variable_node`
//! and the synthetic shape from the various `ensure_*` / anchor
//! builders) — bytes that the IR parity harness compares against
//! `expected.json`.

use unsnarl_oxc_parity::VariableDeclarationKind;

use super::*;
use crate::visual_element::VisualElement;

#[test]
fn binding_shape_orders_kind_after_common_fields() {
    let node = BindingVisualNode::const_binding("n_scope_0_a_15", "a", 2);
    let text = serde_json::to_string_pretty(&node).expect("serialize");
    assert_eq!(
        text,
        r#"{
  "type": "node",
  "id": "n_scope_0_a_15",
  "name": "a",
  "line": 2,
  "endLine": null,
  "isJsxElement": false,
  "unused": false,
  "kind": "ConstBinding",
  "initIsFunction": false
}"#
    );
}

#[test]
fn binding_shape_with_no_extras_emits_just_kind() {
    let node = BindingVisualNode::function_declaration("n_scope_0_compute_81", "compute", 7);
    let value: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&node).expect("serialize")).expect("json");
    assert_eq!(value["kind"], "FunctionDeclaration");
    assert!(value.get("initIsFunction").is_none());
    assert!(value.get("importedName").is_none());
}

#[test]
fn binding_shape_named_import_carries_imported_name() {
    let node =
        BindingVisualNode::named_import_binding("n_scope_0_renamed_0", "renamed", "other", 1);
    let value: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&node).expect("serialize")).expect("json");
    assert_eq!(value["importedName"], "other");
}

#[test]
fn synthetic_shape_orders_kind_immediately_after_id() {
    let node = SyntheticVisualNode {
        end_line: Some(41),
        ..SyntheticVisualNode::expression_statement("expr_stmt_571", "console.log()", 23)
    };
    let text = serde_json::to_string_pretty(&node).expect("serialize");
    assert_eq!(
        text,
        r#"{
  "type": "node",
  "id": "expr_stmt_571",
  "kind": "SyntheticExpressionStatement",
  "name": "console.log()",
  "line": 23,
  "endLine": 41,
  "isJsxElement": false,
  "unused": false
}"#
    );
}

#[test]
fn synthetic_write_op_carries_declaration_kind_after_unused() {
    let node = SyntheticVisualNode {
        extras: SyntheticExtras::WriteOp {
            declaration_kind: Some(VariableDeclarationKind::Let),
        },
        ..SyntheticVisualNode::write_reference("wr_ref_3", "label", 5)
    };
    let value: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&node).expect("serialize")).expect("json");
    assert_eq!(value["kind"], "WriteReference");
    assert_eq!(value["declarationKind"], "let");
}

#[test]
fn untagged_visual_node_enum_round_trips_via_serde_json() {
    let element: VisualElement =
        VisualNode::from(BindingVisualNode::const_binding("n_x", "x", 1)).into();
    let value: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&element).expect("serialize")).expect("json");
    assert_eq!(value["type"], "node");
    assert_eq!(value["kind"], "ConstBinding");
}

#[test]
fn end_line_serializes_as_null_when_none() {
    let element: VisualElement = VisualNode::from(SyntheticVisualNode::if_statement_test(
        "if_test_scope_3_107",
        6,
    ))
    .into();
    let text = serde_json::to_string(&element).expect("serialize");
    assert!(
        text.contains(r#""endLine":null"#),
        "expected null endLine, got: {text}"
    );
}

#[test]
fn empty_extras_dict_does_not_appear_in_output() {
    let element: VisualElement =
        VisualNode::from(BindingVisualNode::formal_parameter("n_param", "p", 1)).into();
    let text = serde_json::to_string(&element).expect("serialize");
    assert!(
        !text.contains("{}"),
        "untagged None extras should not surface as a nested empty object: {text}"
    );
    assert!(
        !text.contains("\"None\""),
        "untagged None extras should not surface the variant tag: {text}"
    );
}
