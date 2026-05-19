//! Convenience constructor for `ClassifyResult::Reference`.
//!
//! Mirrors `reference` in `classify/reference.ts`.

use unsnarl_ir::reference::reference_flags::ReferenceFlagBits;

use crate::classify::classify_result::ClassifyResult;

pub(crate) fn reference(flags: ReferenceFlagBits, init: bool) -> ClassifyResult {
    ClassifyResult::Reference { flags, init }
}

#[cfg(test)]
#[path = "reference_test.rs"]
mod reference_test;
