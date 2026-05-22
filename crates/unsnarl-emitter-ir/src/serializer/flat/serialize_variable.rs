//! Serialize a `VariableData` into a `SerializedVariable`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use unsnarl_ir::primitive::{SourceIndex, Span};
use unsnarl_ir::serialized::{
    SerializedReferenceId, SerializedScopeId, SerializedVariable, SerializedVariableId,
};
use unsnarl_ir::{IrArena, ReferenceId, ScopeId, VariableId};

use crate::serializer::flat::serialize_definition::serialize_definition;
use crate::serializer::flat::span_of::span_of_identifier;

static T_LOOKUP_NS: AtomicU64 = AtomicU64::new(0);
static T_IDENTIFIERS_NS: AtomicU64 = AtomicU64::new(0);
static T_REFERENCES_NS: AtomicU64 = AtomicU64::new(0);
static T_DEFS_NS: AtomicU64 = AtomicU64::new(0);
static T_BUILD_NS: AtomicU64 = AtomicU64::new(0);
static N_IDENTIFIERS: AtomicU64 = AtomicU64::new(0);
static N_REFERENCES: AtomicU64 = AtomicU64::new(0);
static N_DEFS: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Default)]
pub struct SerializeVariableStats {
    pub lookup_ns: u64,
    pub identifiers_ns: u64,
    pub references_ns: u64,
    pub defs_ns: u64,
    pub build_ns: u64,
    pub identifiers_total: u64,
    pub references_total: u64,
    pub defs_total: u64,
}

pub fn take_serialize_variable_stats() -> SerializeVariableStats {
    SerializeVariableStats {
        lookup_ns: T_LOOKUP_NS.swap(0, Ordering::Relaxed),
        identifiers_ns: T_IDENTIFIERS_NS.swap(0, Ordering::Relaxed),
        references_ns: T_REFERENCES_NS.swap(0, Ordering::Relaxed),
        defs_ns: T_DEFS_NS.swap(0, Ordering::Relaxed),
        build_ns: T_BUILD_NS.swap(0, Ordering::Relaxed),
        identifiers_total: N_IDENTIFIERS.swap(0, Ordering::Relaxed),
        references_total: N_REFERENCES.swap(0, Ordering::Relaxed),
        defs_total: N_DEFS.swap(0, Ordering::Relaxed),
    }
}

fn record(counter: &AtomicU64, t: Instant) {
    counter.fetch_add(t.elapsed().as_nanos() as u64, Ordering::Relaxed);
}

pub fn serialize_variable(
    arena: &IrArena,
    variable: VariableId,
    scope_ids: &HashMap<ScopeId, SerializedScopeId>,
    variable_ids: &HashMap<VariableId, SerializedVariableId>,
    reference_ids: &HashMap<ReferenceId, SerializedReferenceId>,
    index: &SourceIndex<'_>,
) -> SerializedVariable {
    let t = Instant::now();
    let v = &arena.variables[variable];
    let id = variable_ids
        .get(&variable)
        .cloned()
        .unwrap_or_else(|| panic!("Variable id not found"));
    let scope = scope_ids
        .get(&v.scope)
        .cloned()
        .unwrap_or_else(|| panic!("Scope id not found for variable {}", v.name()));
    record(&T_LOOKUP_NS, t);

    let t = Instant::now();
    let identifiers: Vec<Span> = v
        .identifiers
        .iter()
        .map(|ident| span_of_identifier(ident, index))
        .collect();
    N_IDENTIFIERS.fetch_add(identifiers.len() as u64, Ordering::Relaxed);
    record(&T_IDENTIFIERS_NS, t);

    let t = Instant::now();
    let references: Vec<SerializedReferenceId> = v
        .references
        .iter()
        .filter_map(|r| reference_ids.get(r).cloned())
        .collect();
    N_REFERENCES.fetch_add(references.len() as u64, Ordering::Relaxed);
    record(&T_REFERENCES_NS, t);

    let t = Instant::now();
    let defs: Vec<_> = v
        .defs
        .iter()
        .map(|&d| serialize_definition(&arena.definitions[d], index))
        .collect();
    N_DEFS.fetch_add(defs.len() as u64, Ordering::Relaxed);
    record(&T_DEFS_NS, t);

    let t = Instant::now();
    let out = SerializedVariable::new(
        id,
        v.name().to_string(),
        scope,
        identifiers,
        references,
        defs,
    );
    record(&T_BUILD_NS, t);
    out
}
