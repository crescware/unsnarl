//! String ID for a serialized Variable row.

use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(transparent)]
pub struct SerializedVariableId(String);

impl SerializedVariableId {
    pub fn new(value: String) -> Self {
        assert!(!value.is_empty(), "SerializedVariableId must be non-empty");
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
