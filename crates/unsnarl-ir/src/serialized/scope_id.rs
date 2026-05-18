//! String ID for a serialized Scope row. Ports
//! `ts/src/ir/serialized/scope-id.ts`.

use serde::Serialize;

use crate::filled_string::FilledString;

#[derive(Serialize)]
#[serde(transparent)]
pub struct SerializedScopeId(FilledString);

impl SerializedScopeId {
    pub fn new(value: FilledString) -> Self {
        Self(value)
    }
}
