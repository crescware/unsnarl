import { isAstNode } from "./is-ast-node.js";
import { isFunctionExit } from "./is-function-exit.js";

export function caseExitsFunction(consequent: readonly unknown[]): boolean {
  if (consequent.length === 0) {
    return false;
  }
  const last = consequent[consequent.length - 1];
  if (!isAstNode(last)) {
    return false;
  }
  return isFunctionExit(last);
}
