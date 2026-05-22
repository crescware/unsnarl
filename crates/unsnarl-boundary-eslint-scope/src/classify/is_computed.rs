//! Does this AST node have a `computed: true` flag?
//!
//! Matches on `AstKind` variants that carry a `computed: bool`
//! field; variants that don't carry one return `false`.
//! `ComputedMemberExpression` is always computed,
//! `StaticMemberExpression` / `PrivateFieldExpression` are always
//! not.

use oxc_ast::AstKind;

pub(crate) fn is_computed(node: &AstKind<'_>) -> bool {
    match node {
        AstKind::ComputedMemberExpression(_) => true,
        AstKind::StaticMemberExpression(_) | AstKind::PrivateFieldExpression(_) => false,
        AstKind::ObjectProperty(p) => p.computed,
        AstKind::BindingProperty(p) => p.computed,
        AstKind::MethodDefinition(m) => m.computed,
        AstKind::PropertyDefinition(p) => p.computed,
        AstKind::AccessorProperty(a) => a.computed,
        _ => false,
    }
}

#[cfg(test)]
#[path = "is_computed_test.rs"]
mod is_computed_test;
