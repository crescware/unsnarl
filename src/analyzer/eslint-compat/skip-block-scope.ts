export function skipBlockScope(parentType: string): boolean {
  return (
    parentType === "FunctionDeclaration" ||
    parentType === "FunctionExpression" ||
    parentType === "ArrowFunctionExpression" ||
    parentType === "CatchClause"
  );
}
