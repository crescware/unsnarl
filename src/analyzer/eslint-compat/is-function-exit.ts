import { isNodeLike } from "./is-node-like.js";
import type { NodeLike } from "./node-like.js";

export function isFunctionExit(node: NodeLike): boolean {
  switch (node.type) {
    case "ReturnStatement":
    case "ThrowStatement":
      return true;
    case "BlockStatement": {
      const body = node["body"];
      if (Array.isArray(body) && body.length > 0) {
        const last = body[body.length - 1];
        if (isNodeLike(last)) {
          return isFunctionExit(last);
        }
      }
      return false;
    }
    case "IfStatement": {
      const consequent = node["consequent"];
      const alternate = node["alternate"];
      if (
        isNodeLike(consequent) &&
        isNodeLike(alternate) &&
        isFunctionExit(consequent) &&
        isFunctionExit(alternate)
      ) {
        return true;
      }
      return false;
    }
    default:
      return false;
  }
}
