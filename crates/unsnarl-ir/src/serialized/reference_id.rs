//! String ID for a serialized Reference row.

use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(transparent)]
pub struct SerializedReferenceId(String);

impl SerializedReferenceId {
    pub fn new(value: String) -> Self {
        assert!(!value.is_empty(), "SerializedReferenceId must be non-empty");
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
