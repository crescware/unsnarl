//! Result type returned by [`crate::run_analysis::run_analysis`].
//!
//! Mirrors `AnalyzedSource` in `ts/src/pipeline/analyze/analyzed-source.ts`.
//! TS exposes `rootScope: Scope` because TS scopes carry their arena
//! identity through `Scope` references; the Rust port owns the
//! [`IrArena`] explicitly because downstream callers (notably
//! `FlatSerializer::serialize`) iterate scopes / variables / references /
//! definitions through `IndexVec<*Id, _>` rows on the arena rather than
//! by following node references.

use unsnarl_ir::diagnostic::Diagnostic;
use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::IrArena;

use crate::annotations_impl::AnnotationsImpl;

pub struct AnalyzedSource<'a> {
    pub arena: IrArena,
    pub root_scope: ScopeId,
    pub annotations: AnnotationsImpl,
    pub diagnostics: Vec<Diagnostic>,
    pub raw: &'a str,
}
