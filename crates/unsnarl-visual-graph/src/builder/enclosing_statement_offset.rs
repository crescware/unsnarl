//! Visual-layer correlation: which `ExpressionStatement` (if any)
//! encloses a callback function scope.
//!
//! The IR's `callbackArgument` annotation carries only the structural
//! fact (`callee` + `argIndex`). Deciding *whether* a callback is
//! hosted by a statement-level CallProxy wrapper -- and under which
//! `expr_stmt_<offset>` it groups -- is a rendering concern, resolved
//! here from the `ExpressionStatement` spans the builder already owns
//! (`BuilderContext::expression_statement_containers_by_offset`).
//!
//! A `Some` result mirrors what an enclosing `ExpressionStatement`
//! would supply: variable-bound, returned, and otherwise
//! non-statement callbacks return `None` and get only their label,
//! never a wrapper.

use std::collections::HashMap;

use unsnarl_ir::serialized::SerializedExpressionStatementContainer;

/// Start offset (UTF-16) of the innermost `ExpressionStatement` whose
/// span contains `[start, end]`, or `None` when none encloses it.
///
/// `ExpressionStatement` spans nest without partial overlap, so among
/// the containers that contain `[start, end]` the one with the largest
/// start offset is the innermost -- the same statement the analyzer's
/// AST walk would have reached. The returned offset is also the map
/// key, so callers can look the container back up for its
/// `expr_stmt_<offset>` id and head.
pub fn enclosing_statement_offset(
    start: u32,
    end: u32,
    containers: &HashMap<u32, &SerializedExpressionStatementContainer>,
) -> Option<u32> {
    containers
        .values()
        .filter(|c| c.start_span.offset.0 <= start && c.end_span.offset.0 >= end)
        .map(|c| c.start_span.offset.0)
        .max()
}

#[cfg(test)]
#[path = "enclosing_statement_offset_test.rs"]
mod enclosing_statement_offset_test;
