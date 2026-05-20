//! `UpdateOperator`: the 2 update operators ECMA defines for
//! `UpdateExpression.operator`.
//!
//! Placed in `unsnarl-oxc-parity` for the same reason as
//! `AssignOperator`: values come directly from oxc's
//! `UpdateExpression.operator` (TS
//! `analyzer/expression-statement-head.ts:149`). The set is fixed at
//! `++` and `--` in current ECMA, so the JSON shape is pinned by
//! `#[serde(rename = "...")]`.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum UpdateOperator {
    #[serde(rename = "++")]
    Increment,
    #[serde(rename = "--")]
    Decrement,
}

impl UpdateOperator {
    /// Source spelling of the operator (`"++"` / `"--"`). Used by
    /// `render_head_expression` to format the operator inside a
    /// rendered update label.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Increment => "++",
            Self::Decrement => "--",
        }
    }
}
