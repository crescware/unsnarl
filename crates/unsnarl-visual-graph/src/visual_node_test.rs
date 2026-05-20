//! Locks down the JSON field order produced by each of the two
//! `VisualNode` shapes. The order matches the JS object-literal
//! order at the corresponding TS construction sites
//! (`makeVariableNode` for the binding shape, the various
//! `ensure-*` / anchor builders for the synthetic shape) — bytes
//! that the IR parity harness compares against `expected.json`.

use unsnarl_oxc_parity::VariableDeclarationKind;

use super::*;
use crate::visual_element::VisualElement;

#[test]
fn binding_shape_orders_kind_after_common_fields() {
    let node = BindingVisualNode {
        r#type: NodeTypeTag::Node,
        id: "n_scope_0_a_15".to_string(),
        name: "a".to_string(),
        line: 2,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        kind: BindingNodeKind::ConstBinding,
        extras: BindingExtras::Variable {
            init_is_function: false,
        },
    };
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
    let node = BindingVisualNode {
        r#type: NodeTypeTag::Node,
        id: "n_scope_0_compute_81".to_string(),
        name: "compute".to_string(),
        line: 7,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        kind: BindingNodeKind::FunctionDeclaration,
        extras: BindingExtras::None {},
    };
    let value: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&node).expect("serialize")).expect("json");
    assert_eq!(value["kind"], "FunctionDeclaration");
    assert!(value.get("initIsFunction").is_none());
    assert!(value.get("importedName").is_none());
}

#[test]
fn binding_shape_named_import_carries_imported_name() {
    let node = BindingVisualNode {
        r#type: NodeTypeTag::Node,
        id: "n_scope_0_renamed_0".to_string(),
        name: "renamed".to_string(),
        line: 1,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        kind: BindingNodeKind::NamedImportBinding,
        extras: BindingExtras::NamedImport {
            imported_name: "other".to_string(),
        },
    };
    let value: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&node).expect("serialize")).expect("json");
    assert_eq!(value["importedName"], "other");
}

#[test]
fn synthetic_shape_orders_kind_immediately_after_id() {
    let node = SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id: "expr_stmt_571".to_string(),
        kind: SyntheticNodeKind::SyntheticExpressionStatement,
        name: "console.log()".to_string(),
        line: 23,
        end_line: Some(41),
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::None {},
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
        r#type: NodeTypeTag::Node,
        id: "wr_ref_3".to_string(),
        kind: SyntheticNodeKind::WriteReference,
        name: "label".to_string(),
        line: 5,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::WriteOp {
            declaration_kind: Some(VariableDeclarationKind::Let),
        },
    };
    let value: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&node).expect("serialize")).expect("json");
    assert_eq!(value["kind"], "WriteReference");
    assert_eq!(value["declarationKind"], "let");
}

#[test]
fn untagged_visual_node_enum_round_trips_via_serde_json() {
    let element = VisualElement::Node(VisualNode::Binding(BindingVisualNode {
        r#type: NodeTypeTag::Node,
        id: "n_x".to_string(),
        name: "x".to_string(),
        line: 1,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        kind: BindingNodeKind::ConstBinding,
        extras: BindingExtras::Variable {
            init_is_function: false,
        },
    }));
    let value: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&element).expect("serialize")).expect("json");
    assert_eq!(value["type"], "node");
    assert_eq!(value["kind"], "ConstBinding");
}

#[test]
fn end_line_serializes_as_null_when_none() {
    let element = VisualElement::Node(VisualNode::Synthetic(SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id: "if_test_scope_3_107".to_string(),
        kind: SyntheticNodeKind::SyntheticIfStatementTest,
        name: "if-test".to_string(),
        line: 6,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::None {},
    }));
    let text = serde_json::to_string(&element).expect("serialize");
    assert!(
        text.contains(r#""endLine":null"#),
        "expected null endLine, got: {text}"
    );
}

#[test]
fn empty_extras_dict_does_not_appear_in_output() {
    let element = VisualElement::Node(VisualNode::Binding(BindingVisualNode {
        r#type: NodeTypeTag::Node,
        id: "n_param".to_string(),
        name: "p".to_string(),
        line: 1,
        end_line: None,
        is_jsx_element: false,
        unused: false,
        kind: BindingNodeKind::FormalParameter,
        extras: BindingExtras::None {},
    }));
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
