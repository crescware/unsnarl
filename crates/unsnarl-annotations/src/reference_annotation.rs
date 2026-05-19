//! Side-table row for `ReferenceData`.
//!
//! Field order matches the source interface (`owners`, `flags`,
//! `predicateContainer`, `completion`, `jsxElement`,
//! `expressionStatementContainer`).
//!
//! `Serialize` is intentionally not derived on `ReferenceAnnotation`.
//! Three fields (`completion`, `jsx_element`,
//! `expression_statement_container`) reference `unsnarl-ir` types
//! whose in-memory shape carries `SourceOffset` while the
//! pipeline-emitted shape carries `Span` (the conversion happens
//! inside `SerializedReference` at serialize time). Deriving
//! `Serialize` here would therefore force `Serialize` impls on
//! `ReferenceCompletion`, `JsxElementContainer`, and
//! `ExpressionStatementContainer` that no pipeline path consumes
//! and that produce a second JSON form different from the
//! `SerializedReference` output -- a synthetic representation that
//! conflicts with the workspace derive policy (`docs/derives.md`).
//! Field order is preserved by struct declaration order; the inner
//! `ReferenceAnnotationFlags` retains its `Serialize` derive and
//! field-order test because its in-memory shape matches its
//! pipeline-emitted counterpart inside `SerializedFlags`.
//!
//! `owners` holds `VariableId` rather than borrowed `&Variable`
//! values to keep the IR `'a`-free (Step 7 decision, #116:
//! "Lifetime: IR は lifetime-free").

use serde::Serialize;

use unsnarl_ir::reference::{
    ExpressionStatementContainer, JsxElementContainer, PredicateContainer, ReferenceCompletion,
};
use unsnarl_ir::VariableId;

#[derive(Serialize)]
pub struct ReferenceAnnotationFlags {
    pub call: bool,
    pub receiver: bool,
}

pub struct ReferenceAnnotation {
    pub owners: Vec<VariableId>,
    pub flags: ReferenceAnnotationFlags,
    pub predicate_container: Option<PredicateContainer>,
    pub completion: ReferenceCompletion,
    pub jsx_element: Option<JsxElementContainer>,
    pub expression_statement_container: Option<ExpressionStatementContainer>,
}

#[cfg(test)]
#[path = "reference_annotation_test.rs"]
mod reference_annotation_test;
