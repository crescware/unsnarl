import type { AstNode, Span } from "../../ir/model.js";
import { spanFromOffset } from "../../util/span.js";

export function spanOf(node: AstNode, raw: string): Span {
  return spanFromOffset(raw, node.start ?? 0);
}
