import { AST_TYPE } from "../../constants.js";
export function skipBlockScope(parentType: string): boolean {
  return (
    parentType === "FunctionDeclaration" ||
    parentType === "FunctionExpression" ||
    parentType === "ArrowFunctionExpression" ||
    parentType === AST_TYPE.CatchClause
  );
}
