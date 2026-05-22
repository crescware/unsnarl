//! Build the ExpressionStatement-wrapping side info for a reference.
//!
//! Two responsibilities are split so the AST-bearing handle is
//! supplied separately by the call site:
//!
//! * [`nearest_expression_statement`] returns the path entry of the
//!   innermost `ExpressionStatement` (or `None`).
//! * [`build_expression_statement_container`] turns that entry's span
//!   plus the statement's `expression` (looked up by the caller from
//!   the underlying AST) into the IR row.
//!
//! Keeping the lookup outside this crate lets `unsnarl-analyzer` stay
//! free of bookkeeping for `(path index → AST reference)`; the
//! pipeline crate owns that mapping.

use oxc_ast::ast::Expression;
use oxc_span::Span;

use unsnarl_ir::reference::ExpressionStatementContainer;
use unsnarl_ir::SourceOffset;
use unsnarl_oxc_parity::AstType;

use crate::build_head_expression::build_head_expression;
use crate::path_entry::PathEntry;

pub fn nearest_expression_statement(path: &[PathEntry]) -> Option<&PathEntry> {
    path.iter()
        .rev()
        .find(|e| matches!(e.node.r#type, AstType::ExpressionStatement))
}

pub fn build_expression_statement_container(
    statement_span: Span,
    expression: Option<&Expression<'_>>,
) -> ExpressionStatementContainer {
    ExpressionStatementContainer {
        start_offset: SourceOffset(statement_span.start),
        end_offset: SourceOffset(statement_span.end),
        head: build_head_expression(expression, statement_span),
    }
}

#[cfg(test)]
#[path = "expression_statement_container_test.rs"]
mod expression_statement_container_test;
