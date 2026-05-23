//! Serialize a `VariableData` into a `SerializedVariable`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use unsnarl_instrumentation::{count_if_verbose, record_elapsed_ns, timing_start, verbose};
use unsnarl_ir::primitive::{SourceIndex, Span};
use unsnarl_ir::serialized::{
    SerializedReferenceId, SerializedScopeId, SerializedVariable, SerializedVariableId,
};
use unsnarl_ir::{IrArena, ReferenceId, ScopeId, VariableId};

use crate::serializer::flat::serialize_definition::serialize_definition;
use crate::serializer::flat::span_of::span_of_identifier;

// Per-sub-phase accumulators. All `record_elapsed_ns` / `count_if_verbose`
// callers short-circuit when `--verbose` is off, so these atomics stay
// at zero in the common case.
static T_LOOKUP_NS: AtomicU64 = AtomicU64::new(0);
static T_IDENTIFIERS_NS: AtomicU64 = AtomicU64::new(0);
static T_REFERENCES_NS: AtomicU64 = AtomicU64::new(0);
static T_DEFS_NS: AtomicU64 = AtomicU64::new(0);
static T_BUILD_NS: AtomicU64 = AtomicU64::new(0);
static N_IDENTIFIERS: AtomicU64 = AtomicU64::new(0);
static N_REFERENCES: AtomicU64 = AtomicU64::new(0);
static N_DEFS: AtomicU64 = AtomicU64::new(0);

/// Reset every per-sub-phase accumulator ahead of a fresh
/// `serialize_variable` loop. No-op when verbose is off.
pub fn reset_stats() {
    if !verbose() {
        return;
    }
    for c in [
        &T_LOOKUP_NS,
        &T_IDENTIFIERS_NS,
        &T_REFERENCES_NS,
        &T_DEFS_NS,
        &T_BUILD_NS,
        &N_IDENTIFIERS,
        &N_REFERENCES,
        &N_DEFS,
    ] {
        c.store(0, Ordering::Relaxed);
    }
}

/// Drain every accumulator into a single `tracing::info!` event and
/// reset them to zero. No-op when verbose is off.
pub fn emit_stats() {
    if !verbose() {
        return;
    }
    let lookup_ns = T_LOOKUP_NS.swap(0, Ordering::Relaxed);
    let identifiers_ns = T_IDENTIFIERS_NS.swap(0, Ordering::Relaxed);
    let references_ns = T_REFERENCES_NS.swap(0, Ordering::Relaxed);
    let defs_ns = T_DEFS_NS.swap(0, Ordering::Relaxed);
    let build_ns = T_BUILD_NS.swap(0, Ordering::Relaxed);
    let identifiers_total = N_IDENTIFIERS.swap(0, Ordering::Relaxed);
    let references_total = N_REFERENCES.swap(0, Ordering::Relaxed);
    let defs_total = N_DEFS.swap(0, Ordering::Relaxed);
    tracing::info!(
        lookup_ms = lookup_ns / 1_000_000,
        identifiers_ms = identifiers_ns / 1_000_000,
        references_ms = references_ns / 1_000_000,
        defs_ms = defs_ns / 1_000_000,
        build_ms = build_ns / 1_000_000,
        identifiers_total = identifiers_total,
        references_total = references_total,
        defs_total = defs_total,
        "serialize_variable sub-phase totals",
    );
}

pub fn serialize_variable(
    arena: &IrArena,
    variable: VariableId,
    scope_ids: &HashMap<ScopeId, SerializedScopeId>,
    variable_ids: &HashMap<VariableId, SerializedVariableId>,
    reference_ids: &HashMap<ReferenceId, SerializedReferenceId>,
    index: &SourceIndex<'_>,
) -> SerializedVariable {
    let t = timing_start();
    let v = &arena.variables[variable];
    let id = variable_ids
        .get(&variable)
        .cloned()
        .unwrap_or_else(|| panic!("Variable id not found"));
    let scope = scope_ids
        .get(&v.scope)
        .cloned()
        .unwrap_or_else(|| panic!("Scope id not found for variable {}", v.name()));
    record_elapsed_ns(&T_LOOKUP_NS, t);

    let t = timing_start();
    let identifiers: Vec<Span> = v
        .identifiers
        .iter()
        .map(|ident| span_of_identifier(ident, index))
        .collect();
    count_if_verbose(&N_IDENTIFIERS, identifiers.len() as u64);
    record_elapsed_ns(&T_IDENTIFIERS_NS, t);

    let t = timing_start();
    let references: Vec<SerializedReferenceId> = v
        .references
        .iter()
        .filter_map(|r| reference_ids.get(r).cloned())
        .collect();
    count_if_verbose(&N_REFERENCES, references.len() as u64);
    record_elapsed_ns(&T_REFERENCES_NS, t);

    let t = timing_start();
    let defs: Vec<_> = v
        .defs
        .iter()
        .map(|&d| serialize_definition(&arena.definitions[d], index))
        .collect();
    count_if_verbose(&N_DEFS, defs.len() as u64);
    record_elapsed_ns(&T_DEFS_NS, t);

    let t = timing_start();
    let out = SerializedVariable::new(
        id,
        v.name().to_string(),
        scope,
        identifiers,
        references,
        defs,
    );
    record_elapsed_ns(&T_BUILD_NS, t);
    out
}
