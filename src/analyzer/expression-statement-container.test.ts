import { describe, expect, test } from "vitest";

import type { AstNode } from "../ir/model.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { findExpressionStatementContainer } from "./expression-statement-container.js";
import type { PathEntry } from "./walk/walk.js";

function entry(node: AstNode, key: string | null = null): PathEntry {
  return { node, key };
}

describe("findExpressionStatementContainer", () => {
  test("returns the statement span and callee head when the expression is a CallExpression", () => {
    const callee = {
      type: AST_TYPE.MemberExpression,
      start: 0,
      end: 11,
    } as const satisfies AstNode;
    const callExpr = {
      type: AST_TYPE.CallExpression,
      start: 0,
      end: 14,
      callee,
    } as const satisfies AstNode;
    const stmt = {
      type: AST_TYPE.ExpressionStatement,
      start: 0,
      end: 15,
      expression: callExpr,
    } as const satisfies AstNode;
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 20 }),
      entry(stmt, "body"),
      entry(callExpr, "expression"),
      entry(callee, "callee"),
      entry({ type: AST_TYPE.Identifier, start: 0, end: 7 }, "object"),
    ] satisfies PathEntry[];
    expect(findExpressionStatementContainer(path)).toEqual({
      startOffset: 0,
      endOffset: 15,
      headStartOffset: 0,
      headEndOffset: 11,
      isCall: true,
    });
  });

  test("returns the whole expression as the head when the expression is not a call", () => {
    const expr = {
      type: AST_TYPE.Identifier,
      start: 5,
      end: 6,
    } as const satisfies AstNode;
    const stmt = {
      type: AST_TYPE.ExpressionStatement,
      start: 5,
      end: 7,
      expression: expr,
    } as const satisfies AstNode;
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 20 }),
      entry(stmt, "body"),
      entry(expr, "expression"),
    ] satisfies PathEntry[];
    expect(findExpressionStatementContainer(path)).toEqual({
      startOffset: 5,
      endOffset: 7,
      headStartOffset: 5,
      headEndOffset: 6,
      isCall: false,
    });
  });

  test("stops at function boundaries — a ref inside a function never gets an ExpressionStatement container", () => {
    const stmt = {
      type: AST_TYPE.ExpressionStatement,
      start: 30,
      end: 50,
    } as const satisfies AstNode;
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 100 }),
      entry({ type: AST_TYPE.FunctionDeclaration, start: 0, end: 100 }),
      entry({ type: AST_TYPE.BlockStatement, start: 15, end: 100 }, "body"),
      entry(stmt, "body"),
      entry({ type: AST_TYPE.Identifier, start: 30, end: 31 }, "expression"),
    ] satisfies PathEntry[];
    expect(findExpressionStatementContainer(path)).toBeNull();
  });

  test("returns null when there is no ExpressionStatement on the path", () => {
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 100 }),
      entry({ type: AST_TYPE.VariableDeclaration, start: 0, end: 10 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 6, end: 7 }, "id"),
    ] satisfies PathEntry[];
    expect(findExpressionStatementContainer(path)).toBeNull();
  });

  test("returns null when the ExpressionStatement is missing offsets", () => {
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 100 }),
      entry({ type: AST_TYPE.ExpressionStatement }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 0, end: 1 }, "expression"),
    ] satisfies PathEntry[];
    expect(findExpressionStatementContainer(path)).toBeNull();
  });
});
