import { AST_TYPE } from "../../ast-type.js";
import type { ScopeManager } from "../manager.js";
import type { NodeLike } from "./node-like.js";
import { skipBlockScope } from "./skip-block-scope.js";

export function handleLeave(
  node: NodeLike,
  parent: NodeLike | null,
  key: string | null,
  manager: ScopeManager,
): void {
  switch (node.type) {
    case AST_TYPE.FunctionDeclaration:
    case AST_TYPE.FunctionExpression:
    case AST_TYPE.ArrowFunctionExpression:
    case AST_TYPE.ForStatement:
    case AST_TYPE.ForOfStatement:
    case AST_TYPE.ForInStatement:
    case AST_TYPE.SwitchStatement:
    case AST_TYPE.SwitchCase:
    case AST_TYPE.CatchClause:
      manager.pop();
      return;
    case AST_TYPE.BlockStatement:
      if (parent && key === "body" && skipBlockScope(parent.type)) {
        return;
      }
      manager.pop();
      return;
    default:
      return;
  }
}
