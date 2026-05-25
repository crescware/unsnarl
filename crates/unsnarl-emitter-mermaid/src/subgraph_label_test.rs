use std::collections::HashMap;

use unsnarl_visual_graph::visual_node::{BindingVisualNode, VisualNode};
use unsnarl_visual_graph::visual_subgraph::{
    ControlExtras, ControlVisualSubgraph, OwnedExtras, OwnedVisualSubgraph, VisualSubgraph,
};

use super::subgraph_label;
use crate::testing::{
    base_case_subgraph, base_class_subgraph, base_const_binding, base_function_subgraph,
    base_if_else_container_subgraph,
};

fn empty_map<'a>() -> HashMap<String, &'a VisualNode> {
    HashMap::new()
}

#[test]
fn function_uses_owner_name_when_present_falling_back_to_node_map_then_empty() {
    let sg: VisualSubgraph = OwnedVisualSubgraph {
        line: 2,
        end_line: Some(5),
        extras: OwnedExtras::Function {
            owner_node_id: Some("n_owner".to_string()),
            owner_name: "myFn".to_string(),
        },
        ..base_function_subgraph()
    }
    .into();
    assert_eq!(subgraph_label(&sg, &empty_map(), false), "myFn()<br/>L2-5");
}

#[test]
fn function_falls_back_to_owner_node_name_when_owner_name_is_empty() {
    let sg: VisualSubgraph = OwnedVisualSubgraph {
        line: 1,
        extras: OwnedExtras::Function {
            owner_node_id: Some("n_owner".to_string()),
            owner_name: String::new(),
        },
        ..base_function_subgraph()
    }
    .into();
    let owner: VisualNode = BindingVisualNode {
        id: "n_owner".to_string(),
        name: "fallback".to_string(),
        ..base_const_binding()
    }
    .into();
    let mut map: HashMap<String, &VisualNode> = HashMap::new();
    map.insert("n_owner".to_string(), &owner);
    assert_eq!(subgraph_label(&sg, &map, false), "fallback()<br/>L1");
}

#[test]
fn function_with_empty_owner_name_and_unknown_owner_node_id_yields_an_empty_name() {
    let sg: VisualSubgraph = OwnedVisualSubgraph {
        line: 1,
        extras: OwnedExtras::Function {
            owner_node_id: Some("n_owner".to_string()),
            owner_name: String::new(),
        },
        ..base_function_subgraph()
    }
    .into();
    assert_eq!(subgraph_label(&sg, &empty_map(), false), "()<br/>L1");
}

#[test]
fn case_with_explicit_case_test_gets_case_prefix() {
    let sg: VisualSubgraph = ControlVisualSubgraph {
        line: 4,
        extras: ControlExtras::Case {
            case_test: Some("1".to_string()),
        },
        ..base_case_subgraph()
    }
    .into();
    assert_eq!(subgraph_label(&sg, &empty_map(), false), "case 1 L4");
}

#[test]
fn case_with_null_case_test_renders_as_default() {
    let sg: VisualSubgraph = ControlVisualSubgraph {
        line: 4,
        extras: ControlExtras::Case { case_test: None },
        ..base_case_subgraph()
    }
    .into();
    assert_eq!(subgraph_label(&sg, &empty_map(), false), "default L4");
}

#[test]
fn class_with_a_class_name_renders_as_class_name() {
    let sg: VisualSubgraph = OwnedVisualSubgraph {
        line: 5,
        end_line: Some(7),
        extras: OwnedExtras::Class {
            class_name: Some("Foo".to_string()),
        },
        ..base_class_subgraph()
    }
    .into();
    assert_eq!(
        subgraph_label(&sg, &empty_map(), false),
        "class Foo<br/>L5-7"
    );
}

#[test]
fn class_with_class_name_none_renders_as_class_anonymous() {
    let sg: VisualSubgraph = OwnedVisualSubgraph {
        line: 2,
        end_line: Some(4),
        extras: OwnedExtras::Class { class_name: None },
        ..base_class_subgraph()
    }
    .into();
    assert_eq!(
        subgraph_label(&sg, &empty_map(), false),
        "class (anonymous)<br/>L2-4"
    );
}

#[test]
fn if_else_container_with_has_else_true_says_if_else_otherwise_if() {
    let sg_with: VisualSubgraph = OwnedVisualSubgraph {
        extras: OwnedExtras::IfElseContainer { has_else: true },
        ..base_if_else_container_subgraph()
    }
    .into();
    assert_eq!(subgraph_label(&sg_with, &empty_map(), false), "if-else L1");
    let sg_without: VisualSubgraph = OwnedVisualSubgraph {
        extras: OwnedExtras::IfElseContainer { has_else: false },
        ..base_if_else_container_subgraph()
    }
    .into();
    assert_eq!(subgraph_label(&sg_without, &empty_map(), false), "if L1");
}

#[test]
fn debug_true_appends_subgraph_kind_to_the_standard_label() {
    let sg: VisualSubgraph = OwnedVisualSubgraph {
        line: 2,
        end_line: Some(5),
        extras: OwnedExtras::Function {
            owner_node_id: Some("n_owner".to_string()),
            owner_name: "myFn".to_string(),
        },
        ..base_function_subgraph()
    }
    .into();
    assert_eq!(
        subgraph_label(&sg, &empty_map(), true),
        "myFn()<br/>L2-5<br/>function"
    );
}

#[test]
fn debug_true_if_else_container_surfaces_the_kind_even_when_the_prefix_differs() {
    let sg: VisualSubgraph = OwnedVisualSubgraph {
        extras: OwnedExtras::IfElseContainer { has_else: true },
        ..base_if_else_container_subgraph()
    }
    .into();
    assert_eq!(
        subgraph_label(&sg, &empty_map(), true),
        "if-else L1<br/>if-else-container"
    );
}
