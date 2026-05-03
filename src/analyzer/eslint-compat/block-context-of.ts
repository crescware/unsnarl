import type { BlockContext } from "../../ir/model.js";
import type { PathEntry } from "../walk/walk.js";
import { ifChainRootOffset } from "./if-chain-root-offset.js";
import type { NodeLike } from "./node-like.js";

export function blockContextOf(
  parent: NodeLike | null,
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
    ...(chainRoot !== undefined ? { ifChainRootOffset: chainRoot } : {}),
  };
}
