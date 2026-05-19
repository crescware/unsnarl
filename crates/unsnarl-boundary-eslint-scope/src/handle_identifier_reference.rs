//! Classify and bind an identifier reference.
//!
//! Mirrors `handleIdentifierReference` in
//! `ts/src/boundary/eslint-scope/handle-identifier-reference.ts`.
//! Step 9.7 will additionally call `visitor.on_reference?.(...)` once
//! `AnalysisVisitor::on_reference` lands; the binding pathway itself
//! is fully wired here.

use oxc_ast::AstKind;

use unsnarl_ir::ids::{ReferenceId, ScopeId};
use unsnarl_ir::primitive::AstIdentifier;
use unsnarl_oxc_parity::AstType;

use crate::classify::classify_identifier::classify_identifier;
use crate::classify::classify_result::ClassifyResult;
use crate::resolve::bind_reference;
use crate::state::{current_scope, ScopeBuilderState};
use crate::walk::PathEntry;

pub(crate) fn handle_identifier_reference(
    state: &mut ScopeBuilderState,
    parent: Option<&AstKind<'_>>,
    key: Option<&'static str>,
    path: &[PathEntry<'_>],
    name: &str,
    span: oxc_span::Span,
    ast_type: AstType,
) -> Option<(ReferenceId, ScopeId)> {
    let result = classify_identifier(parent, key, path);
    let ClassifyResult::Reference { flags, init } = result else {
        return None;
    };
    let scope = current_scope(state);
    let identifier = AstIdentifier::new(ast_type, name.to_string(), span);
    let ref_id = bind_reference(state, scope, identifier, flags, init);
    Some((ref_id, scope))
}
