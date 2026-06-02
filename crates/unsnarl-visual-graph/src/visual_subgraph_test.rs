//! Locks down the JSON field order produced by each of the two
//! `VisualSubgraph` shapes. The "owned" path puts `kind` directly
//! after `id` and `elements` last; the "control" path puts
//! `elements` in the middle and `kind` near the end.

use super::*;
use crate::direction::Direction;

#[test]
fn function_subgraph_emits_kind_early_and_elements_last() {
    let sg = OwnedVisualSubgraph {
        end_line: Some(9),
        ..OwnedVisualSubgraph::function(
            "s_scope_1",
            7,
            Some("n_scope_0_compute_81".to_string()),
            "compute",
            Vec::new(),
            Direction::RL,
        )
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
fn module_subgraph_carries_module_source_before_elements() {
    let sg = OwnedVisualSubgraph::module("sg_m", 1, "./utils/helper", Vec::new(), Direction::RL);
    let text = serde_json::to_string_pretty(&sg).expect("serialize");
    assert_eq!(
        text,
        r#"{
  "type": "subgraph",
  "id": "sg_m",
  "kind": "module",
  "line": 1,
  "endLine": null,
  "direction": "RL",
  "moduleSource": "./utils/helper",
  "elements": []
}"#
    );
    let sg: VisualSubgraph = sg.into();
    assert_eq!(sg.module_source(), Some("./utils/helper"));
    assert!(matches!(
        sg.kind(),
        crate::subgraph_kind::SubgraphKind::Module
    ));
}

#[test]
fn class_subgraph_carries_class_name_before_elements() {
    let sg = OwnedVisualSubgraph {
        end_line: Some(4),
        ..OwnedVisualSubgraph::class(
            "s_scope_2",
            1,
            Some("Foo".to_string()),
            Vec::new(),
            Direction::RL,
        )
    };
    let value: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&sg).expect("serialize")).expect("json");
    assert_eq!(value["kind"], "class");
    assert_eq!(value["className"], "Foo");
}

#[test]
fn if_else_container_carries_has_else_before_elements() {
    let sg = OwnedVisualSubgraph {
        end_line: Some(10),
        ..OwnedVisualSubgraph::if_else_container(
            "cont_if_scope_3_107",
            6,
            true,
            Vec::new(),
            Direction::RL,
        )
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
    let sg = OwnedVisualSubgraph::return_subgraph("s_return_xyz", 15, Vec::new(), Direction::RL);
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
        end_line: Some(10),
        ..ControlVisualSubgraph::if_subgraph("s_scope_3", 5, Vec::new(), Direction::RL)
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
        end_line: Some(6),
        ..ControlVisualSubgraph::case(
            "s_scope_4",
            4,
            Some("\"a\"".to_string()),
            Vec::new(),
            Direction::RL,
        )
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
    let sg = ControlVisualSubgraph::case("s_default", 9, None, Vec::new(), Direction::RL);
    let text = serde_json::to_string(&sg).expect("serialize");
    assert!(
        text.contains(r#""caseTest":null"#),
        "expected null caseTest, got: {text}"
    );
}
