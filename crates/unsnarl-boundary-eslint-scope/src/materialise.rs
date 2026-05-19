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

use oxc_ast::ast::{
    ClassType, Expression, FunctionType, MethodDefinitionType, PropertyDefinitionType,
};
use oxc_ast::AstKind;
use oxc_span::GetSpan;

use unsnarl_ir::primitive::AstNode;
use unsnarl_oxc_parity::{as_ast_type, AstType};

use crate::walk::PathEntry;

pub fn ast_type_of(kind: &AstKind<'_>) -> AstType {
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
        // oxc separates the function body wrapper (`FunctionBody`)
        // from the standalone `BlockStatement`, but the npm
        // `oxc-parser` emits both as `BlockStatement` (ESTree's
        // shape). Mirror the TS spelling so downstream consumers see
        // the function body as a BlockStatement parent.
        AstKind::FunctionBody(_) => AstType::BlockStatement,
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

pub fn ast_node_of(kind: &AstKind<'_>) -> AstNode {
    AstNode {
        r#type: ast_type_of(kind),
        span: kind.span(),
    }
}

pub(crate) fn materialise_path(path: &[PathEntry<'_>]) -> Vec<AstNode> {
    path.iter().map(|p| ast_node_of(&p.node)).collect()
}

/// Convert an `oxc_ast::Expression` to the boundary's lifetime-free
/// `AstNode` shape.
///
/// Mirrors the ESTree-style `node.type` strings the TS port consumes
/// (e.g. all numeric / string / boolean / null / regexp / bigint
/// literals collapse to `AstType::Literal`; member expressions and
/// private-in expressions collapse to their ESTree counterparts).
pub fn ast_node_of_expression(expr: &Expression<'_>) -> AstNode {
    let (ty, span) = match expr {
        Expression::BooleanLiteral(n) => (AstType::Literal, n.span),
        Expression::NullLiteral(n) => (AstType::Literal, n.span),
        Expression::NumericLiteral(n) => (AstType::Literal, n.span),
        Expression::BigIntLiteral(n) => (AstType::Literal, n.span),
        Expression::RegExpLiteral(n) => (AstType::Literal, n.span),
        Expression::StringLiteral(n) => (AstType::Literal, n.span),
        Expression::TemplateLiteral(n) => (AstType::TemplateLiteral, n.span),
        Expression::Identifier(n) => (AstType::Identifier, n.span),
        Expression::MetaProperty(n) => (AstType::MetaProperty, n.span),
        Expression::Super(n) => (AstType::Super, n.span),
        Expression::ArrayExpression(n) => (AstType::ArrayExpression, n.span),
        Expression::ArrowFunctionExpression(n) => (AstType::ArrowFunctionExpression, n.span),
        Expression::AssignmentExpression(n) => (AstType::AssignmentExpression, n.span),
        Expression::AwaitExpression(n) => (AstType::AwaitExpression, n.span),
        Expression::BinaryExpression(n) => (AstType::BinaryExpression, n.span),
        Expression::CallExpression(n) => (AstType::CallExpression, n.span),
        Expression::ChainExpression(n) => (AstType::ChainExpression, n.span),
        Expression::ClassExpression(n) => (AstType::ClassExpression, n.span),
        Expression::ConditionalExpression(n) => (AstType::ConditionalExpression, n.span),
        Expression::FunctionExpression(n) => (AstType::FunctionExpression, n.span),
        Expression::ImportExpression(n) => (AstType::ImportExpression, n.span),
        Expression::LogicalExpression(n) => (AstType::LogicalExpression, n.span),
        Expression::NewExpression(n) => (AstType::NewExpression, n.span),
        Expression::ObjectExpression(n) => (AstType::ObjectExpression, n.span),
        Expression::ParenthesizedExpression(n) => (AstType::ParenthesizedExpression, n.span),
        Expression::SequenceExpression(n) => (AstType::SequenceExpression, n.span),
        Expression::TaggedTemplateExpression(n) => (AstType::TaggedTemplateExpression, n.span),
        Expression::ThisExpression(n) => (AstType::ThisExpression, n.span),
        Expression::UnaryExpression(n) => (AstType::UnaryExpression, n.span),
        Expression::UpdateExpression(n) => (AstType::UpdateExpression, n.span),
        Expression::YieldExpression(n) => (AstType::YieldExpression, n.span),
        // `PrivateInExpression` (e.g. `#field in obj`) is rendered as
        // a `BinaryExpression` in ESTree-compatible output.
        Expression::PrivateInExpression(n) => (AstType::BinaryExpression, n.span),
        Expression::JSXElement(n) => (AstType::JSXElement, n.span),
        Expression::JSXFragment(n) => (AstType::JSXFragment, n.span),
        Expression::TSAsExpression(n) => (AstType::TSAsExpression, n.span),
        Expression::TSSatisfiesExpression(n) => (AstType::TSSatisfiesExpression, n.span),
        Expression::TSTypeAssertion(n) => (AstType::TSTypeAssertion, n.span),
        Expression::TSNonNullExpression(n) => (AstType::TSNonNullExpression, n.span),
        Expression::TSInstantiationExpression(n) => (AstType::TSInstantiationExpression, n.span),
        Expression::ComputedMemberExpression(n) => (AstType::MemberExpression, n.span),
        Expression::StaticMemberExpression(n) => (AstType::MemberExpression, n.span),
        Expression::PrivateFieldExpression(n) => (AstType::MemberExpression, n.span),
        Expression::V8IntrinsicExpression(n) => (AstType::V8IntrinsicExpression, n.span),
    };
    AstNode { r#type: ty, span }
}

#[cfg(test)]
#[path = "materialise_test.rs"]
mod materialise_test;
