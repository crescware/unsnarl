//! Serialized counterpart of `Variable`.
//!
//! `defs` is `Vec<SerializedDefinition>` rather than a "non-empty"
//! newtype: the boundary upholds the "at least one def" invariant
//! by filtering implicit-arguments bindings (the sole producer of
//! empty defs) at entry.

use serde::Serialize;

use crate::filled_string::FilledString;
use crate::primitive::Span;
use crate::serialized::reference_id::SerializedReferenceId;
use crate::serialized::scope_id::SerializedScopeId;
use crate::serialized::serialized_definition::SerializedDefinition;
use crate::serialized::variable_id::SerializedVariableId;

#[derive(Serialize)]
pub struct SerializedVariable {
    pub id: SerializedVariableId,
    pub name: FilledString,
    pub scope: SerializedScopeId,
    pub identifiers: Vec<Span>,
    pub references: Vec<SerializedReferenceId>,
    pub defs: Vec<SerializedDefinition>,
}
