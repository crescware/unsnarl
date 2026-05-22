//! Whether a switch-case consequent definitely exits the surrounding
//! function (via `return` or `throw`).
//!
//! The empty consequent answers `false` (no statement is reachable,
//! so the clause cannot exit the function on its own). A non-empty
//! consequent exits the function only when
//! [`abrupt_completion_type_of`] reports a non-empty set consisting
//! solely of `Return` / `Throw`.

use oxc_ast::ast::Statement;

use unsnarl_ir::completion::CompletionType;

use crate::abrupt_completion_type_of::abrupt_completion_type_of;

pub fn case_exits_function(consequent: &[Statement<'_>]) -> bool {
    let Some(last) = consequent.last() else {
        return false;
    };
    let Some(types) = abrupt_completion_type_of(last) else {
        return false;
    };
    assert!(
        !types.is_empty(),
        "case_exits_function: abrupt_completion_type_of returned an empty type set"
    );
    types
        .iter()
        .all(|t| matches!(t, CompletionType::Return | CompletionType::Throw))
}

#[cfg(test)]
#[path = "case_exits_function_test.rs"]
mod case_exits_function_test;
