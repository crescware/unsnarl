use std::collections::HashMap;

use unsnarl_visual_graph::direction::Direction;
use unsnarl_visual_graph::visual_node::{BindingVisualNode, VisualNode};
use unsnarl_visual_graph::visual_subgraph::{
    ControlExtras, ControlVisualSubgraph, FunctionCallbackArg, OwnedExtras, OwnedVisualSubgraph,
    VisualSubgraph,
};

use super::subgraph_label;
use crate::mermaid_fixtures::{
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
fn function_with_no_owner_falls_back_to_anonymous_label_when_callback_arg_is_absent() {
    let sg: VisualSubgraph = OwnedVisualSubgraph {
        line: 3,
        end_line: Some(7),
        extras: OwnedExtras::Function {
            owner_node_id: None,
            owner_name: String::new(),
        },
        ..base_function_subgraph()
    }
    .into();
    assert_eq!(
        subgraph_label(&sg, &empty_map(), false),
        "(anonymous)<br/>L3-7"
    );
}

#[test]
fn function_with_no_owner_uses_callback_arg_header_when_present() {
    let sg: VisualSubgraph = OwnedVisualSubgraph {
        line: 3,
        end_line: Some(7),
        extras: OwnedExtras::Function {
            owner_node_id: None,
            owner_name: String::new(),
        },
        callback_arg: Some(FunctionCallbackArg {
            callee: "run".to_string(),
            arg_index: 0,
        }),
        ..base_function_subgraph()
    }
    .into();
    assert_eq!(
        subgraph_label(&sg, &empty_map(), false),
        "run(args[0])<br/>L3-7"
    );
}

#[test]
fn function_callback_arg_label_renders_higher_arg_indices_verbatim() {
    let sg: VisualSubgraph = OwnedVisualSubgraph {
        line: 9,
        extras: OwnedExtras::Function {
            owner_node_id: None,
            owner_name: String::new(),
        },
        callback_arg: Some(FunctionCallbackArg {
            callee: "scheduler.run".to_string(),
            arg_index: 2,
        }),
        ..base_function_subgraph()
    }
    .into();
    assert_eq!(
        subgraph_label(&sg, &empty_map(), false),
        "scheduler.run(args[2])<br/>L9"
    );
}

#[test]
fn function_owner_present_ignores_callback_arg() {
    // When an owner binding is available the function label uses
    // the owner name; the `callback_arg` field exists only to
    // disambiguate anonymous callbacks, so it must not override a
    // named owner.
    let sg: VisualSubgraph = OwnedVisualSubgraph {
        line: 2,
        end_line: Some(5),
        extras: OwnedExtras::Function {
            owner_node_id: Some("n_owner".to_string()),
            owner_name: "myFn".to_string(),
        },
        callback_arg: Some(FunctionCallbackArg {
            callee: "outer".to_string(),
            arg_index: 0,
        }),
        ..base_function_subgraph()
    }
    .into();
    assert_eq!(subgraph_label(&sg, &empty_map(), false), "myFn()<br/>L2-5");
}

#[test]
fn call_proxy_label_uses_call_name() {
    let sg: VisualSubgraph =
        OwnedVisualSubgraph::call_proxy("expr_stmt_42", 4, "run()", Vec::new(), Direction::RL)
            .into();
    assert_eq!(subgraph_label(&sg, &empty_map(), false), "run()<br/>L4");
}

#[test]
fn call_proxy_label_includes_end_line_when_call_spans_multiple_lines() {
    let mut sg = OwnedVisualSubgraph::call_proxy(
        "expr_stmt_50",
        2,
        "console.log()",
        Vec::new(),
        Direction::RL,
    );
    sg.end_line = Some(6);
    let sg: VisualSubgraph = sg.into();
    assert_eq!(
        subgraph_label(&sg, &empty_map(), false),
        "console.log()<br/>L2-6"
    );
}

#[test]
fn module_label_names_the_source_without_a_line_range() {
    let sg: VisualSubgraph = OwnedVisualSubgraph::module(
        "sg___utils_helper",
        1,
        "./utils/helper",
        Vec::new(),
        Direction::RL,
    )
    .into();
    assert_eq!(
        subgraph_label(&sg, &empty_map(), false),
        "module ./utils/helper"
    );
}

#[test]
fn module_label_appends_kind_under_debug() {
    let sg: VisualSubgraph =
        OwnedVisualSubgraph::module("sg_m", 2, "m", Vec::new(), Direction::RL).into();
    assert_eq!(
        subgraph_label(&sg, &empty_map(), true),
        "module m<br/>module"
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
