//! TS-only type-subtree detector.
//!
//! Mirrors `ts/src/analyzer/skip-types.ts`. Returns `true` when a node
//! (identified by its [`AstType`] and the key it occupies in its
//! parent) is a TypeScript type-only subtree the analyzer should skip
//! over.

use crate::AstType;

pub fn is_type_only_subtree(node_type: &AstType, key: Option<&str>) -> bool {
    if let Some(k) = key {
        if matches!(
            k,
            "typeAnnotation"
                | "typeArguments"
                | "typeParameters"
                | "returnType"
                | "superTypeArguments"
                | "implements"
        ) {
            return true;
        }
    }
    matches!(
        node_type,
        AstType::TSInterfaceDeclaration
            | AstType::TSTypeAliasDeclaration
            | AstType::TSEnumDeclaration
            | AstType::TSEnumBody
            | AstType::TSEnumMember
            | AstType::TSModuleDeclaration
            | AstType::TSDeclareFunction
            | AstType::TSAbstractMethodDefinition
            | AstType::TSAbstractPropertyDefinition
            | AstType::TSAbstractAccessorProperty
            | AstType::TSImportEqualsDeclaration
            | AstType::TSExportAssignment
            | AstType::TSNamespaceExportDeclaration
    )
}

#[cfg(test)]
#[path = "skip_types_test.rs"]
mod skip_types_test;
