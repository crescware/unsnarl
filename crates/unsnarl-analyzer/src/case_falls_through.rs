//! Whether the consequent of a switch-case clause falls through to
//! the next case clause.
//!
//! Mirrors `ts/src/analyzer/case-falls-through.ts`. An empty
//! consequent falls through (per ECMA §13.12.10); otherwise the
//! clause falls through iff its trailing statement does not transfer
//! control out of the switch.

use oxc_ast::ast::Statement;

use crate::is_control_exit::is_control_exit;

pub fn case_falls_through(consequent: &[Statement<'_>]) -> bool {
    let Some(last) = consequent.last() else {
        return true;
    };
    !is_control_exit(last)
}

#[cfg(test)]
#[path = "case_falls_through_test.rs"]
mod case_falls_through_test;
