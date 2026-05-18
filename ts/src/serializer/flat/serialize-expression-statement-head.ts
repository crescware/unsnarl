import { parse } from "valibot";

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
import type {
  HeadExpression,
  HeadOperand,
} from "../../ir/reference/expression-statement-head.js";
import {
  serializedHeadExpression$,
  type SerializedHeadExpression,
  type SerializedHeadOperand,
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

    case assign$.literal:
      return parse(serializedHeadExpression$, {
        kind: assign$.literal,
        operator: head.operator,
        left: serializeHeadOperand(head.left, raw),
        right: serializeHeadOperand(head.right, raw),
      });

    case update$.literal:
      return parse(serializedHeadExpression$, {
        kind: update$.literal,
        operator: head.operator,
        prefix: head.prefix,
        argument: serializeHeadOperand(head.argument, raw),
      });

    case elided$.literal:
      return parse(serializedHeadExpression$, { kind: elided$.literal });

    case raw$.literal:
      return parse(serializedHeadExpression$, {
        kind: raw$.literal,
        startSpan: spanFromOffset(raw, head.startOffset),
        endSpan: spanFromOffset(raw, head.endOffset),
      });
  }
}

function serializeHeadOperand(
  operand: HeadOperand,
  raw: string,
): SerializedHeadOperand {
  return {
    head: serializeHeadExpression(operand.head, raw),
    startSpan: spanFromOffset(raw, operand.startOffset),
    endSpan: spanFromOffset(raw, operand.endOffset),
  };
}
