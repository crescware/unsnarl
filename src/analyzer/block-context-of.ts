import type { AstNode } from "../ir/primitive/ast-node.js";
import type { BlockContext } from "../ir/scope/block-context.js";
import { ifChainRootOffset } from "./if-chain-root-offset.js";
import type { PathEntry } from "./walk/path-entry.js";

export function blockContextOf(
  parent: AstNode | null,
  key: string | null,
  path: readonly PathEntry[],
): BlockContext | null {
  if (!parent || key === null) {
    return null;
  }
  const chainRoot = ifChainRootOffset(parent, key, path);
  return {
    kind: "other",
    parentType: parent.type,
    key,
    parentSpanOffset: parent.start ?? 0,
    ifChainRootOffset: chainRoot,
  };
}
