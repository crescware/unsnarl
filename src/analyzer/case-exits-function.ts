import { isAbruptCompletionStatement } from "./is-abrupt-completion-statement.js";
import { isAstNode } from "./is-ast-node.js";

export function caseExitsFunction(consequent: readonly unknown[]): boolean {
  if (consequent.length === 0) {
    return false;
  }
  const last = consequent[consequent.length - 1];
  if (!isAstNode(last)) {
    return false;
  }
  return isAbruptCompletionStatement(last);
}
