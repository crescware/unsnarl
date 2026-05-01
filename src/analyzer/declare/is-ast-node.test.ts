import { describe, expect, test } from "vitest";

import { isAstNode } from "./is-ast-node.js";

describe("isAstNode (declare)", () => {
  test("object with string `type` is an AST node", () => {
    expect(isAstNode({ type: "Identifier" })).toBe(true);
  });

  test("non-objects are rejected", () => {
    expect(isAstNode(null)).toBe(false);
    expect(isAstNode(undefined)).toBe(false);
    expect(isAstNode("Identifier")).toBe(false);
    expect(isAstNode(0)).toBe(false);
  });

  test("missing or non-string `type` is rejected", () => {
    expect(isAstNode({})).toBe(false);
    expect(isAstNode({ type: 1 })).toBe(false);
  });
});
