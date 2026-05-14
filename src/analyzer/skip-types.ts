import { AST_TYPE, type AstType } from "../parser/ast-type.js";

const TS_TYPE_KEYS = new Set<string>([
  "typeAnnotation",
  "typeArguments",
  "typeParameters",
  "returnType",
  "superTypeArguments",
  "implements",
]);

const TS_PURE_TYPE_NODE_TYPES: ReadonlySet<AstType> = new Set<AstType>([
  AST_TYPE.TSInterfaceDeclaration,
  AST_TYPE.TSTypeAliasDeclaration,
  AST_TYPE.TSEnumDeclaration,
  AST_TYPE.TSEnumBody,
  AST_TYPE.TSEnumMember,
  AST_TYPE.TSModuleDeclaration,
  AST_TYPE.TSDeclareFunction,
  AST_TYPE.TSAbstractMethodDefinition,
  AST_TYPE.TSAbstractPropertyDefinition,
  AST_TYPE.TSAbstractAccessorProperty,
  AST_TYPE.TSImportEqualsDeclaration,
  AST_TYPE.TSExportAssignment,
  AST_TYPE.TSNamespaceExportDeclaration,
]);

export function isTypeOnlySubtree(
  nodeType: AstType,
  key: string | null,
): boolean {
  if (key !== null && TS_TYPE_KEYS.has(key)) {
    return true;
  }
  if (TS_PURE_TYPE_NODE_TYPES.has(nodeType)) {
    return true;
  }
  return false;
}
