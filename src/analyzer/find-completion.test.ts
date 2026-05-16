import { describe, expect, test } from "vitest";

import type { PathEntry } from "../boundary/eslint-scope/walk/path-entry.js";
import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { findCompletion } from "./find-completion.js";

function entry(node: AstNode, key: string | null = null): PathEntry {
  return { node, key };
}

describe("findCompletion", () => {
  test("returns return-completion when a ReturnStatement is on the path", () => {
    const path = [
      entry({ type: AST_TYPE.FunctionDeclaration, start: 0, end: 100 }),
      entry({ type: AST_TYPE.BlockStatement, start: 15, end: 100 }, "body"),
      entry({ type: AST_TYPE.ReturnStatement, start: 20, end: 50 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 27, end: 28 }, "argument"),
    ] satisfies PathEntry[];
    expect(findCompletion(path)).toEqual({
      kind: "return",
      startOffset: 20,
      endOffset: 50,
    });
  });

  test("returns return-completion with the body span when an arrow has an expression body", () => {
    const bodyNode = {
      type: AST_TYPE.BinaryExpression,
      start: 30,
      end: 50,
    } as const satisfies AstNode;
    const arrowNode = {
      type: AST_TYPE.ArrowFunctionExpression,
      start: 10,
      end: 60,
      body: bodyNode,
    } as const satisfies AstNode;
    const path = [
      entry(arrowNode),
      entry(bodyNode, "body"),
      entry({ type: AST_TYPE.Identifier, start: 30, end: 31 }, "left"),
    ] satisfies PathEntry[];
    expect(findCompletion(path)).toEqual({
      kind: "return",
      startOffset: 30,
      endOffset: 50,
    });
  });

  test("returns normal-completion for a block-body arrow with no inner ReturnStatement", () => {
    const bodyNode = {
      type: AST_TYPE.BlockStatement,
      start: 25,
      end: 60,
    } as const satisfies AstNode;
    const arrowNode = {
      type: AST_TYPE.ArrowFunctionExpression,
      start: 10,
      end: 60,
      body: bodyNode,
    } as const satisfies AstNode;
    const path = [
      entry(arrowNode),
      entry(bodyNode, "body"),
      entry({ type: AST_TYPE.ExpressionStatement, start: 30, end: 50 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 30, end: 31 }, "expression"),
    ] satisfies PathEntry[];
    expect(findCompletion(path)).toEqual({ kind: "normal" });
  });

  test("prefers an inner ReturnStatement over the enclosing arrow body", () => {
    const bodyNode = {
      type: AST_TYPE.BlockStatement,
      start: 25,
      end: 60,
    } as const satisfies AstNode;
    const arrowNode = {
      type: AST_TYPE.ArrowFunctionExpression,
      start: 10,
      end: 60,
      body: bodyNode,
    } as const satisfies AstNode;
    const path = [
      entry(arrowNode),
      entry(bodyNode, "body"),
      entry({ type: AST_TYPE.ReturnStatement, start: 30, end: 50 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 37, end: 38 }, "argument"),
    ] satisfies PathEntry[];
    expect(findCompletion(path)).toEqual({
      kind: "return",
      startOffset: 30,
      endOffset: 50,
    });
  });

  test("returns normal-completion at FunctionDeclaration with no inner exit statement", () => {
    const path = [
      entry({ type: AST_TYPE.FunctionDeclaration, start: 0, end: 80 }),
      entry({ type: AST_TYPE.BlockStatement, start: 15, end: 80 }, "body"),
      entry({ type: AST_TYPE.ExpressionStatement, start: 20, end: 35 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 20, end: 21 }, "expression"),
    ] satisfies PathEntry[];
    expect(findCompletion(path)).toEqual({ kind: "normal" });
  });

  test("returns normal-completion for a top-level identifier with no exit ancestor", () => {
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 100 }),
      entry({ type: AST_TYPE.ExpressionStatement, start: 0, end: 10 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 0, end: 5 }, "expression"),
    ] satisfies PathEntry[];
    expect(findCompletion(path)).toEqual({ kind: "normal" });
  });

  test("returns throw-completion when a ThrowStatement is on the path", () => {
    const path = [
      entry({ type: AST_TYPE.FunctionDeclaration, start: 0, end: 100 }),
      entry({ type: AST_TYPE.BlockStatement, start: 15, end: 100 }, "body"),
      entry({ type: AST_TYPE.ThrowStatement, start: 20, end: 50 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 26, end: 27 }, "argument"),
    ] satisfies PathEntry[];
    expect(findCompletion(path)).toEqual({
      kind: "throw",
      startOffset: 20,
      endOffset: 50,
    });
  });

  test("returns throw-completion for a top-level throw with no enclosing function", () => {
    // Top-level throws (e.g. in a module body) still consume the value: it
    // propagates as an unhandled exception. The visual-graph builder may
    // decide to route the use to module-level, but the analyzer must report
    // the throw completion faithfully.
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 60 }),
      entry({ type: AST_TYPE.ThrowStatement, start: 0, end: 30 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 6, end: 7 }, "argument"),
    ] satisfies PathEntry[];
    expect(findCompletion(path)).toEqual({
      kind: "throw",
      startOffset: 0,
      endOffset: 30,
    });
  });

  test("stops at an inner arrow boundary when a throw is in the enclosing function", () => {
    // An inner function would catch a re-thrown value before it reaches the
    // outer throw, so the identifier inside the arrow does not feed the outer
    // throw's [[Value]].
    const path = [
      entry({ type: AST_TYPE.FunctionDeclaration, start: 0, end: 100 }),
      entry({ type: AST_TYPE.BlockStatement, start: 15, end: 100 }, "body"),
      entry({ type: AST_TYPE.ThrowStatement, start: 20, end: 90 }, "body"),
      entry(
        {
          type: AST_TYPE.ArrowFunctionExpression,
          start: 26,
          end: 85,
        },
        "argument",
      ),
      entry({ type: AST_TYPE.Identifier, start: 30, end: 31 }, "body"),
    ] satisfies PathEntry[];
    expect(findCompletion(path)).toEqual({ kind: "normal" });
  });

  test("falls back to normal-completion when ReturnStatement offsets are missing", () => {
    const path = [
      entry({ type: AST_TYPE.FunctionDeclaration, start: 0, end: 100 }),
      entry({ type: AST_TYPE.ReturnStatement }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 27, end: 28 }, "argument"),
    ] satisfies PathEntry[];
    expect(findCompletion(path)).toEqual({ kind: "normal" });
  });

  test("falls back to normal-completion when ThrowStatement offsets are missing", () => {
    const path = [
      entry({ type: AST_TYPE.FunctionDeclaration, start: 0, end: 100 }),
      entry({ type: AST_TYPE.ThrowStatement }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 27, end: 28 }, "argument"),
    ] satisfies PathEntry[];
    expect(findCompletion(path)).toEqual({ kind: "normal" });
  });
});
