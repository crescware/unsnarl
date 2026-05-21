//! Mirrors `ts/src/emitter/mermaid/emit-node.test.ts`.

use unsnarl_visual_graph::visual_node::{BindingVisualNode, VisualNode};

use super::emit_node;
use crate::testing::{base_const_binding, base_render_state};

#[test]
fn appends_indent_id_syntax_as_a_single_line() {
    let mut state = base_render_state();
    let node = VisualNode::Binding(BindingVisualNode {
        id: "n_x".to_string(),
        name: "x".to_string(),
        line: 5,
        ..base_const_binding()
    });
    emit_node(&mut state, &node, "  ");
    assert_eq!(state.lines.len(), 1);
    assert!(state.lines[0].starts_with("  n_x["));
}

#[test]
fn respects_the_supplied_indent_verbatim() {
    let mut state = base_render_state();
    let node = VisualNode::Binding(BindingVisualNode {
        id: "n_x".to_string(),
        ..base_const_binding()
    });
    emit_node(&mut state, &node, "      ");
    assert!(state.lines[0].starts_with("      n_x"));
}

#[test]
fn does_not_modify_any_other_render_state_field() {
    let mut state = base_render_state();
    let node = VisualNode::Binding(base_const_binding());
    emit_node(&mut state, &node, "  ");
    assert!(state.placeholder_ids.is_empty());
    assert!(state.nest_class_map.is_empty());
}
