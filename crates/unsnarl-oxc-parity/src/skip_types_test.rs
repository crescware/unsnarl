//! Unit tests for `is_type_only_subtree`.
//!
//! TS-side `skip-types.test.ts` is an integration test driven through
//! `runAnalysis` (a pipeline-layer helper not available until Step 21),
//! so it cannot be mirrored verbatim here. The pipeline-level
//! behaviour those tests assert (type aliases, interface bodies, enum
//! bodies, etc. are not visited as values) is recovered once Step 21
//! lands and the parity harness covers the same fixtures end-to-end.
//! Until then we cover only the decision function itself.

use crate::AstType;

use super::is_type_only_subtree;

#[test]
fn ts_type_only_keys_yield_true_regardless_of_node_type() {
    for key in [
        "typeAnnotation",
        "typeArguments",
        "typeParameters",
        "returnType",
        "superTypeArguments",
        "implements",
    ] {
        assert!(
            is_type_only_subtree(&AstType::Identifier, Some(key)),
            "key={key} should be type-only"
        );
    }
}

#[test]
fn pure_type_node_types_yield_true_regardless_of_key() {
    for ty in [
        AstType::TSInterfaceDeclaration,
        AstType::TSTypeAliasDeclaration,
        AstType::TSEnumDeclaration,
        AstType::TSEnumBody,
        AstType::TSEnumMember,
        AstType::TSModuleDeclaration,
        AstType::TSDeclareFunction,
        AstType::TSAbstractMethodDefinition,
        AstType::TSAbstractPropertyDefinition,
        AstType::TSAbstractAccessorProperty,
        AstType::TSImportEqualsDeclaration,
        AstType::TSExportAssignment,
        AstType::TSNamespaceExportDeclaration,
    ] {
        assert!(is_type_only_subtree(&ty, None));
        assert!(is_type_only_subtree(&ty, Some("body")));
    }
}

#[test]
fn other_value_node_types_with_unrelated_keys_are_not_type_only() {
    assert!(!is_type_only_subtree(&AstType::Identifier, None));
    assert!(!is_type_only_subtree(
        &AstType::FunctionDeclaration,
        Some("body")
    ));
    assert!(!is_type_only_subtree(
        &AstType::VariableDeclarator,
        Some("init")
    ));
}
