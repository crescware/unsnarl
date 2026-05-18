//! Variable row. Cross-entity links go through arena IDs.

use crate::ids::{DefinitionId, ReferenceId, ScopeId};
use crate::primitive::AstIdentifier;

pub struct VariableData {
    name: String,
    pub scope: ScopeId,
    pub identifiers: Vec<AstIdentifier>,
    pub references: Vec<ReferenceId>,
    pub defs: Vec<DefinitionId>,
}

impl VariableData {
    pub fn new(
        name: impl Into<String>,
        scope: ScopeId,
        identifiers: Vec<AstIdentifier>,
        references: Vec<ReferenceId>,
        defs: Vec<DefinitionId>,
    ) -> Self {
        let name = name.into();
        assert!(!name.is_empty(), "VariableData.name must be non-empty");
        Self {
            name,
            scope,
            identifiers,
            references,
            defs,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub type Variable = VariableData;
