import { parse } from "valibot";

import {
  identifier$,
  member$,
  call$,
  new$,
  await$,
  raw$,
} from "../../ir/reference/expression-statement-head-kind.js";
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
    case identifier$.literal:
      return parse(serializedHeadExpression$, {
        kind: identifier$.literal,
        name: head.name,
      });

    case member$.literal:
      return parse(serializedHeadExpression$, {
        kind: member$.literal,
        object: serializeHeadExpression(head.object, raw),
        property: head.property,
      });

    case call$.literal:
      return parse(serializedHeadExpression$, {
        kind: call$.literal,
        callee: serializeHeadExpression(head.callee, raw),
      });

    case new$.literal:
      return parse(serializedHeadExpression$, {
        kind: new$.literal,
        callee: serializeHeadExpression(head.callee, raw),
      });

    case await$.literal:
      return parse(serializedHeadExpression$, {
        kind: await$.literal,
        argument: serializeHeadExpression(head.argument, raw),
      });

    case raw$.literal:
      return parse(serializedHeadExpression$, {
        kind: raw$.literal,
        startSpan: spanFromOffset(raw, head.startOffset),
        endSpan: spanFromOffset(raw, head.endOffset),
      });
  }
}
