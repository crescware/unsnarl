//! Slots where the identifier IS the binding name (not a reference).
//!
//! Mirrors `isDirectBinding` in `classify/is-direct-binding.ts`. The
//! TS port matches on `node.type` strings; the Rust port matches on
//! `AstKind` variants. Same membership: `VariableDeclarator.id`,
//! `Function.id` / `Class.id`, `Function.params` / `Arrow.params`,
//! `CatchClause.param`, `ImportSpecifier.local` (incl. Default /
//! Namespace specifiers). oxc's `Function` covers both TS
//! `FunctionDeclaration` and `FunctionExpression`, and `Class` covers
//! both `ClassDeclaration` and `ClassExpression`.

use oxc_ast::AstKind;

pub(crate) fn is_direct_binding(parent: &AstKind<'_>, key: Option<&'static str>) -> bool {
    match parent {
        AstKind::VariableDeclarator(_) if key == Some("id") => true,
        AstKind::Function(_) if key == Some("id") || key == Some("params") => true,
        AstKind::ArrowFunctionExpression(_) if key == Some("params") => true,
        AstKind::Class(_) if key == Some("id") => true,
        AstKind::CatchClause(_) if key == Some("param") => true,
        AstKind::ImportSpecifier(_)
        | AstKind::ImportDefaultSpecifier(_)
        | AstKind::ImportNamespaceSpecifier(_)
            if key == Some("local") =>
        {
            true
        }
        _ => false,
    }
}

#[cfg(test)]
#[path = "is_direct_binding_test.rs"]
mod is_direct_binding_test;
