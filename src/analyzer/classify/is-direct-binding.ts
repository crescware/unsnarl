export function isDirectBinding(t: string, key: string | null): boolean {
  if (t === "VariableDeclarator" && key === "id") {
    return true;
  }
  if (
    (t === "FunctionDeclaration" || t === "FunctionExpression") &&
    key === "id"
  ) {
    return true;
  }
  if ((t === "ClassDeclaration" || t === "ClassExpression") && key === "id") {
    return true;
  }
  if (
    (t === "FunctionDeclaration" ||
      t === "FunctionExpression" ||
      t === "ArrowFunctionExpression") &&
    key === "params"
  ) {
    return true;
  }
  if (t === "CatchClause" && key === "param") {
    return true;
  }
  if (
    (t === "ImportSpecifier" ||
      t === "ImportDefaultSpecifier" ||
      t === "ImportNamespaceSpecifier") &&
    key === "local"
  ) {
    return true;
  }
  return false;
}
