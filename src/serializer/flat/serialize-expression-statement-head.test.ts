import { describe, expect, test } from "vitest";

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
import type { HeadExpression } from "../../ir/reference/expression-statement-head.js";
import { serializeHeadExpression } from "./serialize-expression-statement-head.js";

describe("serializeHeadExpression", () => {
  test("passes an identifier head through unchanged", () => {
    expect(
      serializeHeadExpression({ kind: identifier$.literal, name: "x" }, ""),
    ).toEqual({ kind: identifier$.literal, name: "x" });
  });

  test("recurses through a member head and keeps the property name", () => {
    const head: HeadExpression = {
      kind: member$.literal,
      object: { kind: identifier$.literal, name: "fns" },
      property: "push",
    };
    expect(serializeHeadExpression(head, "")).toEqual({
      kind: member$.literal,
      object: { kind: identifier$.literal, name: "fns" },
      property: "push",
    });
  });

  test("recurses through a call head", () => {
    const head: HeadExpression = {
      kind: call$.literal,
      callee: { kind: identifier$.literal, name: "foo" },
    };
    expect(serializeHeadExpression(head, "")).toEqual({
      kind: call$.literal,
      callee: { kind: identifier$.literal, name: "foo" },
    });
  });

  test("recurses through a new head", () => {
    const head: HeadExpression = {
      kind: new$.literal,
      callee: { kind: identifier$.literal, name: "C" },
    };
    expect(serializeHeadExpression(head, "")).toEqual({
      kind: new$.literal,
      callee: { kind: identifier$.literal, name: "C" },
    });
  });

  test("recurses through an await head", () => {
    const head: HeadExpression = {
      kind: await$.literal,
      argument: {
        kind: call$.literal,
        callee: { kind: identifier$.literal, name: "go" },
      },
    };
    expect(serializeHeadExpression(head, "")).toEqual({
      kind: await$.literal,
      argument: {
        kind: call$.literal,
        callee: { kind: identifier$.literal, name: "go" },
      },
    });
  });

  test("recurses through an assign head and converts each operand's offsets to spans", () => {
    const raw = "C.z = v";
    const head: HeadExpression = {
      kind: assign$.literal,
      operator: "=",
      left: {
        head: {
          kind: member$.literal,
          object: { kind: identifier$.literal, name: "C" },
          property: "z",
        },
        startOffset: 0,
        endOffset: 3,
      },
      right: {
        head: { kind: identifier$.literal, name: "v" },
        startOffset: 6,
        endOffset: 7,
      },
    };
    expect(serializeHeadExpression(head, raw)).toEqual({
      kind: assign$.literal,
      operator: "=",
      left: {
        head: {
          kind: member$.literal,
          object: { kind: identifier$.literal, name: "C" },
          property: "z",
        },
        startSpan: { offset: 0, line: 1, column: 0 },
        endSpan: { offset: 3, line: 1, column: 3 },
      },
      right: {
        head: { kind: identifier$.literal, name: "v" },
        startSpan: { offset: 6, line: 1, column: 6 },
        endSpan: { offset: 7, line: 1, column: 7 },
      },
    });
  });

  // The elided side has no structural position of its own, so the
  // operand's span IS the only locator for that side. Verify the
  // serializer keeps it intact and well-formed.
  test("preserves the span on an elided assign operand through serialization", () => {
    const raw = "C.z = 1";
    const head: HeadExpression = {
      kind: assign$.literal,
      operator: "=",
      left: {
        head: {
          kind: member$.literal,
          object: { kind: identifier$.literal, name: "C" },
          property: "z",
        },
        startOffset: 0,
        endOffset: 3,
      },
      right: {
        head: { kind: elided$.literal },
        startOffset: 6,
        endOffset: 7,
      },
    };
    expect(serializeHeadExpression(head, raw)).toEqual({
      kind: assign$.literal,
      operator: "=",
      left: {
        head: {
          kind: member$.literal,
          object: { kind: identifier$.literal, name: "C" },
          property: "z",
        },
        startSpan: { offset: 0, line: 1, column: 0 },
        endSpan: { offset: 3, line: 1, column: 3 },
      },
      right: {
        head: { kind: elided$.literal },
        startSpan: { offset: 6, line: 1, column: 6 },
        endSpan: { offset: 7, line: 1, column: 7 },
      },
    });
  });

  test("recurses through an update head and keeps operator + prefix + argument span", () => {
    const raw = "++C.z;";
    const head: HeadExpression = {
      kind: update$.literal,
      operator: "++",
      prefix: true,
      argument: {
        head: {
          kind: member$.literal,
          object: { kind: identifier$.literal, name: "C" },
          property: "z",
        },
        startOffset: 2,
        endOffset: 5,
      },
    };
    expect(serializeHeadExpression(head, raw)).toEqual({
      kind: update$.literal,
      operator: "++",
      prefix: true,
      argument: {
        head: {
          kind: member$.literal,
          object: { kind: identifier$.literal, name: "C" },
          property: "z",
        },
        startSpan: { offset: 2, line: 1, column: 2 },
        endSpan: { offset: 5, line: 1, column: 5 },
      },
    });
  });

  test("converts a raw head's offsets to Spans against the original source", () => {
    const raw = "line1\nline2 += 1;";
    const head: HeadExpression = {
      kind: raw$.literal,
      startOffset: 6,
      endOffset: 16,
    };
    expect(serializeHeadExpression(head, raw)).toEqual({
      kind: raw$.literal,
      startSpan: { offset: 6, line: 2, column: 0 },
      endSpan: { offset: 16, line: 2, column: 10 },
    });
  });
});
