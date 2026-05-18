//! Serialized counterpart of `Reference`. Ports
//! `ts/src/ir/serialized/serialized-reference.ts`.

use serde::Serialize;

use crate::filled_string::FilledString;
use crate::primitive::Span;
use crate::reference::predicate_container::PredicateContainer;
use crate::serialized::reference_id::SerializedReferenceId;
use crate::serialized::scope_id::SerializedScopeId;
use crate::serialized::serialized_expression_statement_head::SerializedHeadExpression;
use crate::serialized::variable_id::SerializedVariableId;

#[derive(Serialize)]
pub struct SerializedReferenceIdentifier {
    pub name: FilledString,
    pub span: Span,
}

/// 4-bool flag block. The internal IR's `flags` bitmask covers only
/// read / write; `call` and `receiver` come from the
/// `ReferenceAnnotation` side table (ported in Step 10) and are folded
/// in at serialization time.
#[derive(Serialize)]
pub struct SerializedFlags {
    pub read: bool,
    pub write: bool,
    pub call: bool,
    pub receiver: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedJsxElement {
    pub start_span: Span,
    pub end_span: Span,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedExpressionStatementContainer {
    pub start_span: Span,
    pub end_span: Span,
    pub head: SerializedHeadExpression,
}

/// Reference-side completion in serialized (span-based) form.
/// Narrowed to `normal` / `return` / `throw` for the same reason
/// `ReferenceCompletion` is narrowed (see
/// `reference/reference-completion.rs`).
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SerializedCompletion {
    Normal,
    #[serde(rename_all = "camelCase")]
    Return {
        start_span: Span,
        end_span: Span,
    },
    #[serde(rename_all = "camelCase")]
    Throw {
        start_span: Span,
        end_span: Span,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedReference {
    pub id: SerializedReferenceId,
    pub identifier: SerializedReferenceIdentifier,
    pub from: SerializedScopeId,
    pub resolved: Option<SerializedVariableId>,
    pub owners: Vec<SerializedVariableId>,
    pub init: bool,
    pub flags: SerializedFlags,
    pub predicate_container: Option<PredicateContainer>,
    pub completion: SerializedCompletion,
    pub jsx_element: Option<SerializedJsxElement>,
    pub expression_statement_container: Option<SerializedExpressionStatementContainer>,
}
