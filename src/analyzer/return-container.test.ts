import { describe, expect, test } from "vitest";

import type { AstNode } from "../ir/model.js";
import { findReturnContainer } from "./return-container.js";
import type { PathEntry } from "./walk.js";

function entry(node: AstNode, key: string | null = null): PathEntry {
  return { node, key };
}

describe("findReturnContainer", () => {
  test("returns the ReturnStatement span when one is on the path", () => {
    const path = [
      entry({ type: "FunctionDeclaration", start: 0, end: 100 }),
      entry({ type: "BlockStatement", start: 15, end: 100 }, "body"),
      entry({ type: "ReturnStatement", start: 20, end: 50 }, "body"),
      entry({ type: "Identifier", start: 27, end: 28 }, "argument"),
    ];
    expect(findReturnContainer(path)).toEqual({
      startOffset: 20,
      endOffset: 50,
    });
  });

  test("uses the body expression span when an arrow has an expression body", () => {
    const bodyNode: AstNode = {
      type: "BinaryExpression",
      start: 30,
      end: 50,
    };
    const arrowNode: AstNode = {
      type: "ArrowFunctionExpression",
      start: 10,
      end: 60,
      body: bodyNode,
    };
    const path = [
      entry(arrowNode),
      entry(bodyNode, "body"),
      entry({ type: "Identifier", start: 30, end: 31 }, "left"),
    ];
    expect(findReturnContainer(path)).toEqual({
      startOffset: 30,
      endOffset: 50,
    });
  });

  test("returns null for a block-body arrow with no inner ReturnStatement", () => {
    const bodyNode: AstNode = {
      type: "BlockStatement",
      start: 25,
      end: 60,
    };
    const arrowNode: AstNode = {
      type: "ArrowFunctionExpression",
      start: 10,
      end: 60,
      body: bodyNode,
    };
    const path = [
      entry(arrowNode),
      entry(bodyNode, "body"),
      entry({ type: "ExpressionStatement", start: 30, end: 50 }, "body"),
      entry({ type: "Identifier", start: 30, end: 31 }, "expression"),
    ];
    expect(findReturnContainer(path)).toBeNull();
  });

  test("prefers an inner ReturnStatement over the enclosing arrow body", () => {
    const bodyNode: AstNode = {
      type: "BlockStatement",
      start: 25,
      end: 60,
    };
    const arrowNode: AstNode = {
      type: "ArrowFunctionExpression",
      start: 10,
      end: 60,
      body: bodyNode,
    };
    const path = [
      entry(arrowNode),
      entry(bodyNode, "body"),
      entry({ type: "ReturnStatement", start: 30, end: 50 }, "body"),
      entry({ type: "Identifier", start: 37, end: 38 }, "argument"),
    ];
    expect(findReturnContainer(path)).toEqual({
      startOffset: 30,
      endOffset: 50,
    });
  });

  test("stops at FunctionDeclaration with no inner ReturnStatement", () => {
    const path = [
      entry({ type: "FunctionDeclaration", start: 0, end: 80 }),
      entry({ type: "BlockStatement", start: 15, end: 80 }, "body"),
      entry({ type: "ExpressionStatement", start: 20, end: 35 }, "body"),
      entry({ type: "Identifier", start: 20, end: 21 }, "expression"),
    ];
    expect(findReturnContainer(path)).toBeNull();
  });

  test("returns null for a top-level identifier with no return/arrow ancestor", () => {
    const path = [
      entry({ type: "Program", start: 0, end: 100 }),
      entry({ type: "ExpressionStatement", start: 0, end: 10 }, "body"),
      entry({ type: "Identifier", start: 0, end: 5 }, "expression"),
    ];
    expect(findReturnContainer(path)).toBeNull();
  });

  test("returns null when ReturnStatement offsets are missing", () => {
    const path = [
      entry({ type: "FunctionDeclaration", start: 0, end: 100 }),
      entry({ type: "ReturnStatement" }, "body"),
      entry({ type: "Identifier", start: 27, end: 28 }, "argument"),
    ];
    expect(findReturnContainer(path)).toBeNull();
  });
});
