//! Per-scope record for `break` / `continue` statements.
//!
//! Issue #96 surfaces the ECMA §6.2.4 `[[Target]]` of `break` /
//! `continue` in visual output. `return` / `throw` already have a
//! visualization path through `SerializedReference.completion`
//! because they carry an argument identifier; `break` / `continue`
//! carry only a label (which is not an identifier reference per
//! eslint-scope's classification), so the IR needs a separate slot
//! that lives on the enclosing scope.
//!
//! `target` is `Some(name)` for labelled forms (`break outer`,
//! `continue outer`) and `None` for bare forms (`break`, `continue`).

use serde::Serialize;

use crate::primitive::Span;

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AbruptStatementType {
    Break,
    Continue,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbruptStatement {
    pub r#type: AbruptStatementType,
    pub target: Option<String>,
    pub span: Span,
    pub end_span: Span,
}
