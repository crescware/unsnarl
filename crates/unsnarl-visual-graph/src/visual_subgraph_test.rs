//! Locks down the JSON field order produced by each of the two
//! `VisualSubgraph` shapes. The "owned" path puts `kind` directly
//! after `id` and `elements` last; the "control" path puts
//! `elements` in the middle and `kind` near the end.

use super::*;
use crate::direction::Direction;
use crate::visual_element_type::SubgraphTypeTag;

#[test]
fn function_subgraph_emits_kind_early_and_elements_last() {
    let sg = OwnedVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: "s_scope_1".to_string(),
        kind: OwnedSubgraphKind::Function,
        line: 7,
        end_line: Some(9),
        direction: Direction::RL,
        extras: OwnedExtras::Function {
            owner_node_id: Some("n_scope_0_compute_81".to_string()),
            owner_name: "compute".to_string(),
        },
        elements: Vec::new(),
    };
    let text = serde_json::to_string_pretty(&sg).expect("serialize");
    assert_eq!(
        text,
        r#"{
  "type": "subgraph",
  "id": "s_scope_1",
  "kind": "function",
  "line": 7,
  "endLine": 9,
  "direction": "RL",
  "ownerNodeId": "n_scope_0_compute_81",
  "ownerName": "compute",
  "elements": []
}"#
    );
}

#[test]
fn class_subgraph_carries_class_name_before_elements() {
    let sg = OwnedVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: "s_scope_2".to_string(),
        kind: OwnedSubgraphKind::Class,
        line: 1,
        end_line: Some(4),
        direction: Direction::RL,
        extras: OwnedExtras::Class {
            class_name: Some("Foo".to_string()),
        },
        elements: Vec::new(),
    };
    let value: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&sg).expect("serialize")).expect("json");
    assert_eq!(value["kind"], "class");
    assert_eq!(value["className"], "Foo");
}

#[test]
fn if_else_container_carries_has_else_before_elements() {
    let sg = OwnedVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: "cont_if_scope_3_107".to_string(),
        kind: OwnedSubgraphKind::IfElseContainer,
        line: 6,
        end_line: Some(10),
        direction: Direction::RL,
        extras: OwnedExtras::IfElseContainer { has_else: true },
        elements: Vec::new(),
    };
    let text = serde_json::to_string_pretty(&sg).expect("serialize");
    assert_eq!(
        text,
        r#"{
  "type": "subgraph",
  "id": "cont_if_scope_3_107",
  "kind": "if-else-container",
  "line": 6,
  "endLine": 10,
  "direction": "RL",
  "hasElse": true,
  "elements": []
}"#
    );
}

#[test]
fn return_subgraph_emits_no_extras_just_kind_then_elements() {
    let sg = OwnedVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: "s_return_xyz".to_string(),
        kind: OwnedSubgraphKind::Return,
        line: 15,
        end_line: None,
        direction: Direction::RL,
        extras: OwnedExtras::None {},
        elements: Vec::new(),
    };
    let text = serde_json::to_string_pretty(&sg).expect("serialize");
    assert_eq!(
        text,
        r#"{
  "type": "subgraph",
  "id": "s_return_xyz",
  "kind": "return",
  "line": 15,
  "endLine": null,
  "direction": "RL",
  "elements": []
}"#
    );
}

#[test]
fn control_subgraph_places_elements_before_kind() {
    let sg = ControlVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: "s_scope_3".to_string(),
        line: 5,
        end_line: Some(10),
        direction: Direction::RL,
        elements: Vec::new(),
        kind: ControlSubgraphKind::If,
        extras: ControlExtras::None {},
    };
    let text = serde_json::to_string_pretty(&sg).expect("serialize");
    assert_eq!(
        text,
        r#"{
  "type": "subgraph",
  "id": "s_scope_3",
  "line": 5,
  "endLine": 10,
  "direction": "RL",
  "elements": [],
  "kind": "if"
}"#
    );
}

#[test]
fn case_subgraph_emits_case_test_after_kind() {
    let sg = ControlVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: "s_scope_4".to_string(),
        line: 4,
        end_line: Some(6),
        direction: Direction::RL,
        elements: Vec::new(),
        kind: ControlSubgraphKind::Case,
        extras: ControlExtras::Case {
            case_test: Some("\"a\"".to_string()),
        },
    };
    let text = serde_json::to_string_pretty(&sg).expect("serialize");
    assert_eq!(
        text,
        r#"{
  "type": "subgraph",
  "id": "s_scope_4",
  "line": 4,
  "endLine": 6,
  "direction": "RL",
  "elements": [],
  "kind": "case",
  "caseTest": "\"a\""
}"#
    );
}

#[test]
fn default_case_subgraph_emits_case_test_null() {
    let sg = ControlVisualSubgraph {
        r#type: SubgraphTypeTag::Subgraph,
        id: "s_default".to_string(),
        line: 9,
        end_line: None,
        direction: Direction::RL,
        elements: Vec::new(),
        kind: ControlSubgraphKind::Case,
        extras: ControlExtras::Case { case_test: None },
    };
    let text = serde_json::to_string(&sg).expect("serialize");
    assert!(
        text.contains(r#""caseTest":null"#),
        "expected null caseTest, got: {text}"
    );
}
