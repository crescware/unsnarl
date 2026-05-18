import { AST_TYPE, type AstType } from "../../parser/ast-type.js";
export function skipBlockScope(parentType: AstType): boolean {
  return (
    parentType === AST_TYPE.FunctionDeclaration ||
    parentType === AST_TYPE.FunctionExpression ||
    parentType === AST_TYPE.ArrowFunctionExpression ||
    parentType === AST_TYPE.CatchClause
  );
}
