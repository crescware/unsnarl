import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../ast-type.js";
import { isControlExit } from "./is-control-exit.js";
import type { NodeLike } from "./node-like.js";

describe("isControlExit", () => {
  test.each([
    { type: AST_TYPE.BreakStatement, expected: true },
    { type: AST_TYPE.ContinueStatement, expected: true },
    { type: AST_TYPE.ReturnStatement, expected: true },
    { type: AST_TYPE.ThrowStatement, expected: true },
    { type: AST_TYPE.ExpressionStatement, expected: false },
    { type: AST_TYPE.VariableDeclaration, expected: false },
  ])("type=$type -> $expected", ({ type, expected }) => {
    expect(isControlExit({ type } as NodeLike)).toBe(expected);
  });

  test("BlockStatement: ends in BreakStatement -> true", () => {
    const node = {
      type: AST_TYPE.BlockStatement,
      body: [
        { type: AST_TYPE.ExpressionStatement },
        { type: AST_TYPE.BreakStatement },
      ],
    } as const satisfies NodeLike;
    expect(isControlExit(node)).toBe(true);
  });

  test("BlockStatement: empty body -> false", () => {
    expect(isControlExit({ type: AST_TYPE.BlockStatement, body: [] })).toBe(
      false,
    );
  });

  test("IfStatement: both branches exit (mixed kinds) -> true", () => {
    const node = {
      type: AST_TYPE.IfStatement,
      consequent: { type: AST_TYPE.BreakStatement },
      alternate: { type: AST_TYPE.ReturnStatement },
    } as const satisfies NodeLike;
    expect(isControlExit(node)).toBe(true);
  });

  test("IfStatement: only one branch exits -> false", () => {
    const node = {
      type: AST_TYPE.IfStatement,
      consequent: { type: AST_TYPE.BreakStatement },
      alternate: { type: AST_TYPE.ExpressionStatement },
    } as const satisfies NodeLike;
    expect(isControlExit(node)).toBe(false);
  });
});
