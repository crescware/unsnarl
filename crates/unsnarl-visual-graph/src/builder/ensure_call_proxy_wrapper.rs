//! Look up (or create on first sight) the statement-level `CallProxy`
//! wrapper subgraph for an `ExpressionStatement`-hosted callback.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedExpressionStatementContainer;

use crate::direction::Direction;
use crate::visual_subgraph::OwnedVisualSubgraph;

use super::arena::{BuildArena, Container, ElementHandle, SubgraphIdx};
use super::context::BuilderContext;
use super::expression_statement_node_id::expression_statement_node_id;
use super::render_head_expression::render_head_expression;
use super::state::BuildState;

/// Look up (or create on first sight) the `CallProxy` wrapper
/// subgraph for `statement`. Taking the
/// [`SerializedExpressionStatementContainer`] by reference -- the one
/// [`super::expression_statement_index::ExpressionStatementIndex::enclosing`]
/// just returned -- means the wrapper's id, `callName`, and span lines
/// come straight off a known statement; there is no offset to look back
/// up and no miss to guard.
///
/// First-sight allocation appends the wrapper to `container` at
/// the call site, so the wrapper lands in the same source-order
/// position as the first callback child belonging to that
/// statement. Subsequent callbacks for the same statement reuse
/// the cached subgraph index and only emit themselves as wrapper
/// children.
pub fn ensure_call_proxy_wrapper(
    arena: &mut BuildArena,
    state: &mut BuildState,
    ctx: &BuilderContext<'_>,
    container: Container,
    call_proxy_by_stmt_offset: &mut HashMap<u32, SubgraphIdx>,
    statement: &SerializedExpressionStatementContainer,
) -> SubgraphIdx {
    let stmt_offset = statement.start_span.offset.0;
    if let Some(&idx) = call_proxy_by_stmt_offset.get(&stmt_offset) {
        return idx;
    }
    let id = expression_statement_node_id(stmt_offset);
    let name = render_head_expression(&statement.head, &ctx.source_index);
    let start_line = statement.start_span.line.0;
    let end_line = if statement.end_span.line.0 != start_line {
        Some(statement.end_span.line.0)
    } else {
        None
    };
    let mut sg =
        OwnedVisualSubgraph::call_proxy(id.clone(), start_line, name, Vec::new(), Direction::RL);
    sg.end_line = end_line;
    let idx = arena.push_subgraph(sg.into());
    arena.append_child(container, ElementHandle::Subgraph(idx));
    // Wire the offset cache so the downstream
    // `ensure_expression_statement_node` call (during reference
    // traversal) returns this wrapper subgraph's id instead of
    // emitting a separate leaf `expr_stmt_<offset>` node.
    state.expression_statement_by_offset.insert(stmt_offset, id);
    call_proxy_by_stmt_offset.insert(stmt_offset, idx);
    idx
}
