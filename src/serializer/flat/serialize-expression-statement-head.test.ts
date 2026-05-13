import { describe, expect, test } from "vitest";

import {
  identifier$,
  member$,
  call$,
  new$,
  await$,
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
