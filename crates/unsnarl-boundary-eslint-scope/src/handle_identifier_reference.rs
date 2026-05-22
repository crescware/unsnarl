//! Classify and bind an identifier reference, then notify the
//! external visitor through `AnalysisVisitor::on_reference`,
//! materialising the internal `AstKind<'a>` parent / path into
//! lifetime-free `AstNode` form so `'a` does not leak past the
//! boundary crate.

use oxc_ast::AstKind;

use unsnarl_ir::primitive::AstIdentifier;
use unsnarl_oxc_parity::AstType;

use crate::classify::classify_identifier::classify_identifier;
use crate::classify::classify_result::ClassifyResult;
use crate::materialise::{ast_node_of, materialise_path};
use crate::resolve::bind_reference;
use crate::state::{current_scope, ScopeBuilderState};
use crate::visitor::AnalysisVisitor;
use crate::walk::PathEntry;

#[allow(clippy::too_many_arguments)]
pub(crate) fn handle_identifier_reference(
    state: &mut ScopeBuilderState,
    visitor: &mut dyn AnalysisVisitor,
    parent: Option<&AstKind<'_>>,
    key: Option<&'static str>,
    path: &[PathEntry<'_>],
    name: &str,
    span: oxc_span::Span,
    ast_type: AstType,
) {
    let result = classify_identifier(parent, key, path);
    let ClassifyResult::Reference { flags, init } = result else {
        return;
    };
    let scope = current_scope(state);
    let identifier = AstIdentifier::new(ast_type, name.to_string(), span);
    let ref_id = bind_reference(state, scope, identifier, flags, init);
    let parent_node = parent.map(ast_node_of);
    let path_materialised = materialise_path(path);
    visitor.on_reference(
        ref_id,
        parent_node.as_ref(),
        key,
        &path_materialised,
        scope,
        state,
    );
}

#[cfg(test)]
#[path = "handle_identifier_reference_test.rs"]
mod handle_identifier_reference_test;
