//! Visitor callback consumed by [`crate::analyze::analyze`].
//!
//! The only callback the adapter dispatches is
//! [`AnalysisVisitor::on_diagnostic`] (currently
//! [`unsnarl_ir::diagnostic_kind::DiagnosticKind::VarDetected`]).

use unsnarl_ir::diagnostic::Diagnostic;

pub trait AnalysisVisitor {
    fn on_diagnostic(&mut self, _diag: &Diagnostic) {}
}
