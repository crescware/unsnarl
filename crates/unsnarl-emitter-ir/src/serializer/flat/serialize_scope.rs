//! Serialize a `ScopeData` into a `SerializedScope`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use unsnarl_annotations::Annotations;
use unsnarl_instrumentation::{count_if_verbose, record_elapsed_ns, timing_start, verbose};
use unsnarl_ir::primitive::{SourceIndex, Utf8ByteOffset};
use unsnarl_ir::serialized::{
    SerializedBlock, SerializedCallbackArgument, SerializedReferenceId, SerializedScope,
    SerializedScopeId, SerializedVariableId,
};
use unsnarl_ir::{IrArena, ReferenceId, ScopeId, VariableId};

use crate::serializer::flat::serialize_expression_statement_head::serialize_head_expression;
use crate::serializer::flat::span_of::span_of_node;

// Per-sub-phase accumulators. All `record_elapsed_ns` / `count_if_verbose`
// callers short-circuit when `--verbose` is off, so these atomics stay
// at zero in the common case.
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

/// Reset every per-sub-phase accumulator. Callers invoke this ahead
/// of a fresh `serialize_scope` loop so the next [`emit_stats`]
/// emission starts from zero. No-op when verbose is off.
pub fn reset_stats() {
    if !verbose() {
        return;
    }
    for c in [
        &T_LOOKUP_NS,
        &T_CHILD_SCOPES_NS,
        &T_BLOCK_NS,
        &T_VARIABLES_NS,
        &T_REFERENCES_NS,
        &T_THROUGH_NS,
        &T_ANNOTATIONS_NS,
        &T_BUILD_NS,
        &N_CHILD_SCOPES,
        &N_VARIABLES,
        &N_REFERENCES,
        &N_THROUGH,
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
    let child_scopes_ns = T_CHILD_SCOPES_NS.swap(0, Ordering::Relaxed);
    let block_ns = T_BLOCK_NS.swap(0, Ordering::Relaxed);
    let variables_ns = T_VARIABLES_NS.swap(0, Ordering::Relaxed);
    let references_ns = T_REFERENCES_NS.swap(0, Ordering::Relaxed);
    let through_ns = T_THROUGH_NS.swap(0, Ordering::Relaxed);
    let annotations_ns = T_ANNOTATIONS_NS.swap(0, Ordering::Relaxed);
    let build_ns = T_BUILD_NS.swap(0, Ordering::Relaxed);
    let child_scopes_total = N_CHILD_SCOPES.swap(0, Ordering::Relaxed);
    let variables_total = N_VARIABLES.swap(0, Ordering::Relaxed);
    let references_total = N_REFERENCES.swap(0, Ordering::Relaxed);
    let through_total = N_THROUGH.swap(0, Ordering::Relaxed);
    tracing::info!(
        lookup_ms = lookup_ns / 1_000_000,
        child_scopes_ms = child_scopes_ns / 1_000_000,
        block_ms = block_ns / 1_000_000,
        variables_ms = variables_ns / 1_000_000,
        references_ms = references_ns / 1_000_000,
        through_ms = through_ns / 1_000_000,
        annotations_ms = annotations_ns / 1_000_000,
        build_ms = build_ns / 1_000_000,
        child_scopes_total = child_scopes_total,
        variables_total = variables_total,
        references_total = references_total,
        through_total = through_total,
        "serialize_scope sub-phase totals",
    );
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
    let t = timing_start();
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
    record_elapsed_ns(&T_LOOKUP_NS, t);

    let t = timing_start();
    let child_scopes: Vec<SerializedScopeId> = s
        .child_scopes
        .iter()
        .filter_map(|c| scope_ids.get(c).cloned())
        .collect();
    count_if_verbose(&N_CHILD_SCOPES, child_scopes.len() as u64);
    record_elapsed_ns(&T_CHILD_SCOPES_NS, t);

    let t = timing_start();
    let block_end_offset = Utf8ByteOffset(s.block.span.end);
    let block = SerializedBlock {
        r#type: s.block.r#type.clone(),
        span: span_of_node(&s.block, index),
        end_span: index.span_at(block_end_offset),
    };
    record_elapsed_ns(&T_BLOCK_NS, t);

    let t = timing_start();
    let variables: Vec<SerializedVariableId> = s
        .variables
        .iter()
        .filter_map(|v| variable_ids.get(v).cloned())
        .collect();
    count_if_verbose(&N_VARIABLES, variables.len() as u64);
    record_elapsed_ns(&T_VARIABLES_NS, t);

    let t = timing_start();
    let references: Vec<SerializedReferenceId> = s
        .references
        .iter()
        .filter_map(|r| reference_ids.get(r).cloned())
        .collect();
    count_if_verbose(&N_REFERENCES, references.len() as u64);
    record_elapsed_ns(&T_REFERENCES_NS, t);

    let t = timing_start();
    let through: Vec<SerializedReferenceId> = s
        .through
        .iter()
        .filter_map(|r| reference_ids.get(r).cloned())
        .collect();
    count_if_verbose(&N_THROUGH, through.len() as u64);
    record_elapsed_ns(&T_THROUGH_NS, t);

    let t = timing_start();
    let ann = annotations.of_scope(scope);
    record_elapsed_ns(&T_ANNOTATIONS_NS, t);

    let t = timing_start();
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
        callback_argument: ann
            .callback_argument
            .as_ref()
            .map(|cb| SerializedCallbackArgument {
                callee: serialize_head_expression(&cb.callee, index),
                arg_index: cb.arg_index,
            }),
        falls_through: ann.falls_through,
        exits_function: ann.exits_function,
        nesting_depths: ann.nesting_depths.clone(),
        abrupt_statements: ann.abrupt_statements.clone(),
    };
    record_elapsed_ns(&T_BUILD_NS, t);
    out
}
