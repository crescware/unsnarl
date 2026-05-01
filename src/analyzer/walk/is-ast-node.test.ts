import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../parser/ast-type.js";
import { isAstNode } from "./is-ast-node.js";

describe("isAstNode", () => {
  test("object with string `type` is an AST node", () => {
    expect(isAstNode({ type: AST_TYPE.Identifier })).toBe(true);
    expect(isAstNode({ type: AST_TYPE.Literal, value: 1 })).toBe(true);
  });

  test("object missing `type` is rejected", () => {
    expect(isAstNode({})).toBe(false);
    expect(isAstNode({ name: "x" })).toBe(false);
  });

  test("object whose `type` is not a string is rejected", () => {
    expect(isAstNode({ type: 1 })).toBe(false);
    expect(isAstNode({ type: null })).toBe(false);
  });

  test("primitives are rejected", () => {
    expect(isAstNode(null)).toBe(false);
    expect(isAstNode(undefined)).toBe(false);
    expect(isAstNode(0)).toBe(false);
    expect(isAstNode("Identifier")).toBe(false);
    expect(isAstNode(true)).toBe(false);
  });

  test("arrays are rejected (no string `type` key)", () => {
    expect(isAstNode([])).toBe(false);
    expect(isAstNode([{ type: AST_TYPE.Identifier }])).toBe(false);
  });
});
