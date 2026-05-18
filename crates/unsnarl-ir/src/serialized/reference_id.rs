//! String ID for a serialized Reference row.

use serde::Serialize;

#[derive(Serialize)]
#[serde(transparent)]
pub struct SerializedReferenceId(String);

impl SerializedReferenceId {
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        assert!(!value.is_empty(), "SerializedReferenceId must be non-empty");
        Self(value)
    }
}
