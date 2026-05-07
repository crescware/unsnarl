import { isAstNode } from "./is-ast-node.js";
import { isControlExit } from "./is-control-exit.js";

export function caseFallsThrough(consequent: readonly unknown[]): boolean {
  if (consequent.length === 0) {
    return true;
  }
  const last = consequent[consequent.length - 1];
  if (!isAstNode(last)) {
    return true;
  }
  return !isControlExit(last);
}
