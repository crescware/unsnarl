//! Visitor callbacks consumed by [`crate::analyze::analyze`].
//!
//! The external callback shape uses materialised `AstNode`
//! (`type` + `span`) for `parent` / `path`, so the boundary's
//! internal `AstKind<'a>` walk-time form does not leak the `'a`
//! lifetime past the trait surface.
//!
//! All three callbacks default to a no-op so consumers can implement
//! only the slots they care about (e.g. the Step 11
//! `build-analysis-visitor` will hook `on_reference` and
//! `on_diagnostic` but ignore `on_scope`).

use unsnarl_ir::diagnostic::Diagnostic;
use unsnarl_ir::ids::{ReferenceId, ScopeId};
use unsnarl_ir::primitive::AstNode;

use crate::state::ScopeBuilderState;

pub trait AnalysisVisitor {
    fn on_scope(
        &mut self,
        _scope_id: ScopeId,
        _parent: Option<&AstNode>,
        _key: Option<&str>,
        _path: &[AstNode],
        _state: &ScopeBuilderState,
    ) {
    }

    fn on_reference(
        &mut self,
        _ref_id: ReferenceId,
        _parent: Option<&AstNode>,
        _key: Option<&str>,
        _path: &[AstNode],
        _scope_id: ScopeId,
        _state: &ScopeBuilderState,
    ) {
    }

    fn on_diagnostic(&mut self, _diag: &Diagnostic) {}
}
