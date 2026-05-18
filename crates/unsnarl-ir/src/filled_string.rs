//! Branded non-empty `String`. Ports `ts/src/util/filled-string.ts`.
//!
//! Although the TS counterpart lives under `util/`, in Rust this type sits
//! in `unsnarl-ir` because both the IR contract types (in this crate) and
//! later crates depend on it, and `unsnarl-ir` is the bottom of the
//! dependency graph.

use serde::Serialize;

#[derive(Serialize)]
#[serde(transparent)]
pub struct FilledString(String);

impl FilledString {
    pub fn new(value: impl Into<String>) -> Result<Self, FilledStringError> {
        let value = value.into();
        if value.is_empty() {
            return Err(FilledStringError::Empty);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub enum FilledStringError {
    Empty,
}
