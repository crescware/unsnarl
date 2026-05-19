//! Side-table row for `VariableData`.
//!
//! Field order matches the source interface (`isUnused`). The
//! pipeline does not serialize this struct directly; the flat
//! serializer reads `is_unused` to decide whether to filter the
//! variable out of `SerializedVariable` emission. The `Serialize`
//! derive is in place so the on-disk field name matches the TS
//! shape and the field-order invariant is checked by the sibling
//! test.

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VariableAnnotation {
    pub is_unused: bool,
}

#[cfg(test)]
#[path = "variable_annotation_test.rs"]
mod variable_annotation_test;
