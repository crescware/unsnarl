import { describe, expect, test } from "vitest";

import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { isAbruptCompletionStatement } from "./is-abrupt-completion-statement.js";

describe("isAbruptCompletionStatement", () => {
  test.each([
    {
      name: "ReturnStatement -> true",
      node: { type: AST_TYPE.ReturnStatement },
      expected: true,
    },
    {
      name: "ThrowStatement -> true",
      node: { type: AST_TYPE.ThrowStatement },
      expected: true,
    },
    {
      name: "BreakStatement -> false (does not exit fn)",
      node: { type: AST_TYPE.BreakStatement },
      expected: false,
    },
    {
      name: "ContinueStatement -> false",
      node: { type: AST_TYPE.ContinueStatement },
      expected: false,
    },
    {
      name: "ExpressionStatement -> false",
      node: { type: AST_TYPE.ExpressionStatement },
      expected: false,
    },
  ])("$name", ({ node, expected }) => {
    expect(isAbruptCompletionStatement(node as AstNode)).toEqual(expected);
  });

  test("BlockStatement: ends in ReturnStatement -> true", () => {
    const node = {
      type: AST_TYPE.BlockStatement,
      body: [
        { type: AST_TYPE.ExpressionStatement },
        { type: AST_TYPE.ReturnStatement },
      ],
    } as const satisfies AstNode;
    expect(isAbruptCompletionStatement(node)).toEqual(true);
  });

  test("BlockStatement: ends in non-exit -> false", () => {
    const node = {
      type: AST_TYPE.BlockStatement,
      body: [
        { type: AST_TYPE.ReturnStatement },
        { type: AST_TYPE.ExpressionStatement },
      ],
    } as const satisfies AstNode;
    expect(isAbruptCompletionStatement(node)).toEqual(false);
  });

  test("BlockStatement: empty body -> false", () => {
    expect(
      isAbruptCompletionStatement({ type: AST_TYPE.BlockStatement, body: [] }),
    ).toEqual(false);
  });

  test("IfStatement: both branches exit -> true", () => {
    const node = {
      type: AST_TYPE.IfStatement,
      consequent: { type: AST_TYPE.ReturnStatement },
      alternate: { type: AST_TYPE.ThrowStatement },
    } as const satisfies AstNode;
    expect(isAbruptCompletionStatement(node)).toEqual(true);
  });

  test("IfStatement: only consequent exits -> false", () => {
    const node = {
      type: AST_TYPE.IfStatement,
      consequent: { type: AST_TYPE.ReturnStatement },
      alternate: { type: AST_TYPE.ExpressionStatement },
    } as const satisfies AstNode;
    expect(isAbruptCompletionStatement(node)).toEqual(false);
  });

  test("IfStatement: missing alternate -> false", () => {
    const node = {
      type: AST_TYPE.IfStatement,
      consequent: { type: AST_TYPE.ReturnStatement },
    } as const satisfies AstNode;
    expect(isAbruptCompletionStatement(node)).toEqual(false);
  });
});
