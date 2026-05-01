import { describe, expect, test } from "vitest";

import { isFunctionExit } from "./is-function-exit.js";
import type { NodeLike } from "./node-like.js";

describe("isFunctionExit", () => {
  test.each([
    {
      name: "ReturnStatement -> true",
      node: { type: "ReturnStatement" },
      expected: true,
    },
    {
      name: "ThrowStatement -> true",
      node: { type: "ThrowStatement" },
      expected: true,
    },
    {
      name: "BreakStatement -> false (does not exit fn)",
      node: { type: "BreakStatement" },
      expected: false,
    },
    {
      name: "ContinueStatement -> false",
      node: { type: "ContinueStatement" },
      expected: false,
    },
    {
      name: "ExpressionStatement -> false",
      node: { type: "ExpressionStatement" },
      expected: false,
    },
  ])("$name", ({ node, expected }) => {
    expect(isFunctionExit(node as NodeLike)).toBe(expected);
  });

  test("BlockStatement: ends in ReturnStatement -> true", () => {
    const node = {
      type: "BlockStatement",
      body: [{ type: "ExpressionStatement" }, { type: "ReturnStatement" }],
    } as const satisfies NodeLike;
    expect(isFunctionExit(node)).toBe(true);
  });

  test("BlockStatement: ends in non-exit -> false", () => {
    const node = {
      type: "BlockStatement",
      body: [{ type: "ReturnStatement" }, { type: "ExpressionStatement" }],
    } as const satisfies NodeLike;
    expect(isFunctionExit(node)).toBe(false);
  });

  test("BlockStatement: empty body -> false", () => {
    expect(isFunctionExit({ type: "BlockStatement", body: [] })).toBe(false);
  });

  test("IfStatement: both branches exit -> true", () => {
    const node = {
      type: "IfStatement",
      consequent: { type: "ReturnStatement" },
      alternate: { type: "ThrowStatement" },
    } as const satisfies NodeLike;
    expect(isFunctionExit(node)).toBe(true);
  });

  test("IfStatement: only consequent exits -> false", () => {
    const node = {
      type: "IfStatement",
      consequent: { type: "ReturnStatement" },
      alternate: { type: "ExpressionStatement" },
    } as const satisfies NodeLike;
    expect(isFunctionExit(node)).toBe(false);
  });

  test("IfStatement: missing alternate -> false", () => {
    const node = {
      type: "IfStatement",
      consequent: { type: "ReturnStatement" },
    } as const satisfies NodeLike;
    expect(isFunctionExit(node)).toBe(false);
  });
});
