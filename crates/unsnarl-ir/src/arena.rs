//! `IrArena`: the in-memory IR backing store.
//!
//! Object references in the TS IR (e.g. `Scope.upper: Scope | null`,
//! `Variable.references: Reference[]`) become arena IDs in Rust
//! (`ScopeData.upper: Option<ScopeId>`,
//! `VariableData.references: Vec<ReferenceId>`). Each entity is held in
//! an `IndexVec` keyed by its corresponding ID newtype.

use oxc_index::IndexVec;

use crate::ids::{DefinitionId, ReferenceId, ScopeId, VariableId};
use crate::reference::ReferenceData;
use crate::scope::{DefinitionData, ScopeData, VariableData};

pub struct IrArena {
    pub scopes: IndexVec<ScopeId, ScopeData>,
    pub variables: IndexVec<VariableId, VariableData>,
    pub references: IndexVec<ReferenceId, ReferenceData>,
    pub definitions: IndexVec<DefinitionId, DefinitionData>,
}
