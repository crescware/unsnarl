import type { SerializedHeadExpression } from "../../ir/serialized/serialized-expression-statement-head.js";

// Render the structural head mini-AST to a compact one-line display label.
// `raw` segments fall back to slicing the original source so non-recognised
// shapes (assignments, updates, etc.) appear verbatim.
export function renderHeadExpression(
  head: SerializedHeadExpression,
  raw: string,
): string {
  switch (head.kind) {
    case "identifier":
      return head.name;
    case "member":
      return `${renderHeadExpression(head.object, raw)}.${head.property}`;
    case "call":
      return `${renderHeadExpression(head.callee, raw)}()`;
    case "new":
      return `new ${renderHeadExpression(head.callee, raw)}()`;
    case "await":
      return `await ${renderHeadExpression(head.argument, raw)}`;
    case "raw":
      return raw.slice(head.startSpan.offset, head.endSpan.offset);
  }
}
