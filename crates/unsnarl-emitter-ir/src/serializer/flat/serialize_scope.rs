//! Serialize a `ScopeData` into a `SerializedScope`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use unsnarl_annotations::Annotations;
use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::serialized::{
    SerializedBlock, SerializedReferenceId, SerializedScope, SerializedScopeId,
    SerializedVariableId,
};
use unsnarl_ir::{IrArena, ReferenceId, ScopeId, VariableId};

use crate::serializer::flat::span_of::span_of_node;

// Per-sub-phase accumulators drained via `take_serialize_scope_stats`
// after the `flat::serialize_scopes` loop completes.
static T_LOOKUP_NS: AtomicU64 = AtomicU64::new(0);
static T_CHILD_SCOPES_NS: AtomicU64 = AtomicU64::new(0);
static T_BLOCK_NS: AtomicU64 = AtomicU64::new(0);
static T_VARIABLES_NS: AtomicU64 = AtomicU64::new(0);
static T_REFERENCES_NS: AtomicU64 = AtomicU64::new(0);
static T_THROUGH_NS: AtomicU64 = AtomicU64::new(0);
static T_ANNOTATIONS_NS: AtomicU64 = AtomicU64::new(0);
static T_BUILD_NS: AtomicU64 = AtomicU64::new(0);
static N_CHILD_SCOPES: AtomicU64 = AtomicU64::new(0);
static N_VARIABLES: AtomicU64 = AtomicU64::new(0);
static N_REFERENCES: AtomicU64 = AtomicU64::new(0);
static N_THROUGH: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Default)]
pub struct SerializeScopeStats {
    pub lookup_ns: u64,
    pub child_scopes_ns: u64,
    pub block_ns: u64,
    pub variables_ns: u64,
    pub references_ns: u64,
    pub through_ns: u64,
    pub annotations_ns: u64,
    pub build_ns: u64,
    pub child_scopes_total: u64,
    pub variables_total: u64,
    pub references_total: u64,
    pub through_total: u64,
}

pub fn take_serialize_scope_stats() -> SerializeScopeStats {
    SerializeScopeStats {
        lookup_ns: T_LOOKUP_NS.swap(0, Ordering::Relaxed),
        child_scopes_ns: T_CHILD_SCOPES_NS.swap(0, Ordering::Relaxed),
        block_ns: T_BLOCK_NS.swap(0, Ordering::Relaxed),
        variables_ns: T_VARIABLES_NS.swap(0, Ordering::Relaxed),
        references_ns: T_REFERENCES_NS.swap(0, Ordering::Relaxed),
        through_ns: T_THROUGH_NS.swap(0, Ordering::Relaxed),
        annotations_ns: T_ANNOTATIONS_NS.swap(0, Ordering::Relaxed),
        build_ns: T_BUILD_NS.swap(0, Ordering::Relaxed),
        child_scopes_total: N_CHILD_SCOPES.swap(0, Ordering::Relaxed),
        variables_total: N_VARIABLES.swap(0, Ordering::Relaxed),
        references_total: N_REFERENCES.swap(0, Ordering::Relaxed),
        through_total: N_THROUGH.swap(0, Ordering::Relaxed),
    }
}

fn record(counter: &AtomicU64, t: Instant) {
    counter.fetch_add(t.elapsed().as_nanos() as u64, Ordering::Relaxed);
}

pub fn serialize_scope(
    arena: &IrArena,
    scope: ScopeId,
    scope_ids: &HashMap<ScopeId, SerializedScopeId>,
    variable_ids: &HashMap<VariableId, SerializedVariableId>,
    reference_ids: &HashMap<ReferenceId, SerializedReferenceId>,
    annotations: &dyn Annotations,
    index: &SourceIndex<'_>,
) -> SerializedScope {
    let t = Instant::now();
    let s = &arena.scopes[scope];
    let id = scope_ids
        .get(&scope)
        .cloned()
        .unwrap_or_else(|| panic!("Scope id not found"));
    let upper = s.upper.and_then(|u| scope_ids.get(&u).cloned());
    let variable_scope = scope_ids
        .get(&s.variable_scope)
        .cloned()
        .unwrap_or_else(|| id.clone());
    record(&T_LOOKUP_NS, t);

    let t = Instant::now();
    let child_scopes: Vec<SerializedScopeId> = s
        .child_scopes
        .iter()
        .filter_map(|c| scope_ids.get(c).cloned())
        .collect();
    N_CHILD_SCOPES.fetch_add(child_scopes.len() as u64, Ordering::Relaxed);
    record(&T_CHILD_SCOPES_NS, t);

    let t = Instant::now();
    let block_end_offset = s.block.span.end as usize;
    let block = SerializedBlock {
        r#type: s.block.r#type.clone(),
        span: span_of_node(&s.block, index),
        end_span: index.span_at(block_end_offset),
    };
    record(&T_BLOCK_NS, t);

    let t = Instant::now();
    let variables: Vec<SerializedVariableId> = s
        .variables
        .iter()
        .filter_map(|v| variable_ids.get(v).cloned())
        .collect();
    N_VARIABLES.fetch_add(variables.len() as u64, Ordering::Relaxed);
    record(&T_VARIABLES_NS, t);

    let t = Instant::now();
    let references: Vec<SerializedReferenceId> = s
        .references
        .iter()
        .filter_map(|r| reference_ids.get(r).cloned())
        .collect();
    N_REFERENCES.fetch_add(references.len() as u64, Ordering::Relaxed);
    record(&T_REFERENCES_NS, t);

    let t = Instant::now();
    let through: Vec<SerializedReferenceId> = s
        .through
        .iter()
        .filter_map(|r| reference_ids.get(r).cloned())
        .collect();
    N_THROUGH.fetch_add(through.len() as u64, Ordering::Relaxed);
    record(&T_THROUGH_NS, t);

    let t = Instant::now();
    let ann = annotations.of_scope(scope);
    record(&T_ANNOTATIONS_NS, t);

    let t = Instant::now();
    let out = SerializedScope {
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
    };
    record(&T_BUILD_NS, t);
    out
}
