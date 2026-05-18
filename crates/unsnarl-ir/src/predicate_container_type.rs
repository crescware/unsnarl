//! Predicate container categorization. Ports
//! `ts/src/analyzer/predicate-container-type.ts`.

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
