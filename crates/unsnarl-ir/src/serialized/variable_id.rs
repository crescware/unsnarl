//! String ID for a serialized Variable row. Ports
//! `ts/src/ir/serialized/variable-id.ts`.

use serde::Serialize;

use crate::filled_string::FilledString;

#[derive(Serialize)]
#[serde(transparent)]
pub struct SerializedVariableId(FilledString);

impl SerializedVariableId {
    pub fn new(value: FilledString) -> Self {
        Self(value)
    }
}
