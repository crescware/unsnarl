import { describe, expect, test } from "vitest";

import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { isControlExit } from "./is-control-exit.js";

describe("isControlExit", () => {
  test.each([
    { type: AST_TYPE.BreakStatement, expected: true },
    { type: AST_TYPE.ContinueStatement, expected: true },
    { type: AST_TYPE.ReturnStatement, expected: true },
    { type: AST_TYPE.ThrowStatement, expected: true },
    { type: AST_TYPE.ExpressionStatement, expected: false },
    { type: AST_TYPE.VariableDeclaration, expected: false },
  ])("type=$type -> $expected", ({ type, expected }) => {
    expect(isControlExit({ type } as AstNode)).toEqual(expected);
  });

  test("BlockStatement: ends in BreakStatement -> true", () => {
    const node = {
      type: AST_TYPE.BlockStatement,
      body: [
        { type: AST_TYPE.ExpressionStatement },
        { type: AST_TYPE.BreakStatement },
      ],
    } as const satisfies AstNode;
    expect(isControlExit(node)).toEqual(true);
  });

  test("BlockStatement: empty body -> false", () => {
    expect(isControlExit({ type: AST_TYPE.BlockStatement, body: [] })).toEqual(
      false,
    );
  });

  test("IfStatement: both branches exit (mixed kinds) -> true", () => {
    const node = {
      type: AST_TYPE.IfStatement,
      consequent: { type: AST_TYPE.BreakStatement },
      alternate: { type: AST_TYPE.ReturnStatement },
    } as const satisfies AstNode;
    expect(isControlExit(node)).toEqual(true);
  });

  test("IfStatement: only one branch exits -> false", () => {
    const node = {
      type: AST_TYPE.IfStatement,
      consequent: { type: AST_TYPE.BreakStatement },
      alternate: { type: AST_TYPE.ExpressionStatement },
    } as const satisfies AstNode;
    expect(isControlExit(node)).toEqual(false);
  });
});
