//! Top-level on-disk IR shape.
//!
//! `SERIALIZED_IR_VERSION` and `SerializedIrVersion` live in this
//! file rather than their own module: `SerializedIR.version` is the
//! sole consumer, so promoting a single `u32` constant to its own
//! module would add a path without a corresponding cohesion gain.

use serde::Serialize;

use crate::diagnostic::Diagnostic;
use crate::language::Language;
use crate::serialized::serialized_reference::SerializedReference;
use crate::serialized::serialized_scope::SerializedScope;
use crate::serialized::serialized_variable::SerializedVariable;
use crate::serialized::variable_id::SerializedVariableId;

/// On-disk schema version of [`SerializedIR`]. Bump every time the
/// serialized shape changes so downstream consumers can switch on it.
pub const SERIALIZED_IR_VERSION: SerializedIrVersion = SerializedIrVersion(1);

/// Newtype over `u32` so a schema version cannot be confused with
/// other 32-bit IR scalars (source offsets, depth counters, flag
/// bits, ...). `#[serde(transparent)]` keeps the on-disk JSON shape
/// a bare number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct SerializedIrVersion(pub u32);

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
