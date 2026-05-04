import type { AstNode } from "../../ir/primitive/ast-node.js";

export function isComputed(node: AstNode): boolean {
  return (node as { computed?: boolean }).computed === true;
}
