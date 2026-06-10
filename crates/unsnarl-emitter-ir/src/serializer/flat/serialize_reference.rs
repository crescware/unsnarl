//! Serialize a `ReferenceData` into a `SerializedReference`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use unsnarl_annotations::Annotations;
use unsnarl_instrumentation::{count_if_verbose, record_elapsed_ns, timing_start, verbose};
use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::reference::reference_flags::ReferenceFlags;
use unsnarl_ir::reference::ReferenceCompletion;
use unsnarl_ir::serialized::{
    SerializedCompletion, SerializedExpressionStatementContainer, SerializedFlags,
    SerializedJsxElement, SerializedReference, SerializedReferenceId,
    SerializedReferenceIdentifier, SerializedScopeId, SerializedVariableId,
};
use unsnarl_ir::{IrArena, ReferenceId, ScopeId, VariableId};

use crate::serializer::flat::serialize_expression_statement_head::{
    emit_head_stats, serialize_head_expression,
};
use crate::serializer::flat::span_of::span_of_identifier;

// Per-sub-phase accumulators. All `record_elapsed_ns` / `count_if_verbose`
// callers short-circuit when `--verbose` is off, so these atomics stay
// at zero in the common case.
static T_LOOKUP_NS: AtomicU64 = AtomicU64::new(0);
static T_ANNOTATIONS_NS: AtomicU64 = AtomicU64::new(0);
static T_OWNERS_NS: AtomicU64 = AtomicU64::new(0);
static T_COMPLETION_NS: AtomicU64 = AtomicU64::new(0);
static T_JSX_NS: AtomicU64 = AtomicU64::new(0);
static T_EXPR_STMT_CONTAINER_NS: AtomicU64 = AtomicU64::new(0);
static T_EXPR_STMT_HEAD_NS: AtomicU64 = AtomicU64::new(0);
static T_IDENTIFIER_NS: AtomicU64 = AtomicU64::new(0);
static T_BUILD_NS: AtomicU64 = AtomicU64::new(0);
static N_OWNERS_TOTAL: AtomicU64 = AtomicU64::new(0);
static N_RETURN: AtomicU64 = AtomicU64::new(0);
static N_THROW: AtomicU64 = AtomicU64::new(0);
static N_JSX: AtomicU64 = AtomicU64::new(0);
static N_EXPR_STMT: AtomicU64 = AtomicU64::new(0);

/// Reset every per-sub-phase accumulator. Called by [`emit_stats`]
/// before it emits, and again by callers ahead of a fresh
/// `serialize_reference` loop so the next emission starts from zero.
/// No-op when verbose is off (the atomics are already zero).
pub fn reset_stats() {
    if !verbose() {
        return;
    }
    for c in [
        &T_LOOKUP_NS,
        &T_ANNOTATIONS_NS,
        &T_OWNERS_NS,
        &T_COMPLETION_NS,
        &T_JSX_NS,
        &T_EXPR_STMT_CONTAINER_NS,
        &T_EXPR_STMT_HEAD_NS,
        &T_IDENTIFIER_NS,
        &T_BUILD_NS,
        &N_OWNERS_TOTAL,
        &N_RETURN,
        &N_THROW,
        &N_JSX,
        &N_EXPR_STMT,
    ] {
        c.store(0, Ordering::Relaxed);
    }
}

/// Drain every per-sub-phase accumulator into a single
/// `tracing::info!` event and reset them to zero. Also drains the
/// expression-statement-head accumulators (via
/// [`emit_head_stats`]) so all per-call timing for the just-finished
/// `serialize_references` loop lands in the same group of events.
/// No-op when verbose is off.
pub fn emit_stats() {
    if !verbose() {
        return;
    }
    let lookup_ns = T_LOOKUP_NS.swap(0, Ordering::Relaxed);
    let annotations_ns = T_ANNOTATIONS_NS.swap(0, Ordering::Relaxed);
    let owners_ns = T_OWNERS_NS.swap(0, Ordering::Relaxed);
    let completion_ns = T_COMPLETION_NS.swap(0, Ordering::Relaxed);
    let jsx_ns = T_JSX_NS.swap(0, Ordering::Relaxed);
    let expr_stmt_container_ns = T_EXPR_STMT_CONTAINER_NS.swap(0, Ordering::Relaxed);
    let expr_stmt_head_ns = T_EXPR_STMT_HEAD_NS.swap(0, Ordering::Relaxed);
    let identifier_ns = T_IDENTIFIER_NS.swap(0, Ordering::Relaxed);
    let build_ns = T_BUILD_NS.swap(0, Ordering::Relaxed);
    let owners_total = N_OWNERS_TOTAL.swap(0, Ordering::Relaxed);
    let return_count = N_RETURN.swap(0, Ordering::Relaxed);
    let throw_count = N_THROW.swap(0, Ordering::Relaxed);
    let jsx_count = N_JSX.swap(0, Ordering::Relaxed);
    let expression_statement_count = N_EXPR_STMT.swap(0, Ordering::Relaxed);
    let (head_nodes, head_span_calls, head_span_ns) = emit_head_stats();
    tracing::info!(
        lookup_ms = lookup_ns / 1_000_000,
        annotations_ms = annotations_ns / 1_000_000,
        owners_ms = owners_ns / 1_000_000,
        completion_ms = completion_ns / 1_000_000,
        jsx_ms = jsx_ns / 1_000_000,
        expr_stmt_container_ms = expr_stmt_container_ns / 1_000_000,
        expr_stmt_head_ms = expr_stmt_head_ns / 1_000_000,
        identifier_ms = identifier_ns / 1_000_000,
        build_ms = build_ns / 1_000_000,
        owners_total = owners_total,
        return_count = return_count,
        throw_count = throw_count,
        jsx_count = jsx_count,
        expression_statement_count = expression_statement_count,
        head_nodes_total = head_nodes,
        head_span_calls = head_span_calls,
        head_span_ms = head_span_ns / 1_000_000,
        "serialize_reference sub-phase totals",
    );
}

pub fn serialize_reference(
    arena: &IrArena,
    reference: ReferenceId,
    scope_ids: &HashMap<ScopeId, SerializedScopeId>,
    variable_ids: &HashMap<VariableId, SerializedVariableId>,
    reference_ids: &HashMap<ReferenceId, SerializedReferenceId>,
    annotations: &dyn Annotations,
    index: &SourceIndex<'_>,
) -> SerializedReference {
    let t = timing_start();
    let r = &arena.references[reference];
    let id = reference_ids
        .get(&reference)
        .cloned()
        .unwrap_or_else(|| panic!("Reference id not found"));
    let from = scope_ids
        .get(&r.from)
        .cloned()
        .unwrap_or_else(|| panic!("Scope id not found for reference {}", r.identifier.name()));
    record_elapsed_ns(&T_LOOKUP_NS, t);

    let t = timing_start();
    let ann = annotations.of_reference(reference);
    record_elapsed_ns(&T_ANNOTATIONS_NS, t);

    let t = timing_start();
    let resolved = r.resolved.and_then(|v| variable_ids.get(&v).cloned());
    let owners: Vec<SerializedVariableId> = ann
        .owners
        .iter()
        .filter_map(|v| variable_ids.get(v).cloned())
        .collect();
    count_if_verbose(&N_OWNERS_TOTAL, owners.len() as u64);
    record_elapsed_ns(&T_OWNERS_NS, t);

    let read = (r.flags & ReferenceFlags::READ).0 != 0;
    let write = (r.flags & ReferenceFlags::WRITE).0 != 0;
    let flags = SerializedFlags {
        read,
        write,
        call: ann.flags.call,
        receiver: ann.flags.receiver,
    };

    let t = timing_start();
    let completion = match &ann.completion {
        ReferenceCompletion::Normal => SerializedCompletion::Normal,
        ReferenceCompletion::Return {
            start_offset,
            end_offset,
        } => {
            count_if_verbose(&N_RETURN, 1);
            SerializedCompletion::Return {
                start_span: index.span_at(*start_offset),
                end_span: index.span_at(*end_offset),
            }
        }
        ReferenceCompletion::Throw {
            start_offset,
            end_offset,
        } => {
            count_if_verbose(&N_THROW, 1);
            SerializedCompletion::Throw {
                start_span: index.span_at(*start_offset),
                end_span: index.span_at(*end_offset),
            }
        }
    };
    record_elapsed_ns(&T_COMPLETION_NS, t);

    let t = timing_start();
    let jsx_element = ann.jsx_element.as_ref().map(|jsx| {
        count_if_verbose(&N_JSX, 1);
        SerializedJsxElement {
            start_span: index.span_at(jsx.start_offset),
            end_span: index.span_at(jsx.end_offset),
        }
    });
    record_elapsed_ns(&T_JSX_NS, t);

    let expression_statement_container = ann.expression_statement_container.as_ref().map(|c| {
        count_if_verbose(&N_EXPR_STMT, 1);
        let t_container = timing_start();
        let start_span = index.span_at(c.start_offset);
        let end_span = index.span_at(c.end_offset);
        record_elapsed_ns(&T_EXPR_STMT_CONTAINER_NS, t_container);
        let t_head = timing_start();
        let head = serialize_head_expression(&c.head, index);
        record_elapsed_ns(&T_EXPR_STMT_HEAD_NS, t_head);
        SerializedExpressionStatementContainer {
            start_span,
            end_span,
            head,
            expression_start_span: c.expression_start_offset.map(|o| index.span_at(o)),
        }
    });

    let t = timing_start();
    let identifier = SerializedReferenceIdentifier::new(
        r.identifier.name().to_string(),
        span_of_identifier(&r.identifier, index),
    );
    record_elapsed_ns(&T_IDENTIFIER_NS, t);

    let t = timing_start();
    let out = SerializedReference {
        id,
        identifier,
        from,
        resolved,
        owners,
        init: r.init,
        flags,
        predicate_container: ann.predicate_container.clone(),
        completion,
        jsx_element,
        expression_statement_container,
    };
    record_elapsed_ns(&T_BUILD_NS, t);
    out
}
