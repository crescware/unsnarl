//! Variable row. Cross-entity links go through arena IDs.

use crate::ids::{DefinitionId, ReferenceId, ScopeId};
use crate::non_empty::assert_non_empty;
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
        name: String,
        scope: ScopeId,
        identifiers: Vec<AstIdentifier>,
        references: Vec<ReferenceId>,
        defs: Vec<DefinitionId>,
    ) -> Self {
        assert_non_empty(&name, "VariableData.name");
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
