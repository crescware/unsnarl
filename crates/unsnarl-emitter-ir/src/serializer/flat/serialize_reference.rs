//! Serialize a `ReferenceData` into a `SerializedReference`.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use unsnarl_annotations::Annotations;
use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::reference::reference_flags::ReferenceFlags;
use unsnarl_ir::reference::ReferenceCompletion;
use unsnarl_ir::serialized::{
    SerializedCompletion, SerializedExpressionStatementContainer, SerializedFlags,
    SerializedJsxElement, SerializedReference, SerializedReferenceId,
    SerializedReferenceIdentifier, SerializedScopeId, SerializedVariableId,
};
use unsnarl_ir::{IrArena, ReferenceId, ScopeId, VariableId};

use crate::serializer::flat::serialize_expression_statement_head::serialize_head_expression;
use crate::serializer::flat::span_of::span_of_identifier;

// Per-sub-phase accumulators populated by `serialize_reference`.
// Caller drains them via `take_serialize_reference_stats()` so the
// totals can be emitted as a single tracing event after the
// `serialize_references` loop completes, instead of producing one
// span per call across tens of thousands of references.
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

/// Snapshot of the per-sub-phase totals accumulated by every
/// `serialize_reference` call since the last `take_*` call.
#[derive(Debug, Default)]
pub struct SerializeReferenceStats {
    pub lookup_ns: u64,
    pub annotations_ns: u64,
    pub owners_ns: u64,
    pub completion_ns: u64,
    pub jsx_ns: u64,
    pub expression_statement_container_ns: u64,
    pub expression_statement_head_ns: u64,
    pub identifier_ns: u64,
    pub build_ns: u64,
    pub owners_total: u64,
    pub return_count: u64,
    pub throw_count: u64,
    pub jsx_count: u64,
    pub expression_statement_count: u64,
}

/// Drain and return the accumulated `serialize_reference` sub-phase
/// counters, resetting them to zero so the next loop starts clean.
pub fn take_serialize_reference_stats() -> SerializeReferenceStats {
    SerializeReferenceStats {
        lookup_ns: T_LOOKUP_NS.swap(0, Ordering::Relaxed),
        annotations_ns: T_ANNOTATIONS_NS.swap(0, Ordering::Relaxed),
        owners_ns: T_OWNERS_NS.swap(0, Ordering::Relaxed),
        completion_ns: T_COMPLETION_NS.swap(0, Ordering::Relaxed),
        jsx_ns: T_JSX_NS.swap(0, Ordering::Relaxed),
        expression_statement_container_ns: T_EXPR_STMT_CONTAINER_NS.swap(0, Ordering::Relaxed),
        expression_statement_head_ns: T_EXPR_STMT_HEAD_NS.swap(0, Ordering::Relaxed),
        identifier_ns: T_IDENTIFIER_NS.swap(0, Ordering::Relaxed),
        build_ns: T_BUILD_NS.swap(0, Ordering::Relaxed),
        owners_total: N_OWNERS_TOTAL.swap(0, Ordering::Relaxed),
        return_count: N_RETURN.swap(0, Ordering::Relaxed),
        throw_count: N_THROW.swap(0, Ordering::Relaxed),
        jsx_count: N_JSX.swap(0, Ordering::Relaxed),
        expression_statement_count: N_EXPR_STMT.swap(0, Ordering::Relaxed),
    }
}

fn record(counter: &AtomicU64, t: Instant) {
    counter.fetch_add(t.elapsed().as_nanos() as u64, Ordering::Relaxed);
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
    let t = Instant::now();
    let r = &arena.references[reference];
    let id = reference_ids
        .get(&reference)
        .cloned()
        .unwrap_or_else(|| panic!("Reference id not found"));
    let from = scope_ids
        .get(&r.from)
        .cloned()
        .unwrap_or_else(|| panic!("Scope id not found for reference {}", r.identifier.name()));
    record(&T_LOOKUP_NS, t);

    let t = Instant::now();
    let ann = annotations.of_reference(reference);
    record(&T_ANNOTATIONS_NS, t);

    let t = Instant::now();
    let resolved = r.resolved.and_then(|v| variable_ids.get(&v).cloned());
    let owners: Vec<SerializedVariableId> = ann
        .owners
        .iter()
        .filter_map(|v| variable_ids.get(v).cloned())
        .collect();
    N_OWNERS_TOTAL.fetch_add(owners.len() as u64, Ordering::Relaxed);
    record(&T_OWNERS_NS, t);

    let read = (r.flags & ReferenceFlags::READ).0 != 0;
    let write = (r.flags & ReferenceFlags::WRITE).0 != 0;
    let flags = SerializedFlags {
        read,
        write,
        call: ann.flags.call,
        receiver: ann.flags.receiver,
    };

    let t = Instant::now();
    let completion = match &ann.completion {
        ReferenceCompletion::Normal => SerializedCompletion::Normal,
        ReferenceCompletion::Return {
            start_offset,
            end_offset,
        } => {
            N_RETURN.fetch_add(1, Ordering::Relaxed);
            SerializedCompletion::Return {
                start_span: index.span_at(*start_offset),
                end_span: index.span_at(*end_offset),
            }
        }
        ReferenceCompletion::Throw {
            start_offset,
            end_offset,
        } => {
            N_THROW.fetch_add(1, Ordering::Relaxed);
            SerializedCompletion::Throw {
                start_span: index.span_at(*start_offset),
                end_span: index.span_at(*end_offset),
            }
        }
    };
    record(&T_COMPLETION_NS, t);

    let t = Instant::now();
    let jsx_element = ann.jsx_element.as_ref().map(|jsx| {
        N_JSX.fetch_add(1, Ordering::Relaxed);
        SerializedJsxElement {
            start_span: index.span_at(jsx.start_offset),
            end_span: index.span_at(jsx.end_offset),
        }
    });
    record(&T_JSX_NS, t);

    let expression_statement_container = ann.expression_statement_container.as_ref().map(|c| {
        N_EXPR_STMT.fetch_add(1, Ordering::Relaxed);
        let t_container = Instant::now();
        let start_span = index.span_at(c.start_offset);
        let end_span = index.span_at(c.end_offset);
        record(&T_EXPR_STMT_CONTAINER_NS, t_container);
        let t_head = Instant::now();
        let head = serialize_head_expression(&c.head, index);
        record(&T_EXPR_STMT_HEAD_NS, t_head);
        SerializedExpressionStatementContainer {
            start_span,
            end_span,
            head,
        }
    });

    let t = Instant::now();
    let identifier = SerializedReferenceIdentifier::new(
        r.identifier.name().to_string(),
        span_of_identifier(&r.identifier, index),
    );
    record(&T_IDENTIFIER_NS, t);

    let t = Instant::now();
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
    record(&T_BUILD_NS, t);
    out
}
