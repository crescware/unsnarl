import type { HeadExpression } from "../../ir/reference/expression-statement-head.js";
import type { SerializedHeadExpression } from "../../ir/serialized/serialized-expression-statement-head.js";
import { spanFromOffset } from "../../util/span.js";

export function serializeHeadExpression(
  head: HeadExpression,
  raw: string,
): SerializedHeadExpression {
  switch (head.kind) {
    case "identifier":
      return { kind: "identifier", name: head.name };
    case "member":
      return {
        kind: "member",
        object: serializeHeadExpression(head.object, raw),
        property: head.property,
      };
    case "call":
      return {
        kind: "call",
        callee: serializeHeadExpression(head.callee, raw),
      };
    case "new":
      return {
        kind: "new",
        callee: serializeHeadExpression(head.callee, raw),
      };
    case "await":
      return {
        kind: "await",
        argument: serializeHeadExpression(head.argument, raw),
      };
    case "raw":
      return {
        kind: "raw",
        startSpan: spanFromOffset(raw, head.startOffset),
        endSpan: spanFromOffset(raw, head.endOffset),
      };
  }
}
