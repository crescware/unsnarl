//! String ID for a serialized Scope row.

use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(transparent)]
pub struct SerializedScopeId(String);

impl SerializedScopeId {
    pub fn new(value: String) -> Self {
        assert!(!value.is_empty(), "SerializedScopeId must be non-empty");
        Self(value)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
