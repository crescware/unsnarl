import { isFunctionExit } from "./is-function-exit.js";
import { isNodeLike } from "./is-node-like.js";

export function caseExitsFunction(consequent: readonly unknown[]): boolean {
  if (consequent.length === 0) {
    return false;
  }
  const last = consequent[consequent.length - 1];
  if (!isNodeLike(last)) {
    return false;
  }
  return isFunctionExit(last);
}
