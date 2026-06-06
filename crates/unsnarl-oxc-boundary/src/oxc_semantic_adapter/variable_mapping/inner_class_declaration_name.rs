//! Detect a named class *declaration* and return its inner-name spans.

use oxc_ast::ast::ClassType;
use oxc_ast::AstKind;

/// If `anchor` is the `Class` node of a named class *declaration*,
/// return `(name, id_span, class_span)`. Class *expressions* already
/// receive an inner-name binding from `oxc_semantic`, so they return
/// `None` here.
pub(super) fn inner_class_declaration_name<'a>(
    anchor: &'a AstKind<'_>,
) -> Option<(&'a str, oxc_span::Span, oxc_span::Span)> {
    let AstKind::Class(class) = anchor else {
        return None;
    };
    if !matches!(class.r#type, ClassType::ClassDeclaration) {
        return None;
    }
    let id = class.id.as_ref()?;
    Some((id.name.as_str(), id.span, class.span))
}
