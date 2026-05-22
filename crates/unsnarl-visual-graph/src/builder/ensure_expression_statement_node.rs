//! For a reference whose container is an ExpressionStatement, emit a
//! single synthetic node that stands in for the whole statement and
//! return its id. Subsequent references that share the same
//! container reuse the cached id rather than emitting a duplicate
//! node. Returns `None` when the reference is not inside an
//! ExpressionStatement (no synthetic stand-in is needed).

use unsnarl_ir::serialized::SerializedReference;

use crate::visual_element_type::NodeTypeTag;
use crate::visual_node::{SyntheticExtras, SyntheticNodeKind, SyntheticVisualNode, VisualNode};

use super::arena::{BuildArena, Container, ElementHandle};
use super::expression_statement_node_id::expression_statement_node_id;
use super::render_head_expression::render_head_expression;
use super::state::BuildState;

pub fn ensure_expression_statement_node(
    arena: &mut BuildArena,
    state: &mut BuildState,
    r: &SerializedReference,
    raw: &str,
    target: Container,
) -> Option<String> {
    let container = r.expression_statement_container.as_ref()?;
    let offset = container.start_span.offset.0;
    if let Some(existing) = state.expression_statement_by_offset.get(&offset) {
        return Some(existing.clone());
    }
    let id = expression_statement_node_id(offset);
    let name = render_head_expression(&container.head, raw);
    let start_line = container.start_span.line.0;
    let end_line = if container.end_span.line.0 != start_line {
        Some(container.end_span.line.0)
    } else {
        None
    };
    let node = VisualNode::Synthetic(SyntheticVisualNode {
        r#type: NodeTypeTag::Node,
        id: id.clone(),
        kind: SyntheticNodeKind::SyntheticExpressionStatement,
        name,
        line: start_line,
        end_line,
        is_jsx_element: false,
        unused: false,
        extras: SyntheticExtras::None {},
    });
    let idx = arena.push_node(node);
    arena.append_child(target, ElementHandle::Node(idx));
    state
        .expression_statement_by_offset
        .insert(offset, id.clone());
    state
        .node_id_origin_scope
        .insert(id.clone(), r.from.value().to_string());
    Some(id)
}

#[cfg(test)]
#[path = "ensure_expression_statement_node_test.rs"]
mod ensure_expression_statement_node_test;
