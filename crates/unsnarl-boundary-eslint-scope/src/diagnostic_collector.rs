//! Mutable bag of `Diagnostic` values used by the scope builder during
//! `analyze`.
//!
//! Mirrors `DiagnosticCollector` in `ts/src/util/diagnostic.ts`. The TS
//! version exposes `add` / `list`; the Rust port mirrors that shape
//! but lowers `list` to `into_list` because the build state is owned
//! by `ScopeBuilderState` and the diagnostics escape the boundary
//! crate only once at `finish` time.

use unsnarl_ir::diagnostic::Diagnostic;
use unsnarl_ir::diagnostic_kind::DiagnosticKind;
use unsnarl_ir::primitive::Span;

pub(crate) struct DiagnosticCollector {
    items: Vec<Diagnostic>,
}

impl DiagnosticCollector {
    pub(crate) fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub(crate) fn add(&mut self, kind: DiagnosticKind, message: String, span: Span) {
        assert!(!message.is_empty(), "Diagnostic message must be non-empty");
        self.items.push(Diagnostic {
            kind,
            message,
            span,
        });
    }

    pub(crate) fn into_list(self) -> Vec<Diagnostic> {
        self.items
    }
}
