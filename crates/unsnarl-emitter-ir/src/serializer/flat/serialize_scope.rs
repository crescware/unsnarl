//! Serialize a `ScopeData` into a `SerializedScope`.

use std::collections::HashMap;

use unsnarl_annotations::Annotations;
use unsnarl_ir::primitive::span_from_offset;
use unsnarl_ir::serialized::{
    SerializedBlock, SerializedReferenceId, SerializedScope, SerializedScopeId,
    SerializedVariableId,
};
use unsnarl_ir::{IrArena, ReferenceId, ScopeId, VariableId};

use crate::serializer::flat::span_of::span_of_node;

pub fn serialize_scope(
    arena: &IrArena,
    scope: ScopeId,
    scope_ids: &HashMap<ScopeId, SerializedScopeId>,
    variable_ids: &HashMap<VariableId, SerializedVariableId>,
    reference_ids: &HashMap<ReferenceId, SerializedReferenceId>,
    annotations: &dyn Annotations,
    raw: &str,
) -> SerializedScope {
    let s = &arena.scopes[scope];
    let id = scope_ids
        .get(&scope)
        .cloned()
        .unwrap_or_else(|| panic!("Scope id not found"));
    let upper = s.upper.and_then(|u| scope_ids.get(&u).cloned());
    let child_scopes = s
        .child_scopes
        .iter()
        .filter_map(|c| scope_ids.get(c).cloned())
        .collect();
    let variable_scope = scope_ids
        .get(&s.variable_scope)
        .cloned()
        .unwrap_or_else(|| id.clone());
    let block_end_offset = s.block.span.end as usize;
    let block = SerializedBlock {
        r#type: s.block.r#type.clone(),
        span: span_of_node(&s.block, raw),
        end_span: span_from_offset(raw, block_end_offset),
    };
    let variables = s
        .variables
        .iter()
        .filter_map(|v| variable_ids.get(v).cloned())
        .collect();
    let references = s
        .references
        .iter()
        .filter_map(|r| reference_ids.get(r).cloned())
        .collect();
    let through = s
        .through
        .iter()
        .filter_map(|r| reference_ids.get(r).cloned())
        .collect();
    let ann = annotations.of_scope(scope);
    SerializedScope {
        id,
        r#type: s.r#type,
        is_strict: s.is_strict,
        upper,
        child_scopes,
        variable_scope,
        block,
        variables,
        references,
        through,
        function_expression_scope: s.function_expression_scope,
        block_context: ann.block_context.clone(),
        falls_through: ann.falls_through,
        exits_function: ann.exits_function,
        nesting_depths: ann.nesting_depths.clone(),
    }
}
