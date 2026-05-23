//! Emits the `subgraph ... end` block for a subgraph that is not
//! wrapped together with an owner node (or for the body half of a
//! wrapped function).

use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_subgraph::VisualSubgraph;

use crate::emit_node::emit_node;
use crate::emit_subgraph::emit_subgraph;
use crate::record_nest_slot::record_nest_slot;
use crate::render_state::RenderState;
use crate::strategy::EmptySubgraphContext;
use crate::subgraph_label::subgraph_label_into;

pub fn emit_plain_subgraph(
    state: &mut RenderState<'_>,
    sg: &VisualSubgraph,
    indent: &str,
    depth: u32,
) {
    // Build the `subgraph <id>["<label>"]` line directly into a
    // single owned `String` so the chain
    // `line_range_label` → `escape` → `subgraph_label` → outer
    // `format!` stops allocating one short-lived `String` per
    // nesting level (~35k subgraphs on `mermaid.js`).
    let mut header = String::with_capacity(indent.len() + 11 + sg.id().len() + sg.id().len() + 32);
    header.push_str(indent);
    header.push_str("subgraph ");
    header.push_str(sg.id());
    header.push_str("[\"");
    subgraph_label_into(&mut header, sg, &state.node_map, state.debug);
    header.push_str("\"]");
    state.lines.push(header);
    record_nest_slot(state, sg.id(), depth);
    let child_indent = format!("{indent}  ");
    let mut direction_line = String::with_capacity(child_indent.len() + "direction ".len() + 2);
    direction_line.push_str(&child_indent);
    direction_line.push_str("direction ");
    direction_line.push_str(sg.direction().as_str());
    state.lines.push(direction_line);

    let mut emitted_children = 0usize;
    for e in sg.elements() {
        if let VisualElement::Node(n) = e {
            if !state.wrapped_owner_ids.contains(n.id()) {
                emit_node(state, n, &child_indent);
                emitted_children += 1;
            }
        }
    }
    for e in sg.elements() {
        if let VisualElement::Subgraph(child) = e {
            emit_subgraph(state, child, &child_indent, depth + 1);
            emitted_children += 1;
        }
    }
    if emitted_children == 0 {
        let patch = state
            .strategy
            .empty_subgraph_placeholder(EmptySubgraphContext {
                subgraph_id: sg.id(),
                indent: &child_indent,
            });
        if let Some(p) = patch {
            state.lines.push(p.line);
            state.placeholder_ids.push(p.placeholder_id);
        }
    }
    let mut end_line = String::with_capacity(indent.len() + 3);
    end_line.push_str(indent);
    end_line.push_str("end");
    state.lines.push(end_line);
}

#[cfg(test)]
#[path = "emit_plain_subgraph_test.rs"]
mod emit_plain_subgraph_test;
