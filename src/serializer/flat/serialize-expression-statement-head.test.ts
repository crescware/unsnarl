import { describe, expect, test } from "vitest";

import type { HeadExpression } from "../../ir/reference/expression-statement-head.js";
import { serializeHeadExpression } from "./serialize-expression-statement-head.js";

describe("serializeHeadExpression", () => {
  test("passes an identifier head through unchanged", () => {
    expect(
      serializeHeadExpression({ kind: "identifier", name: "x" }, ""),
    ).toEqual({ kind: "identifier", name: "x" });
  });

  test("recurses through a member head and keeps the property name", () => {
    const head: HeadExpression = {
      kind: "member",
      object: { kind: "identifier", name: "fns" },
      property: "push",
    };
    expect(serializeHeadExpression(head, "")).toEqual({
      kind: "member",
      object: { kind: "identifier", name: "fns" },
      property: "push",
    });
  });

  test("recurses through a call head", () => {
    const head: HeadExpression = {
      kind: "call",
      callee: { kind: "identifier", name: "foo" },
    };
    expect(serializeHeadExpression(head, "")).toEqual({
      kind: "call",
      callee: { kind: "identifier", name: "foo" },
    });
  });

  test("recurses through a new head", () => {
    const head: HeadExpression = {
      kind: "new",
      callee: { kind: "identifier", name: "C" },
    };
    expect(serializeHeadExpression(head, "")).toEqual({
      kind: "new",
      callee: { kind: "identifier", name: "C" },
    });
  });

  test("recurses through an await head", () => {
    const head: HeadExpression = {
      kind: "await",
      argument: {
        kind: "call",
        callee: { kind: "identifier", name: "go" },
      },
    };
    expect(serializeHeadExpression(head, "")).toEqual({
      kind: "await",
      argument: {
        kind: "call",
        callee: { kind: "identifier", name: "go" },
      },
    });
  });

  test("converts a raw head's offsets to Spans against the original source", () => {
    const raw = "line1\nline2 += 1;";
    const head: HeadExpression = {
      kind: "raw",
      startOffset: 6,
      endOffset: 16,
    };
    expect(serializeHeadExpression(head, raw)).toEqual({
      kind: "raw",
      startSpan: { offset: 6, line: 2, column: 0 },
      endSpan: { offset: 16, line: 2, column: 10 },
    });
  });
});
