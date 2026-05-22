//! `PredicateContainerType`: the seven statement types that own a
//! predicate (the test expression of `if`, `switch`, `while`,
//! `do`-`while`, and the three `for` forms).
//!
//! Placed in `unsnarl-oxc-parity`, not `unsnarl-ir`, because the
//! variant strings are a curated subset of `AstType` values and
//! must stay value-for-value aligned with what oxc emits for those
//! statement nodes; the change driver is the same as `AstType`'s,
//! so the type belongs in the same crate.
//!
//! An "use `AstType` directly + assert! on construction" model
//! (mirroring the `FilledString` correction) was considered and
//! rejected. Downstream consumers (visual-graph routing, etc.)
//! dispatch on this discriminator, and an exhaustive `match` over
//! 7 variants gives stronger compile-time coverage than a runtime
//! assert plus a `match` over the ~150-variant `AstType`.

use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum PredicateContainerType {
    IfStatement,
    SwitchStatement,
    WhileStatement,
    DoWhileStatement,
    ForStatement,
    ForOfStatement,
    ForInStatement,
}
