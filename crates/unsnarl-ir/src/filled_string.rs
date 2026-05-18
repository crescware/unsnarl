//! Branded non-empty `String`. Ports `ts/src/util/filled-string.ts`.
//!
//! Although the TS counterpart lives under `util/`, in Rust this type sits
//! in `unsnarl-ir` because both the IR contract types (in this crate) and
//! later crates depend on it, and `unsnarl-ir` is the bottom of the
//! dependency graph.
//!
//! `new` is infallible: every in-tree producer (the boundary that
//! pulls identifier names out of the parser AST, the serializer that
//! synthesises string IDs, etc.) is required by contract to pass a
//! non-empty string. The invariant is checked unconditionally with
//! `assert!` — a violation is a producer bug we want to catch in
//! release too, and `is_empty()` is an O(1) length read, so eliding
//! the check would buy nothing. Returning `Result` here would force
//! every caller to handle a case that can't happen under a correct
//! producer and would dilute the brand.

use serde::Serialize;

#[derive(Serialize)]
#[serde(transparent)]
pub struct FilledString(String);

impl FilledString {
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        assert!(!value.is_empty(), "FilledString must be non-empty");
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
