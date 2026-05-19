//! Nesting-depth tracking.
//!
//! `NestingDepths` lives in `unsnarl-ir` (rather than
//! `unsnarl-annotations`) because `SerializedScope` embeds it directly
//! and `unsnarl-ir` cannot depend on `unsnarl-annotations`.

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NestingKind {
    Function,
    If,
    For,
    While,
    Switch,
    TryCatchFinally,
    Block,
}

/// Nesting count for a single `NestingKind`. Newtype over `u32` so
/// depths cannot be confused with source offsets / line numbers /
/// other 32-bit IR counters. `#[serde(transparent)]` keeps the
/// on-disk JSON shape a bare number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct NestingDepth(pub u32);

#[derive(Serialize)]
pub struct NestingDepths {
    pub function: NestingDepth,
    pub r#if: NestingDepth,
    pub r#for: NestingDepth,
    pub r#while: NestingDepth,
    pub switch: NestingDepth,
    #[serde(rename = "try-catch-finally")]
    pub try_catch_finally: NestingDepth,
    pub block: NestingDepth,
}

impl NestingDepths {
    /// Build a `NestingDepths` with the same depth across all
    /// `NestingKind`s. The argument is `NestingDepth` (not bare `u32`)
    /// because relaxing it to `u32` would let any 32-bit scalar
    /// (offsets, line numbers, version) flow into the function and
    /// silently become a depth -- defeating the point of the newtype.
    pub fn uniform(value: NestingDepth) -> Self {
        Self {
            function: value,
            r#if: value,
            r#for: value,
            r#while: value,
            switch: value,
            try_catch_finally: value,
            block: value,
        }
    }
}

#[cfg(test)]
#[path = "nesting_kind_test.rs"]
mod nesting_kind_test;
