import { AST_TYPE } from "../../parser/ast-type.js";
import type { PathEntry } from "../walk/path-entry.js";
import type { NodeLike } from "./node-like.js";

// `else if` is parsed as a bare IfStatement placed directly in the outer
// IfStatement's `alternate` slot (no enclosing BlockStatement). When the
// current consequent / alternate belongs to such a chained IfStatement, walk
// the AST ancestor path back through every alternate-of-IfStatement link
// and return the start offset of the outermost IfStatement so all branches
// in the chain can share a single merge container key.
export function ifChainRootOffset(
  parent: NodeLike | null,
  key: string | null,
  path: readonly PathEntry[],
): number | undefined {
  if (!parent || parent.type !== AST_TYPE.IfStatement) {
    return undefined;
  }
  if (key !== "consequent" && key !== "alternate") {
    return undefined;
  }
  let chainTop: NodeLike = parent;
  for (let i = path.length - 1; i >= 1; i--) {
    const entry = path[i];
    if (!entry) {
      break;
    }
    const entryNode = entry.node as unknown as NodeLike;
    if (entryNode !== chainTop) {
      break;
    }
    if (entry.key !== "alternate") {
      break;
    }
    const ancestorEntry = path[i - 1];
    if (!ancestorEntry) {
      break;
    }
    const ancestor = ancestorEntry.node as unknown as NodeLike;
    if (ancestor.type !== AST_TYPE.IfStatement) {
      break;
    }
    chainTop = ancestor;
  }
  if (chainTop === parent) {
    return undefined;
  }
  return chainTop.start ?? 0;
}
