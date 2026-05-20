//! Result of classifying an identifier into a reference / binding /
//! skipped slot.
//!
//! Mirrors `ClassifyResult` in `classify/classify-result.ts`.

use unsnarl_ir::reference::reference_flags::ReferenceFlagBits;

pub(crate) enum ClassifyResult {
    Binding,
    Skip,
    Reference {
        flags: ReferenceFlagBits,
        init: bool,
    },
}
