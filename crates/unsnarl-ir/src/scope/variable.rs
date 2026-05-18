//! Variable row. Ports `ts/src/ir/scope/variable.ts`.
//!
//! TS holds direct object references (`scope: Scope`, `references:
//! Reference[]`, `defs: Definition[]`); the Rust IR uses arena IDs.

use crate::ids::{DefinitionId, ReferenceId, ScopeId};
use crate::primitive::AstIdentifier;

pub struct VariableData {
    pub name: String,
    pub scope: ScopeId,
    pub identifiers: Vec<AstIdentifier>,
    pub references: Vec<ReferenceId>,
    pub defs: Vec<DefinitionId>,
}

pub type Variable = VariableData;
