import type { AstNode } from "../../ir/model.js";

export function offsetOf(node: AstNode): number {
  return node.start ?? 0;
}
