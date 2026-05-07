import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { isAstNode } from "./is-ast-node.js";

export function isFunctionExit(node: AstNode): boolean {
  switch (node.type) {
    case AST_TYPE.ReturnStatement:
    case AST_TYPE.ThrowStatement:
      return true;
    case AST_TYPE.BlockStatement: {
      const body = node["body"];
      if (Array.isArray(body) && body.length > 0) {
        const last = body[body.length - 1];
        if (isAstNode(last)) {
          return isFunctionExit(last);
        }
      }
      return false;
    }
    case AST_TYPE.IfStatement: {
      const consequent = node["consequent"];
      const alternate = node["alternate"];
      if (
        isAstNode(consequent) &&
        isAstNode(alternate) &&
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
