import { describe, expect, test } from "vitest";

import {
  break$,
  continue$,
  return$,
  throw$,
} from "../ir/completion/completion-type.js";
import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { abruptCompletionTypeOf } from "./abrupt-completion-type-of.js";

describe("abruptCompletionTypeOf", () => {
  test.each([
    {
      name: "ReturnStatement -> {return}",
      node: { type: AST_TYPE.ReturnStatement },
      expected: new Set([return$.literal]),
    },
    {
      name: "ThrowStatement -> {throw}",
      node: { type: AST_TYPE.ThrowStatement },
      expected: new Set([throw$.literal]),
    },
    {
      name: "BreakStatement -> {break}",
      node: { type: AST_TYPE.BreakStatement },
      expected: new Set([break$.literal]),
    },
    {
      name: "ContinueStatement -> {continue}",
      node: { type: AST_TYPE.ContinueStatement },
      expected: new Set([continue$.literal]),
    },
    {
      name: "ExpressionStatement -> null",
      node: { type: AST_TYPE.ExpressionStatement },
      expected: null,
    },
  ])("$name", ({ node, expected }) => {
    expect(abruptCompletionTypeOf(node as AstNode)).toEqual(expected);
  });

  test("BlockStatement: ends in ReturnStatement -> {return}", () => {
    const node = {
      type: AST_TYPE.BlockStatement,
      body: [
        { type: AST_TYPE.ExpressionStatement },
        { type: AST_TYPE.ReturnStatement },
      ],
    } as const satisfies AstNode;
    expect(abruptCompletionTypeOf(node)).toEqual(new Set([return$.literal]));
  });

  test("BlockStatement: ends in BreakStatement -> {break}", () => {
    const node = {
      type: AST_TYPE.BlockStatement,
      body: [{ type: AST_TYPE.BreakStatement }],
    } as const satisfies AstNode;
    expect(abruptCompletionTypeOf(node)).toEqual(new Set([break$.literal]));
  });

  test("BlockStatement: ends in non-abrupt -> null", () => {
    const node = {
      type: AST_TYPE.BlockStatement,
      body: [
        { type: AST_TYPE.ReturnStatement },
        { type: AST_TYPE.ExpressionStatement },
      ],
    } as const satisfies AstNode;
    expect(abruptCompletionTypeOf(node)).toEqual(null);
  });

  test("BlockStatement: empty body -> null", () => {
    expect(
      abruptCompletionTypeOf({ type: AST_TYPE.BlockStatement, body: [] }),
    ).toEqual(null);
  });

  test("IfStatement: return / throw -> {return, throw}", () => {
    const node = {
      type: AST_TYPE.IfStatement,
      consequent: { type: AST_TYPE.ReturnStatement },
      alternate: { type: AST_TYPE.ThrowStatement },
    } as const satisfies AstNode;
    expect(abruptCompletionTypeOf(node)).toEqual(
      new Set([return$.literal, throw$.literal]),
    );
  });

  test("IfStatement: break / throw -> {break, throw}", () => {
    const node = {
      type: AST_TYPE.IfStatement,
      consequent: { type: AST_TYPE.BreakStatement },
      alternate: { type: AST_TYPE.ThrowStatement },
    } as const satisfies AstNode;
    expect(abruptCompletionTypeOf(node)).toEqual(
      new Set([break$.literal, throw$.literal]),
    );
  });

  test("IfStatement: return / continue -> {return, continue}", () => {
    const node = {
      type: AST_TYPE.IfStatement,
      consequent: { type: AST_TYPE.ReturnStatement },
      alternate: { type: AST_TYPE.ContinueStatement },
    } as const satisfies AstNode;
    expect(abruptCompletionTypeOf(node)).toEqual(
      new Set([return$.literal, continue$.literal]),
    );
  });

  test("IfStatement: only consequent abrupt -> null", () => {
    const node = {
      type: AST_TYPE.IfStatement,
      consequent: { type: AST_TYPE.ReturnStatement },
      alternate: { type: AST_TYPE.ExpressionStatement },
    } as const satisfies AstNode;
    expect(abruptCompletionTypeOf(node)).toEqual(null);
  });

  test("IfStatement: missing alternate -> null", () => {
    const node = {
      type: AST_TYPE.IfStatement,
      consequent: { type: AST_TYPE.ReturnStatement },
    } as const satisfies AstNode;
    expect(abruptCompletionTypeOf(node)).toEqual(null);
  });

  test("LabeledStatement: pre-existing limitation -> null (see issue #97)", () => {
    // ECMA §14.13.4 says the labelled wrapper inherits the body's
    // completion. abruptCompletionTypeOf intentionally does not
    // implement that; routing through the switch default returns
    // null even when the body is a definite return.
    const node = {
      type: AST_TYPE.LabeledStatement,
      label: { type: AST_TYPE.Identifier, name: "outer" },
      body: { type: AST_TYPE.ReturnStatement },
    } as const satisfies AstNode;
    expect(abruptCompletionTypeOf(node)).toEqual(null);
  });
});
