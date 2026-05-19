//! Hoist each import specifier's `local` binding into the enclosing
//! scope.
//!
//! Mirrors `handleImportDeclaration` in
//! `ts/src/boundary/eslint-scope/hoisting/handle-import-declaration.ts`.
//! TS reads `node["specifiers"]` as an array of `NodeLike` and pulls
//! `spec["local"]`; the Rust port iterates
//! `ImportDeclaration.specifiers: Option<Vec<ImportDeclarationSpecifier>>`
//! and reads each variant's `local: BindingIdentifier` field directly.
//!
//! Each declared local sees the *spec* node as its `def_node` and the
//! *import* node as its `parent`, matching the TS shape exactly.

use oxc_ast::ast::{ImportDeclaration, ImportDeclarationSpecifier};

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::AstType;

use crate::state::{declare_variable, ScopeBuilderState};

pub(crate) fn handle_import_declaration(
    state: &mut ScopeBuilderState,
    scope: ScopeId,
    import: &ImportDeclaration<'_>,
) {
    let Some(specifiers) = import.specifiers.as_ref() else {
        return;
    };
    let import_node = AstNode {
        r#type: AstType::ImportDeclaration,
        span: import.span,
    };
    for spec in specifiers {
        let (local, spec_node) = match spec {
            ImportDeclarationSpecifier::ImportSpecifier(s) => (
                &s.local,
                AstNode {
                    r#type: AstType::ImportSpecifier,
                    span: s.span,
                },
            ),
            ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => (
                &s.local,
                AstNode {
                    r#type: AstType::ImportDefaultSpecifier,
                    span: s.span,
                },
            ),
            ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => (
                &s.local,
                AstNode {
                    r#type: AstType::ImportNamespaceSpecifier,
                    span: s.span,
                },
            ),
        };
        let identifier = AstIdentifier::new(
            AstType::Identifier,
            local.name.as_str().to_string(),
            local.span,
        );
        declare_variable(
            state,
            scope,
            identifier,
            DefinitionType::ImportBinding,
            spec_node,
            Some(import_node.clone()),
        );
    }
}

#[cfg(test)]
#[path = "handle_import_declaration_test.rs"]
mod handle_import_declaration_test;
