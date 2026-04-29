const TS_TYPE_KEYS = new Set<string>([
  "typeAnnotation",
  "typeArguments",
  "typeParameters",
  "returnType",
  "superTypeArguments",
  "implements",
]);

const TS_PURE_TYPE_NODE_TYPES = new Set<string>([
  "TSInterfaceDeclaration",
  "TSTypeAliasDeclaration",
  "TSEnumDeclaration",
  "TSEnumBody",
  "TSEnumMember",
  "TSModuleDeclaration",
  "TSDeclareFunction",
  "TSAbstractMethodDefinition",
  "TSAbstractPropertyDefinition",
  "TSAbstractAccessorProperty",
  "TSImportEqualsDeclaration",
  "TSExportAssignment",
  "TSNamespaceExportDeclaration",
]);

export function isTypeOnlySubtree(
  nodeType: string,
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
