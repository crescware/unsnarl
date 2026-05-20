//! Whether a statement definitely transfers control out of the
//! containing case clause.
//!
//! Mirrors `ts/src/analyzer/is-control-exit.ts`. The TS `isAstNode`
//! predicate guarding dynamic property access is absorbed into Rust's
//! type system: matching on `oxc_ast::Statement` variants gives the
//! same guarantee at compile time.

use oxc_ast::ast::Statement;

pub fn is_control_exit(node: &Statement<'_>) -> bool {
    match node {
        Statement::BreakStatement(_)
        | Statement::ContinueStatement(_)
        | Statement::ReturnStatement(_)
        | Statement::ThrowStatement(_) => true,
        Statement::BlockStatement(block) => match block.body.last() {
            Some(last) => is_control_exit(last),
            None => false,
        },
        Statement::IfStatement(if_stmt) => match &if_stmt.alternate {
            Some(alternate) => is_control_exit(&if_stmt.consequent) && is_control_exit(alternate),
            None => false,
        },
        _ => false,
    }
}

#[cfg(test)]
#[path = "is_control_exit_test.rs"]
mod is_control_exit_test;
