import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { isAstNode } from "./is-ast-node.js";

/**
 * Statement が ECMA §6.2.4 の abrupt completion (return / throw) を産出
 * するか、あるいはネスト構造の終端で必ず産出するかを判定する。
 *
 * 旧名は `isFunctionExit`。Completion 抽象の導入に合わせて、ECMAScript
 * 仕様の語彙（abrupt completion）に揃えてリネームした。break / continue
 * は本判定の対象に含まれない（Reference annotation のスコープ上、値を
 * 運ばない completion はモデル化していない。詳細は
 * `src/ir/reference/completion.ts` を参照）。
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
