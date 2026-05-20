//! Serialize a `ReferenceData` into a `SerializedReference`.
//!
//! Mirrors `serializeReference` in
//! `ts/src/serializer/flat/serialize-reference.ts`.

use std::collections::HashMap;

use unsnarl_annotations::Annotations;
use unsnarl_ir::primitive::span_from_offset;
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

pub fn serialize_reference(
    arena: &IrArena,
    reference: ReferenceId,
    scope_ids: &HashMap<ScopeId, SerializedScopeId>,
    variable_ids: &HashMap<VariableId, SerializedVariableId>,
    reference_ids: &HashMap<ReferenceId, SerializedReferenceId>,
    annotations: &dyn Annotations,
    raw: &str,
) -> SerializedReference {
    let r = &arena.references[reference];
    let id = reference_ids
        .get(&reference)
        .cloned()
        .unwrap_or_else(|| panic!("Reference id not found"));
    let from = scope_ids
        .get(&r.from)
        .cloned()
        .unwrap_or_else(|| panic!("Scope id not found for reference {}", r.identifier.name()));
    let ann = annotations.of_reference(reference);
    let resolved = r.resolved.and_then(|v| variable_ids.get(&v).cloned());
    let owners = ann
        .owners
        .iter()
        .filter_map(|v| variable_ids.get(v).cloned())
        .collect();
    let read = (r.flags & ReferenceFlags::READ).0 != 0;
    let write = (r.flags & ReferenceFlags::WRITE).0 != 0;
    let flags = SerializedFlags {
        read,
        write,
        call: ann.flags.call,
        receiver: ann.flags.receiver,
    };
    let completion = match &ann.completion {
        ReferenceCompletion::Normal => SerializedCompletion::Normal,
        ReferenceCompletion::Return {
            start_offset,
            end_offset,
        } => SerializedCompletion::Return {
            start_span: span_from_offset(raw, start_offset.0 as usize),
            end_span: span_from_offset(raw, end_offset.0 as usize),
        },
        ReferenceCompletion::Throw {
            start_offset,
            end_offset,
        } => SerializedCompletion::Throw {
            start_span: span_from_offset(raw, start_offset.0 as usize),
            end_span: span_from_offset(raw, end_offset.0 as usize),
        },
    };
    let jsx_element = ann.jsx_element.as_ref().map(|jsx| SerializedJsxElement {
        start_span: span_from_offset(raw, jsx.start_offset.0 as usize),
        end_span: span_from_offset(raw, jsx.end_offset.0 as usize),
    });
    let expression_statement_container = ann.expression_statement_container.as_ref().map(|c| {
        SerializedExpressionStatementContainer {
            start_span: span_from_offset(raw, c.start_offset.0 as usize),
            end_span: span_from_offset(raw, c.end_offset.0 as usize),
            head: serialize_head_expression(&c.head, raw),
        }
    });
    SerializedReference {
        id,
        identifier: SerializedReferenceIdentifier::new(
            r.identifier.name().to_string(),
            span_of_identifier(&r.identifier, raw),
        ),
        from,
        resolved,
        owners,
        init: r.init,
        flags,
        predicate_container: ann.predicate_container.clone(),
        completion,
        jsx_element,
        expression_statement_container,
    }
}
