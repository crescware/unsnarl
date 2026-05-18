//! Predicate-container categorization.

use serde::Serialize;

#[derive(Serialize)]
pub enum PredicateContainerType {
    IfStatement,
    SwitchStatement,
    WhileStatement,
    DoWhileStatement,
    ForStatement,
    ForOfStatement,
    ForInStatement,
}
