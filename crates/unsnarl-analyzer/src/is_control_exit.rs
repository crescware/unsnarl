//! Whether a statement definitely transfers control out of the
//! containing case clause.
//!
//! Implemented as a thin wrapper over
//! [`abrupt_completion_type_of`]: a statement transfers control out
//! iff its set of reachable abrupt completion types is non-empty.
//! The wrapper keeps `case_falls_through` and other callers
//! mechanically in sync with the LabelledStatement / IfStatement /
//! BlockStatement recursion rules that
//! `abrupt_completion_type_of` enforces — in particular the
//! ECMA §14.13.4 step 3 label-matching collapse, so
//! `outer: { break outer; }` is correctly treated as a normal
//! completion (no control exit).

use oxc_ast::ast::Statement;

use crate::abrupt_completion_type_of::abrupt_completion_type_of;

pub fn is_control_exit(node: &Statement<'_>) -> bool {
    abrupt_completion_type_of(node).is_some()
}

#[cfg(test)]
#[path = "is_control_exit_test.rs"]
mod is_control_exit_test;
