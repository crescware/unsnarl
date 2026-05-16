import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { isAstNode } from "./is-ast-node.js";

/**
 * True if the Statement produces an ECMA §6.2.4 abrupt completion
 * (return / throw), either directly or at every reachable termination
 * of a nested structure.
 *
 * break / continue are out of scope: they carry no value, so they
 * never appear as a Reference's completion category.
 *
 * @see https://tc39.es/ecma262/#sec-completion-record-specification-type ECMA §6.2.4 Completion Record
 * @see https://github.com/crescware/unsnarl/issues/94 Issue #94
 */
export function isAbruptCompletionStatement(node: AstNode): boolean {
  switch (node.type) {
    case AST_TYPE.ReturnStatement:
    case AST_TYPE.ThrowStatement:
      return true;
    case AST_TYPE.BlockStatement: {
      const body = node["body"];
      if (Array.isArray(body) && body.length > 0) {
        const last = body[body.length - 1];
        if (isAstNode(last)) {
          return isAbruptCompletionStatement(last);
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
        isAbruptCompletionStatement(consequent) &&
        isAbruptCompletionStatement(alternate)
      ) {
        return true;
      }
      return false;
    }
    default:
      return false;
  }
}
