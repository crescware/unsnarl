import { parse } from "valibot";

import type { HeadExpression } from "../../ir/reference/expression-statement-head.js";
import {
  serializedHeadExpression$,
  type SerializedHeadExpression,
} from "../../ir/serialized/serialized-expression-statement-head.js";
import { spanFromOffset } from "../../util/span.js";

export function serializeHeadExpression(
  head: HeadExpression,
  raw: string,
): SerializedHeadExpression {
  switch (head.kind) {
    case "identifier":
      return parse(serializedHeadExpression$, {
        kind: "identifier",
        name: head.name,
      });
    case "member":
      return parse(serializedHeadExpression$, {
        kind: "member",
        object: serializeHeadExpression(head.object, raw),
        property: head.property,
      });
    case "call":
      return parse(serializedHeadExpression$, {
        kind: "call",
        callee: serializeHeadExpression(head.callee, raw),
      });
    case "new":
      return parse(serializedHeadExpression$, {
        kind: "new",
        callee: serializeHeadExpression(head.callee, raw),
      });
    case "await":
      return parse(serializedHeadExpression$, {
        kind: "await",
        argument: serializeHeadExpression(head.argument, raw),
      });
    case "raw":
      return parse(serializedHeadExpression$, {
        kind: "raw",
        startSpan: spanFromOffset(raw, head.startOffset),
        endSpan: spanFromOffset(raw, head.endOffset),
      });
  }
}
