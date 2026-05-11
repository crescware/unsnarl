import { describe, expect, test } from "vitest";

import type { Span } from "../../ir/primitive/span.js";
import type { SerializedHeadExpression } from "../../ir/serialized/serialized-expression-statement-head.js";
import { renderHeadExpression } from "./render-head-expression.js";

function spanAt(offset: number): Span {
  return { offset, line: 1, column: 0 };
}

describe("renderHeadExpression", () => {
  test("renders an identifier head as the bare name", () => {
    expect(renderHeadExpression({ kind: "identifier", name: "x" }, "")).toEqual(
      "x",
    );
  });

  test("renders a member access head as `<object>.<property>`", () => {
    const head: SerializedHeadExpression = {
      kind: "member",
      object: { kind: "identifier", name: "fns" },
      property: "push",
    };
    expect(renderHeadExpression(head, "")).toEqual("fns.push");
  });

  test("renders a call head as `<callee>()`, dropping the call's arguments", () => {
    const head: SerializedHeadExpression = {
      kind: "call",
      callee: {
        kind: "member",
        object: { kind: "identifier", name: "console" },
        property: "log",
      },
    };
    expect(renderHeadExpression(head, "")).toEqual("console.log()");
  });

  test("renders a new head as `new <callee>()`", () => {
    const head: SerializedHeadExpression = {
      kind: "new",
      callee: { kind: "identifier", name: "C" },
    };
    expect(renderHeadExpression(head, "")).toEqual("new C()");
  });

  test("renders an awaited chain by prepending `await ` to the inner head", () => {
    const head: SerializedHeadExpression = {
      kind: "await",
      argument: {
        kind: "call",
        callee: {
          kind: "member",
          object: {
            kind: "call",
            callee: {
              kind: "member",
              object: {
                kind: "call",
                callee: {
                  kind: "member",
                  object: { kind: "identifier", name: "Promise" },
                  property: "resolve",
                },
              },
              property: "then",
            },
          },
          property: "catch",
        },
      },
    };
    expect(renderHeadExpression(head, "")).toEqual(
      "await Promise.resolve().then().catch()",
    );
  });

  test("slices the original source for a raw head", () => {
    const head: SerializedHeadExpression = {
      kind: "raw",
      startSpan: spanAt(0),
      endSpan: spanAt(7),
    };
    expect(renderHeadExpression(head, "x += 1; rest")).toEqual("x += 1;");
  });
});
