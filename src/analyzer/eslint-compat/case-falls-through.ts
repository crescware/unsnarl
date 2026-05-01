import { isControlExit } from "./is-control-exit.js";
import { isNodeLike } from "./is-node-like.js";

export function caseFallsThrough(consequent: readonly unknown[]): boolean {
  if (consequent.length === 0) {
    return true;
  }
  const last = consequent[consequent.length - 1];
  if (!isNodeLike(last)) {
    return true;
  }
  return !isControlExit(last);
}
