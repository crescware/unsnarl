import { describe, expect, test } from "vitest";

import type { PathEntry } from "../boundary/eslint-scope/walk/path-entry.js";
import type { AstNode } from "../ir/primitive/ast-node.js";
import { AST_TYPE } from "../parser/ast-type.js";
import { findThrowContainer } from "./throw-container.js";

function entry(node: AstNode, key: string | null = null): PathEntry {
  return { node, key };
}

describe("findThrowContainer", () => {
  test("returns the ThrowStatement span when one is on the path", () => {
    const path = [
      entry({ type: AST_TYPE.FunctionDeclaration, start: 0, end: 100 }),
      entry({ type: AST_TYPE.BlockStatement, start: 15, end: 100 }, "body"),
      entry({ type: AST_TYPE.ThrowStatement, start: 20, end: 50 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 26, end: 27 }, "argument"),
    ] satisfies PathEntry[];
    expect(findThrowContainer(path)).toEqual({
      startOffset: 20,
      endOffset: 50,
    });
  });

  test("stops at FunctionDeclaration with no inner ThrowStatement", () => {
    const path = [
      entry({ type: AST_TYPE.FunctionDeclaration, start: 0, end: 80 }),
      entry({ type: AST_TYPE.BlockStatement, start: 15, end: 80 }, "body"),
      entry({ type: AST_TYPE.ExpressionStatement, start: 20, end: 35 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 20, end: 21 }, "expression"),
    ] satisfies PathEntry[];
    expect(findThrowContainer(path)).toEqual(null);
  });

  test("stops at the inner function when a throw is in an enclosing function", () => {
    // The outer Throw must not capture an identifier inside an inner function,
    // because the inner function would catch a re-thrown value before it
    // reaches the outer throw.
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
    expect(findThrowContainer(path)).toEqual(null);
  });

  test("returns null for a top-level identifier with no throw ancestor", () => {
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 100 }),
      entry({ type: AST_TYPE.ExpressionStatement, start: 0, end: 10 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 0, end: 5 }, "expression"),
    ] satisfies PathEntry[];
    expect(findThrowContainer(path)).toEqual(null);
  });

  test("returns null when ThrowStatement offsets are missing", () => {
    const path = [
      entry({ type: AST_TYPE.FunctionDeclaration, start: 0, end: 100 }),
      entry({ type: AST_TYPE.ThrowStatement }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 27, end: 28 }, "argument"),
    ] satisfies PathEntry[];
    expect(findThrowContainer(path)).toEqual(null);
  });

  test("returns the ThrowStatement span for a top-level throw with no enclosing function", () => {
    // Top-level throws (e.g. in a module body) still consume the value: it
    // propagates as an unhandled exception. The visual-graph builder may
    // decide to route the use to module-level, but the analyzer must report
    // the throw container faithfully.
    const path = [
      entry({ type: AST_TYPE.Program, start: 0, end: 60 }),
      entry({ type: AST_TYPE.ThrowStatement, start: 0, end: 30 }, "body"),
      entry({ type: AST_TYPE.Identifier, start: 6, end: 7 }, "argument"),
    ] satisfies PathEntry[];
    expect(findThrowContainer(path)).toEqual({
      startOffset: 0,
      endOffset: 30,
    });
  });
});
