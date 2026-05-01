import { describe, expect, test } from "vitest";

import { formatCaseTest } from "./format-case-test.js";
import type { NodeLike } from "./node-like.js";

describe("formatCaseTest", () => {
  test("uses raw slice when start/end are valid and within length budget", () => {
    const node: NodeLike = { type: "BinaryExpression", start: 5, end: 14 };
    const raw = "case x === 1: break;";
    expect(formatCaseTest(node, raw)).toBe(raw.slice(5, 14));
  });

  test.each([
    {
      name: "start missing falls back to type-specific format",
      node: { type: "Identifier", name: "foo" } as NodeLike,
      raw: "irrelevant",
      expected: "foo",
    },
    {
      name: "end <= start falls back to type-specific format",
      node: { type: "Identifier", name: "x", start: 10, end: 10 } as NodeLike,
      raw: "irrelevant",
      expected: "x",
    },
    {
      name: "end > raw.length falls back to type-specific format",
      node: { type: "Identifier", name: "x", start: 0, end: 1000 } as NodeLike,
      raw: "short",
      expected: "x",
    },
    {
      name: "length above 32 falls back to type-specific format",
      node: {
        type: "Identifier",
        name: "long",
        start: 0,
        end: 100,
      } as NodeLike,
      raw: "x".repeat(200),
      expected: "long",
    },
  ])("$name", ({ node, raw, expected }) => {
    expect(formatCaseTest(node, raw)).toBe(expected);
  });

  test.each([
    { type: "NullLiteral", expected: "null" },
    { type: "BooleanLiteral", value: true, expected: "true" },
    { type: "NumericLiteral", value: 42, expected: "42" },
    { type: "StringLiteral", value: "hi", expected: '"hi"' },
    { type: "ArrayExpression", expected: "<expr>" },
    { type: "Identifier", expected: "<expr>" },
  ])("type-specific fallback for $type", ({ type, value, expected }) => {
    const node = { type, value } as NodeLike;
    expect(formatCaseTest(node, "")).toBe(expected);
  });

  test("Identifier without name returns <expr>", () => {
    const node: NodeLike = { type: "Identifier" };
    expect(formatCaseTest(node, "")).toBe("<expr>");
  });
});
