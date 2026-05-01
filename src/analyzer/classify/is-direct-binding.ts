import { AST_TYPE } from "../../ast-type.js";
export function isDirectBinding(t: string, key: string | null): boolean {
  if (t === AST_TYPE.VariableDeclarator && key === "id") {
    return true;
  }
  if (
    (t === AST_TYPE.FunctionDeclaration || t === AST_TYPE.FunctionExpression) &&
    key === "id"
  ) {
    return true;
  }
  if (
    (t === AST_TYPE.ClassDeclaration || t === AST_TYPE.ClassExpression) &&
    key === "id"
  ) {
    return true;
  }
  if (
    (t === AST_TYPE.FunctionDeclaration ||
      t === AST_TYPE.FunctionExpression ||
      t === AST_TYPE.ArrowFunctionExpression) &&
    key === "params"
  ) {
    return true;
  }
  if (t === AST_TYPE.CatchClause && key === "param") {
    return true;
  }
  if (
    (t === AST_TYPE.ImportSpecifier ||
      t === AST_TYPE.ImportDefaultSpecifier ||
      t === AST_TYPE.ImportNamespaceSpecifier) &&
    key === "local"
  ) {
    return true;
  }
  return false;
}
