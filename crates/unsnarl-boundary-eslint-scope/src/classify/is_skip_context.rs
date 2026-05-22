//! Slots where an identifier is purely structural (an ESTree
//! property name, a label, etc.) and must not be treated as a
//! reference.
//!
//! `MemberExpression` is split into `Static` / `Computed` /
//! `PrivateField` variants in oxc; the `"property"` skip rule
//! applies only to the *static* form (the computed form keeps the
//! property as a real expression). `ObjectProperty` is oxc's name
//! for the ESTree `Property` node.

use oxc_ast::AstKind;

use crate::classify::is_computed::is_computed;

pub(crate) fn is_skip_context(parent: &AstKind<'_>, key: Option<&'static str>) -> bool {
    match parent {
        AstKind::ImportSpecifier(_) if key == Some("imported") => true,
        AstKind::ExportSpecifier(_) if key == Some("exported") => true,
        AstKind::StaticMemberExpression(_) | AstKind::PrivateFieldExpression(_)
            if key == Some("property") =>
        {
            true
        }
        AstKind::ObjectProperty(_)
        | AstKind::MethodDefinition(_)
        | AstKind::PropertyDefinition(_)
        | AstKind::AccessorProperty(_)
            if key == Some("key") && !is_computed(parent) =>
        {
            true
        }
        AstKind::JSXAttribute(_) if key == Some("name") => true,
        AstKind::JSXMemberExpression(_) if key == Some("property") => true,
        AstKind::JSXClosingElement(_) => true,
        AstKind::LabeledStatement(_)
        | AstKind::ContinueStatement(_)
        | AstKind::BreakStatement(_)
            if key == Some("label") =>
        {
            true
        }
        _ => false,
    }
}

#[cfg(test)]
#[path = "is_skip_context_test.rs"]
mod is_skip_context_test;
