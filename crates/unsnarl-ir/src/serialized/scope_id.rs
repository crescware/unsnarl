//! String ID for a serialized Scope row.

use serde::Serialize;

#[derive(Serialize)]
#[serde(transparent)]
pub struct SerializedScopeId(String);

impl SerializedScopeId {
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        assert!(!value.is_empty(), "SerializedScopeId must be non-empty");
        Self(value)
    }
}
