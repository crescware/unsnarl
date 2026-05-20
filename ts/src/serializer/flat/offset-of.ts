import type { AstNode } from "../../ir/primitive/ast-node.js";

export function offsetOf(node: AstNode): number {
  return node.start ?? 0;
}
