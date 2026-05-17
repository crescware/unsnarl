import { describe, expect, test } from "vitest";

import type { Span } from "../../ir/primitive/span.js";
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
import { asFilledString } from "../../util/filled-string.js";
import { renderHeadExpression } from "./render-head-expression.js";

function spanAt(offset: number): Span {
  return { offset, line: 1, column: 0 };
}

describe("renderHeadExpression", () => {
  test("renders an identifier head as the bare name", () => {
    expect(
      renderHeadExpression(
        { kind: identifier$.literal, name: asFilledString("x") },
        "",
      ),
    ).toEqual("x");
  });

  test("renders a member access head as `<object>.<property>`", () => {
    const head: SerializedHeadExpression = {
      kind: member$.literal,
      object: {
        kind: identifier$.literal,
        name: asFilledString("fns"),
      },
      property: asFilledString("push"),
    };
    expect(renderHeadExpression(head, "")).toEqual("fns.push");
  });

  test("renders a call head as `<callee>()`, dropping the call's arguments", () => {
    const head: SerializedHeadExpression = {
      kind: call$.literal,
      callee: {
        kind: member$.literal,
        object: {
          kind: identifier$.literal,
          name: asFilledString("console"),
        },
        property: asFilledString("log"),
      },
    };
    expect(renderHeadExpression(head, "")).toEqual("console.log()");
  });

  test("renders a new head as `new <callee>()`", () => {
    const head: SerializedHeadExpression = {
      kind: new$.literal,
      callee: {
        kind: identifier$.literal,
        name: asFilledString("C"),
      },
    };
    expect(renderHeadExpression(head, "")).toEqual("new C()");
  });

  test("renders an awaited chain by prepending `await ` to the inner head", () => {
    const head: SerializedHeadExpression = {
      kind: await$.literal,
      argument: {
        kind: call$.literal,
        callee: {
          kind: member$.literal,
          object: {
            kind: call$.literal,
            callee: {
              kind: member$.literal,
              object: {
                kind: call$.literal,
                callee: {
                  kind: member$.literal,
                  object: {
                    kind: identifier$.literal,
                    name: asFilledString("Promise"),
                  },
                  property: asFilledString("resolve"),
                },
              },
              property: asFilledString("then"),
            },
          },
          property: asFilledString("catch"),
        },
      },
    };
    expect(renderHeadExpression(head, "")).toEqual(
      "await Promise.resolve().then().catch()",
    );
  });

  test("renders an assign head as `<left> <op> <right>`", () => {
    const head: SerializedHeadExpression = {
      kind: assign$.literal,
      operator: "=",
      left: {
        head: {
          kind: member$.literal,
          object: {
            kind: identifier$.literal,
            name: asFilledString("C"),
          },
          property: asFilledString("z"),
        },
        startSpan: spanAt(0),
        endSpan: spanAt(3),
      },
      right: {
        head: { kind: identifier$.literal, name: asFilledString("v") },
        startSpan: spanAt(6),
        endSpan: spanAt(7),
      },
    };
    expect(renderHeadExpression(head, "")).toEqual("C.z = v");
  });

  // The whole point of the elided kind: a literal RHS like `1`
  // collapses to "..." so the label reads "C.z = ..." instead of
  // dragging the literal into a diagram that only exists to show
  // how `C` is used.
  test("renders an assign head with an elided right-hand side", () => {
    const head: SerializedHeadExpression = {
      kind: assign$.literal,
      operator: "=",
      left: {
        head: {
          kind: member$.literal,
          object: {
            kind: identifier$.literal,
            name: asFilledString("C"),
          },
          property: asFilledString("z"),
        },
        startSpan: spanAt(0),
        endSpan: spanAt(3),
      },
      right: {
        head: { kind: elided$.literal },
        startSpan: spanAt(6),
        endSpan: spanAt(7),
      },
    };
    expect(renderHeadExpression(head, "")).toEqual("C.z = ...");
  });

  test("renders a compound assignment operator verbatim between the operands", () => {
    const head: SerializedHeadExpression = {
      kind: assign$.literal,
      operator: "+=",
      left: {
        head: { kind: identifier$.literal, name: asFilledString("count") },
        startSpan: spanAt(0),
        endSpan: spanAt(5),
      },
      right: {
        head: { kind: elided$.literal },
        startSpan: spanAt(9),
        endSpan: spanAt(10),
      },
    };
    expect(renderHeadExpression(head, "")).toEqual("count += ...");
  });

  test("renders a prefix update as `<op><argument>`", () => {
    const head: SerializedHeadExpression = {
      kind: update$.literal,
      operator: "++",
      prefix: true,
      argument: {
        head: {
          kind: member$.literal,
          object: { kind: identifier$.literal, name: asFilledString("C") },
          property: asFilledString("z"),
        },
        startSpan: spanAt(2),
        endSpan: spanAt(5),
      },
    };
    expect(renderHeadExpression(head, "")).toEqual("++C.z");
  });

  test("renders a postfix update as `<argument><op>`", () => {
    const head: SerializedHeadExpression = {
      kind: update$.literal,
      operator: "--",
      prefix: false,
      argument: {
        head: { kind: identifier$.literal, name: asFilledString("n") },
        startSpan: spanAt(0),
        endSpan: spanAt(1),
      },
    };
    expect(renderHeadExpression(head, "")).toEqual("n--");
  });

  test("renders a standalone elided head as the literal `...`", () => {
    expect(renderHeadExpression({ kind: elided$.literal }, "")).toEqual("...");
  });

  test("slices the original source for a raw head", () => {
    const head: SerializedHeadExpression = {
      kind: raw$.literal,
      startSpan: spanAt(0),
      endSpan: spanAt(7),
    };
    expect(renderHeadExpression(head, "x += 1; rest")).toEqual("x += 1;");
  });
});
