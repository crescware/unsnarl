//! Hoist each import specifier's `local` binding into the enclosing
//! scope.
//!
//! Iterates
//! `ImportDeclaration.specifiers: Option<Vec<ImportDeclarationSpecifier>>`
//! and reads each variant's `local: BindingIdentifier` field
//! directly.
//!
//! Each declared local sees the *spec* node as its `def_node` and
//! the *import* node as its `parent` (ESTree shape).

use oxc_ast::ast::{ImportDeclaration, ImportDeclarationSpecifier, ModuleExportName};

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::AstType;

use crate::state::{declare_variable_with_extras, DefinitionExtras, ScopeBuilderState};

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
    let import_source = import.source.value.as_str().to_string();
    for spec in specifiers {
        let (local, spec_node, imported_name) = match spec {
            ImportDeclarationSpecifier::ImportSpecifier(s) => {
                let imported = module_export_name(&s.imported);
                (
                    &s.local,
                    AstNode {
                        r#type: AstType::ImportSpecifier,
                        span: s.span,
                    },
                    Some(imported),
                )
            }
            ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => (
                &s.local,
                AstNode {
                    r#type: AstType::ImportDefaultSpecifier,
                    span: s.span,
                },
                None,
            ),
            ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => (
                &s.local,
                AstNode {
                    r#type: AstType::ImportNamespaceSpecifier,
                    span: s.span,
                },
                None,
            ),
        };
        let identifier = AstIdentifier::new(
            AstType::Identifier,
            local.name.as_str().to_string(),
            local.span,
        );
        declare_variable_with_extras(
            state,
            scope,
            identifier,
            DefinitionType::ImportBinding,
            spec_node,
            Some(import_node.clone()),
            DefinitionExtras {
                import_source: Some(import_source.clone()),
                imported_name,
                ..DefinitionExtras::default()
            },
        );
    }
}

fn module_export_name(name: &ModuleExportName<'_>) -> String {
    match name {
        ModuleExportName::IdentifierName(n) => n.name.as_str().to_string(),
        ModuleExportName::IdentifierReference(n) => n.name.as_str().to_string(),
        ModuleExportName::StringLiteral(n) => n.value.as_str().to_string(),
    }
}

#[cfg(test)]
#[path = "handle_import_declaration_test.rs"]
mod handle_import_declaration_test;
