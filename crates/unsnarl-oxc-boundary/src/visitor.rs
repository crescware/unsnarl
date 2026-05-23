//! Visitor callback consumed by [`crate::analyze::analyze`].
//!
//! After Phase 2's switch to the [`crate::oxc_semantic_adapter`],
//! the only callback the adapter dispatches is
//! [`AnalysisVisitor::on_diagnostic`] (currently
//! [`unsnarl_ir::diagnostic_kind::DiagnosticKind::VarDetected`]). The
//! pre-Phase-2 `on_scope` / `on_reference` callbacks fired by the
//! hand-rolled walker have no consumer in the workspace and were
//! removed alongside that walker.

use unsnarl_ir::diagnostic::Diagnostic;

pub trait AnalysisVisitor {
    fn on_diagnostic(&mut self, _diag: &Diagnostic) {}
}
