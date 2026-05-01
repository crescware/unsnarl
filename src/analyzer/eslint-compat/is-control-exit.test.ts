import { describe, expect, test } from "vitest";

import { isControlExit } from "./is-control-exit.js";
import type { NodeLike } from "./node-like.js";

describe("isControlExit", () => {
  test.each([
    { type: "BreakStatement", expected: true },
    { type: "ContinueStatement", expected: true },
    { type: "ReturnStatement", expected: true },
    { type: "ThrowStatement", expected: true },
    { type: "ExpressionStatement", expected: false },
    { type: "VariableDeclaration", expected: false },
  ])("type=$type -> $expected", ({ type, expected }) => {
    expect(isControlExit({ type } as NodeLike)).toBe(expected);
  });

  test("BlockStatement: ends in BreakStatement -> true", () => {
    const node = {
      type: "BlockStatement",
      body: [{ type: "ExpressionStatement" }, { type: "BreakStatement" }],
    } as const satisfies NodeLike;
    expect(isControlExit(node)).toBe(true);
  });

  test("BlockStatement: empty body -> false", () => {
    expect(isControlExit({ type: "BlockStatement", body: [] })).toBe(false);
  });

  test("IfStatement: both branches exit (mixed kinds) -> true", () => {
    const node = {
      type: "IfStatement",
      consequent: { type: "BreakStatement" },
      alternate: { type: "ReturnStatement" },
    } as const satisfies NodeLike;
    expect(isControlExit(node)).toBe(true);
  });

  test("IfStatement: only one branch exits -> false", () => {
    const node = {
      type: "IfStatement",
      consequent: { type: "BreakStatement" },
      alternate: { type: "ExpressionStatement" },
    } as const satisfies NodeLike;
    expect(isControlExit(node)).toBe(false);
  });
});
