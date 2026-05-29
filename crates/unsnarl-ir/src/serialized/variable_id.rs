//! String ID for a serialized Variable row.
//!
//! Internally stores an `Arc<str>` so that the heavy clone traffic
//! through `serialize_reference` / `serialize_variable` /
//! `serialize_scope` is reduced to an atomic refcount bump rather
//! than a fresh heap allocation per id. The newtype keeps the ID's
//! identity visible at every call site; consumers never touch the
//! `Arc<str>` directly.

use std::sync::Arc;

use serde::{Serialize, Serializer};

use crate::non_empty::assert_non_empty;

#[derive(Clone)]
pub struct SerializedVariableId(Arc<str>);

impl SerializedVariableId {
    pub fn new(value: impl Into<Arc<str>>) -> Self {
        let arc: Arc<str> = value.into();
        assert_non_empty(&arc, "SerializedVariableId");
        Self(arc)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Serialize for SerializedVariableId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}
