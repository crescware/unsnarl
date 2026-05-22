//! Slots where the identifier IS the binding name (not a reference).
//!
//! Membership: `VariableDeclarator.id`, `Function.id` / `Class.id`,
//! `Function.params` / `Arrow.params`, `CatchClause.param`,
//! `ImportSpecifier.local` (including Default / Namespace
//! specifiers). oxc's `Function` covers both `FunctionDeclaration`
//! and `FunctionExpression`, and `Class` covers both
//! `ClassDeclaration` and `ClassExpression`.

use oxc_ast::AstKind;

pub(crate) fn is_direct_binding(parent: &AstKind<'_>, key: Option<&'static str>) -> bool {
    match parent {
        AstKind::VariableDeclarator(_) if key == Some("id") => true,
        AstKind::Function(_) if key == Some("id") || key == Some("params") => true,
        AstKind::ArrowFunctionExpression(_) if key == Some("params") => true,
        AstKind::Class(_) if key == Some("id") => true,
        AstKind::CatchClause(_) if key == Some("param") => true,
        // oxc-specific: ESTree models `Function.params` as a flat
        // `Pattern[]`, so `function f(x)` reaches the binding
        // identifier with `(parent=Function, key="params")`. oxc
        // wraps each parameter in
        // `FormalParameter { pattern: BindingPattern }`, so the
        // non-destructured case ends with
        // `(parent=FormalParameter, key="pattern")`. Treat that slot
        // as a direct binding to produce the ESTree-equivalent
        // classify outcome.
        AstKind::FormalParameter(_) if key == Some("pattern") => true,
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
