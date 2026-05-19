//! Convert internal `AstKind<'a>` walker state into the
//! lifetime-free `AstNode` form exposed to `AnalysisVisitor`.
//!
//! Per issue #118 comment 4 judgment C, the walker keeps
//! `Vec<PathEntry<'a>>` internally so classify routines can read
//! structural fields off the typed AST, but `AnalysisVisitor`
//! callbacks receive `&[AstNode]` (`type` + `span` only) so the
//! `'a` lifetime stays inside the boundary crate.
//!
//! The `AstType` reported here mirrors what the TS layer's
//! `asAstType()` would produce for the equivalent unnormalised
//! `NodeLike.type` string. oxc-side renames (e.g. `Function` →
//! `FunctionDeclaration` / `FunctionExpression`,
//! `StaticMemberExpression` → `MemberExpression`, `ObjectProperty`
//! → `Property`) are flattened here so downstream consumers see the
//! ESTree-style spelling the IR contract uses.

use oxc_ast::ast::{ClassType, FunctionType, MethodDefinitionType, PropertyDefinitionType};
use oxc_ast::AstKind;
use oxc_span::GetSpan;

use unsnarl_ir::primitive::AstNode;
use unsnarl_oxc_parity::{as_ast_type, AstType};

use crate::walk::PathEntry;

pub(crate) fn ast_type_of(kind: &AstKind<'_>) -> AstType {
    match kind {
        AstKind::Function(f) => match f.r#type {
            FunctionType::FunctionDeclaration | FunctionType::TSDeclareFunction => {
                AstType::FunctionDeclaration
            }
            FunctionType::FunctionExpression | FunctionType::TSEmptyBodyFunctionExpression => {
                AstType::FunctionExpression
            }
        },
        AstKind::Class(c) => match c.r#type {
            ClassType::ClassDeclaration => AstType::ClassDeclaration,
            ClassType::ClassExpression => AstType::ClassExpression,
        },
        AstKind::ObjectProperty(_) | AstKind::BindingProperty(_) => AstType::Property,
        AstKind::StaticMemberExpression(_)
        | AstKind::ComputedMemberExpression(_)
        | AstKind::PrivateFieldExpression(_) => AstType::MemberExpression,
        AstKind::BindingRestElement(_) | AstKind::FormalParameterRest(_) => AstType::RestElement,
        AstKind::IdentifierName(_)
        | AstKind::IdentifierReference(_)
        | AstKind::BindingIdentifier(_)
        | AstKind::LabelIdentifier(_) => AstType::Identifier,
        AstKind::MethodDefinition(m) => match m.r#type {
            MethodDefinitionType::MethodDefinition => AstType::MethodDefinition,
            MethodDefinitionType::TSAbstractMethodDefinition => AstType::TSAbstractMethodDefinition,
        },
        AstKind::PropertyDefinition(p) => match p.r#type {
            PropertyDefinitionType::PropertyDefinition => AstType::PropertyDefinition,
            PropertyDefinitionType::TSAbstractPropertyDefinition => {
                AstType::TSAbstractPropertyDefinition
            }
        },
        _ => {
            // Fallback: oxc's own `AstType` variant name matches
            // unsnarl's for the vast majority of nodes (every node
            // that isn't renamed above). The `Debug` round-trip is
            // not on a hot path — it runs once per `on_scope` /
            // `on_reference` callback, both of which fire at most
            // once per AST node.
            let oxc_ty = format!("{:?}", kind.ty());
            as_ast_type(&oxc_ty)
        }
    }
}

pub(crate) fn ast_node_of(kind: &AstKind<'_>) -> AstNode {
    AstNode {
        r#type: ast_type_of(kind),
        span: kind.span(),
    }
}

pub(crate) fn materialise_path(path: &[PathEntry<'_>]) -> Vec<AstNode> {
    path.iter().map(|p| ast_node_of(&p.node)).collect()
}

#[cfg(test)]
#[path = "materialise_test.rs"]
mod materialise_test;
