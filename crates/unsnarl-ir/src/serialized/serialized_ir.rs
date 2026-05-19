//! Top-level on-disk IR shape.
//!
//! `SERIALIZED_IR_VERSION` and `SerializedIrVersion` live in this
//! file rather than their own module. `SerializedIR.version` is the
//! sole consumer; promoting a single `u32` constant to its own
//! module mirrored a TS file-per-export habit that gives no benefit
//! in Rust. The TS-side `diagnostic/diagnostic.ts` was flattened
//! into `diagnostic.rs` at the parent level for the same reason
//! ("子が 1 ファイルのみだったため親に平滑化"); this is the same
//! call applied one level down.

use serde::Serialize;

use crate::diagnostic::Diagnostic;
use crate::language::Language;
use crate::serialized::serialized_reference::SerializedReference;
use crate::serialized::serialized_scope::SerializedScope;
use crate::serialized::serialized_variable::SerializedVariable;
use crate::serialized::variable_id::SerializedVariableId;

/// On-disk schema version of [`SerializedIR`]. Bump every time the
/// serialized shape changes so downstream consumers can switch on it.
pub const SERIALIZED_IR_VERSION: u32 = 1;

pub type SerializedIrVersion = u32;

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
