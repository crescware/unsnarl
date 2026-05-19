//! `IrArena`: the in-memory IR backing store.
//!
//! Cross-entity links go through arena IDs rather than `Rc` /
//! reference graphs: `ScopeData.upper: Option<ScopeId>`,
//! `VariableData.references: Vec<ReferenceId>`, etc. Each entity is
//! held in an `IndexVec` keyed by its corresponding ID newtype. The
//! arena is append-only after build, so no generational handles are
//! needed.

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
