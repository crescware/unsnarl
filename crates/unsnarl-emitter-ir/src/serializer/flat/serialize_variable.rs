//! Serialize a `VariableData` into a `SerializedVariable`.

use std::collections::HashMap;

use unsnarl_ir::serialized::{
    SerializedReferenceId, SerializedScopeId, SerializedVariable, SerializedVariableId,
};
use unsnarl_ir::{IrArena, ReferenceId, ScopeId, VariableId};

use crate::serializer::flat::serialize_definition::serialize_definition;
use crate::serializer::flat::span_of::span_of_identifier;

pub fn serialize_variable(
    arena: &IrArena,
    variable: VariableId,
    scope_ids: &HashMap<ScopeId, SerializedScopeId>,
    variable_ids: &HashMap<VariableId, SerializedVariableId>,
    reference_ids: &HashMap<ReferenceId, SerializedReferenceId>,
    raw: &str,
) -> SerializedVariable {
    let v = &arena.variables[variable];
    let id = variable_ids
        .get(&variable)
        .cloned()
        .unwrap_or_else(|| panic!("Variable id not found"));
    let scope = scope_ids
        .get(&v.scope)
        .cloned()
        .unwrap_or_else(|| panic!("Scope id not found for variable {}", v.name()));
    let identifiers = v
        .identifiers
        .iter()
        .map(|ident| span_of_identifier(ident, raw))
        .collect();
    let references = v
        .references
        .iter()
        .filter_map(|r| reference_ids.get(r).cloned())
        .collect();
    let defs = v
        .defs
        .iter()
        .map(|&d| serialize_definition(&arena.definitions[d], raw))
        .collect();
    SerializedVariable::new(
        id,
        v.name().to_string(),
        scope,
        identifiers,
        references,
        defs,
    )
}
