//! Predicate: should this scope (and its subtree) be dropped from the
//! IR scope tree?

use oxc_ast::AstKind;

/// Predicate: should this scope (and its entire subtree) be omitted
/// from the IR scope tree?
///
/// `oxc_semantic` allocates a scope for several TypeScript type-only
/// constructs (`type X = ...`, `interface X { ... }`, `namespace X
/// { ... }`, mapped / conditional types). The parity baseline never
/// emits a scope for them, so drop the scope's IR row outright.
/// Filtering propagates to descendants in the calling loop via the
/// inherited-filter check; the surrounding subtree is recognised via
/// [`unsnarl_oxc_parity::is_type_only_subtree`].
pub(super) fn is_filtered_out(kind: &AstKind<'_>) -> bool {
    if let AstKind::Function(func) = kind {
        if matches!(
            func.r#type,
            oxc_ast::ast::FunctionType::TSDeclareFunction
                | oxc_ast::ast::FunctionType::TSEmptyBodyFunctionExpression
        ) {
            return true;
        }
    }
    matches!(
        kind,
        AstKind::TSModuleDeclaration(_)
            | AstKind::TSTypeAliasDeclaration(_)
            | AstKind::TSInterfaceDeclaration(_)
            | AstKind::TSConditionalType(_)
            | AstKind::TSMappedType(_)
            | AstKind::TSEnumDeclaration(_)
            | AstKind::TSEnumBody(_)
            | AstKind::TSFunctionType(_)
            | AstKind::TSConstructorType(_)
            | AstKind::TSMethodSignature(_)
            | AstKind::TSConstructSignatureDeclaration(_)
            | AstKind::TSCallSignatureDeclaration(_)
    )
}
