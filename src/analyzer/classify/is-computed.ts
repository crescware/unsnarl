import type { AstNode } from "../../ir/model.js";

export function isComputed(node: AstNode): boolean {
  return (node as { computed?: boolean }).computed === true;
}
