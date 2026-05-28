//! Whether a statement definitely transfers control out of the
//! containing case clause.
//!
//! Matching on `oxc_ast::Statement` variants gives compile-time
//! exhaustiveness over the relevant statement shapes.
//!
//! `LabeledStatement` is handled transparently as the symmetric
//! companion to `abrupt_completion_type_of`'s `LabeledStatement`
//! case (#97 Part 1): the wrapper inherits its body's control-exit
//! status. The label-matching collapse described in ECMA §14.13.4
//! step 3 is tracked on issue #97 Part 2.

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
        Statement::LabeledStatement(ls) => is_control_exit(&ls.body),
        _ => false,
    }
}

#[cfg(test)]
#[path = "is_control_exit_test.rs"]
mod is_control_exit_test;
