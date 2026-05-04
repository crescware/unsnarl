import { describe, expect, test } from "vitest";

import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { findReturnContainer } from "./return-container.js";
import type { PathEntry } from "./walk/path-entry.js";

function entry(node: AstNode, key: string | null = null): PathEntry {
  return { node, key };
}

describe("findReturnContainer", () => {
  test("returns the ReturnStatement span when one is on the path", () => {
    const path = [
      entry({ type: AST_TYPE.FunctionDeclaration, start: 0, end: 100 }),
      entry({ type: AST_TYPE.BlockStatement, start: 15, end: 100 }, "body"),
      entry({ type: AST_TYPE.ReturnStatement, start: 20, end: 50 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 27, end: 28 }, "argument"),
    ] satisfies PathEntry[];
    expect(findReturnContainer(path)).toEqual({
      startOffset: 20,
      endOffset: 50,
    });
  });

  test("uses the body expression span when an arrow has an expression body", () => {
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
    expect(findReturnContainer(path)).toEqual({
      startOffset: 30,
      endOffset: 50,
    });
  });

  test("returns null for a block-body arrow with no inner ReturnStatement", () => {
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
    expect(findReturnContainer(path)).toBeNull();
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
    expect(findReturnContainer(path)).toEqual({
      startOffset: 30,
      endOffset: 50,
    });
  });

  test("stops at FunctionDeclaration with no inner ReturnStatement", () => {
    const path = [
      entry({ type: AST_TYPE.FunctionDeclaration, start: 0, end: 80 }),
      entry({ type: AST_TYPE.BlockStatement, start: 15, end: 80 }, "body"),
      entry({ type: AST_TYPE.ExpressionStatement, start: 20, end: 35 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 20, end: 21 }, "expression"),
    ] satisfies PathEntry[];
    expect(findReturnContainer(path)).toBeNull();
  });

  test("returns null for a top-level identifier with no return/arrow ancestor", () => {
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 100 }),
      entry({ type: AST_TYPE.ExpressionStatement, start: 0, end: 10 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 0, end: 5 }, "expression"),
    ] satisfies PathEntry[];
    expect(findReturnContainer(path)).toBeNull();
  });

  test("returns null when ReturnStatement offsets are missing", () => {
    const path = [
      entry({ type: AST_TYPE.FunctionDeclaration, start: 0, end: 100 }),
      entry({ type: AST_TYPE.ReturnStatement }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 27, end: 28 }, "argument"),
    ] satisfies PathEntry[];
    expect(findReturnContainer(path)).toBeNull();
  });
});
