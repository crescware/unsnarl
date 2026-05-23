//! The subset of `Completion` that a `Reference`'s value can flow
//! into.
//!
//! `Break` / `Continue` accept only a label `Identifier`
//! syntactically; the scope build tracks labels separately from value
//! references, so a reference's value cannot flow into a break /
//! continue completion. The variant set is therefore narrowed to
//! `Normal` / `Return` / `Throw`.

use crate::primitive::SourceOffset;

pub enum ReferenceCompletion {
    Normal,
    Return {
        start_offset: SourceOffset,
        end_offset: SourceOffset,
    },
    Throw {
        start_offset: SourceOffset,
        end_offset: SourceOffset,
    },
}
