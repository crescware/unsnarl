//! String ID for a serialized Variable row.

use serde::Serialize;

#[derive(Serialize)]
#[serde(transparent)]
pub struct SerializedVariableId(String);

impl SerializedVariableId {
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        assert!(!value.is_empty(), "SerializedVariableId must be non-empty");
        Self(value)
    }
}
