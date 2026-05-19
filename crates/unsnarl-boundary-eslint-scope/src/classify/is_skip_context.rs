//! Slots where an identifier is purely structural (an estree
//! property name, a label, etc.) and must not be treated as a
//! reference.
//!
//! Mirrors `isSkipContext` in `classify/is-skip-context.ts`. The TS
//! port uses string equality on `node.type`; the Rust port matches
//! on `AstKind` variants. `MemberExpression` is split into
//! `Static` / `Computed` / `PrivateField` variants in oxc; the
//! `"property"` skip rule applies only to the *static* form (the
//! TS `!isComputed(parent)` check).
//!
//! `ObjectProperty` covers what the TS `AST_TYPE.Property` arm
//! checks for; oxc renames the AST node so the type name differs.

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
