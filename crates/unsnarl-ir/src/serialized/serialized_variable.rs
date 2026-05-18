//! Serialized counterpart of `Variable`. Ports
//! `ts/src/ir/serialized/serialized-variable.ts`.
//!
//! TS uses `tupleWithRest([serializedDefinition$], serializedDefinition$)`
//! for `defs` to encode "at least one def by construction": the
//! serializer filters implicit-arguments bindings (the only producer of
//! empty defs) at boundary entry. The Rust port keeps the data shape
//! (`Vec<SerializedDefinition>`) and relies on the boundary to uphold
//! that invariant.

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
