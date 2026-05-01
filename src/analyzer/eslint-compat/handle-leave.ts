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
    case "FunctionDeclaration":
    case "FunctionExpression":
    case "ArrowFunctionExpression":
    case "ForStatement":
    case "ForOfStatement":
    case "ForInStatement":
    case "SwitchStatement":
    case "SwitchCase":
    case "CatchClause":
      manager.pop();
      return;
    case "BlockStatement":
      if (parent && key === "body" && skipBlockScope(parent.type)) {
        return;
      }
      manager.pop();
      return;
    default:
      return;
  }
}
