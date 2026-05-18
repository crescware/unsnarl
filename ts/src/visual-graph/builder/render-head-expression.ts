import {
  identifier$,
  member$,
  call$,
  new$,
  await$,
  assign$,
  update$,
  elided$,
  raw$,
} from "../../ir/reference/expression-statement-head-kind.js";
import type { SerializedHeadExpression } from "../../ir/serialized/serialized-expression-statement-head.js";

// Render the structural head mini-AST to a compact one-line display label.
// `raw` segments fall back to slicing the original source so non-recognised
// shapes appear verbatim. `elided` segments collapse the corresponding
// operand to "..." so a shape like `C.z = 1` shows up as `C.z = ...`
// without dragging the literal RHS into the diagram.
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
    case assign$.literal: {
      const left = renderHeadExpression(head.left.head, raw);
      const right = renderHeadExpression(head.right.head, raw);
      return `${left} ${head.operator} ${right}`;
    }
    case update$.literal: {
      const arg = renderHeadExpression(head.argument.head, raw);
      return head.prefix ? `${head.operator}${arg}` : `${arg}${head.operator}`;
    }
    case elided$.literal:
      return "...";
    case raw$.literal:
      return raw.slice(head.startSpan.offset, head.endSpan.offset);
  }
}
