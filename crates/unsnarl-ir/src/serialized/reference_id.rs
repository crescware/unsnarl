//! String ID for a serialized Reference row.

use serde::Serialize;

use crate::filled_string::FilledString;

#[derive(Serialize)]
#[serde(transparent)]
pub struct SerializedReferenceId(FilledString);

impl SerializedReferenceId {
    pub fn new(value: FilledString) -> Self {
        Self(value)
    }
}
