import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { Span } from "../../ir/primitive/span.js";
import { spanFromOffset } from "../../util/span.js";

export function spanOf(node: AstNode, raw: string): Span {
  return spanFromOffset(raw, node.start ?? 0);
}
