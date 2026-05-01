import type { BlockContext } from "../../ir/model.js";
import type { NodeLike } from "./node-like.js";

export function blockContextOf(
  parent: NodeLike | null,
  key: string | null,
): BlockContext | null {
  if (!parent || key === null) {
    return null;
  }
  return {
    parentType: parent.type,
    key,
    parentSpanOffset: parent.start ?? 0,
  };
}
