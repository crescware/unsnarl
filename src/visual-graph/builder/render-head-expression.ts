import {
  identifier$,
  member$,
  call$,
  new$,
  await$,
  raw$,
} from "../../ir/reference/expression-statement-head-kind.js";
import type { SerializedHeadExpression } from "../../ir/serialized/serialized-expression-statement-head.js";

// Render the structural head mini-AST to a compact one-line display label.
// `raw` segments fall back to slicing the original source so non-recognised
// shapes (assignments, updates, etc.) appear verbatim.
export function renderHeadExpression(
  head: SerializedHeadExpression,
  raw: string,
): string {
  switch (head.kind) {
    case identifier$.literal:
      return head.name;
    case member$.literal:
      return `${renderHeadExpression(head.object, raw)}.${head.property}`;
    case call$.literal:
      return `${renderHeadExpression(head.callee, raw)}()`;
    case new$.literal:
      return `new ${renderHeadExpression(head.callee, raw)}()`;
    case await$.literal:
      return `await ${renderHeadExpression(head.argument, raw)}`;
    case raw$.literal:
      return raw.slice(head.startSpan.offset, head.endSpan.offset);
  }
}
