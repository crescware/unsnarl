//! Top-level on-disk IR shape.

use serde::Serialize;

use crate::diagnostic::Diagnostic;
use crate::language::Language;
use crate::serialized::serialized_reference::SerializedReference;
use crate::serialized::serialized_scope::SerializedScope;
use crate::serialized::serialized_variable::SerializedVariable;
use crate::serialized::variable_id::SerializedVariableId;
use crate::serialized_ir_version::SerializedIrVersion;

#[derive(Serialize)]
pub struct SerializedSource {
    pub path: String,
    pub language: Language,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedIR {
    pub version: SerializedIrVersion,
    pub source: SerializedSource,
    pub raw: String,
    pub scopes: Vec<SerializedScope>,
    pub variables: Vec<SerializedVariable>,
    pub references: Vec<SerializedReference>,
    pub unused_variable_ids: Vec<SerializedVariableId>,
    pub diagnostics: Vec<Diagnostic>,
}

#[cfg(test)]
#[path = "serialized_ir_test.rs"]
mod serialized_ir_test;
