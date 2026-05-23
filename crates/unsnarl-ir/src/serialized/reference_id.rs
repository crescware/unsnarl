//! String ID for a serialized Reference row.
//!
//! Internally stores an `Arc<str>` so that the heavy clone traffic
//! through `serialize_reference` / `serialize_variable` /
//! `serialize_scope` is reduced to an atomic refcount bump rather
//! than a fresh heap allocation per id. The newtype keeps the ID's
//! identity visible at every call site; consumers never touch the
//! `Arc<str>` directly.

use std::sync::Arc;

use serde::{Serialize, Serializer};

#[derive(Clone)]
pub struct SerializedReferenceId(Arc<str>);

impl SerializedReferenceId {
    pub fn new(value: String) -> Self {
        assert!(!value.is_empty(), "SerializedReferenceId must be non-empty");
        Self(Arc::from(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Serialize for SerializedReferenceId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}
