//! Pins the third TSV column: `path:line [unused ]name`. The
//! `unused ` prefix surfaces only when `node.unused` is true; the
//! default (`unused: false`) emits no prefix.

use unsnarl_visual_graph::visual_element_type::NodeTypeTag;
use unsnarl_visual_graph::visual_node::{
    BindingExtras, BindingNodeKind, BindingVisualNode, VisualNode,
};

use super::format_label;

fn node(name: &str, line: u32, unused: bool) -> VisualNode {
    VisualNode::Binding(BindingVisualNode {
        r#type: NodeTypeTag::Node,
        id: "n1".to_string(),
        name: name.to_string(),
        line,
        end_line: None,
        is_jsx_element: false,
        unused,
        kind: BindingNodeKind::ConstBinding,
        extras: BindingExtras::Variable {
            init_is_function: false,
        },
    })
}

#[test]
fn path_line_name_when_not_unused() {
    assert_eq!(
        format_label("foo.ts", &node("value", 10, false)),
        "foo.ts:10 value"
    );
}

#[test]
fn unused_prefix_when_node_unused_is_true() {
    assert_eq!(
        format_label("foo.ts", &node("value", 10, true)),
        "foo.ts:10 unused value"
    );
}

#[test]
fn unused_false_is_treated_as_not_unused_no_prefix() {
    assert_eq!(
        format_label("foo.ts", &node("value", 10, false)),
        "foo.ts:10 value"
    );
}

#[test]
fn default_unused_false_no_prefix() {
    assert_eq!(format_label("a/b.ts", &node("y", 1, false)), "a/b.ts:1 y");
}
