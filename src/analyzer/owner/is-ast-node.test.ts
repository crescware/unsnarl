import { describe, expect, test } from "vitest";

import { isAstNode } from "./is-ast-node.js";

describe("isAstNode (owner)", () => {
  test("typed object → true", () => {
    expect(isAstNode({ type: "Identifier" })).toBe(true);
  });

  test("primitives and missing-type objects → false", () => {
    expect(isAstNode(null)).toBe(false);
    expect(isAstNode(undefined)).toBe(false);
    expect(isAstNode("Identifier")).toBe(false);
    expect(isAstNode({})).toBe(false);
    expect(isAstNode({ type: 1 })).toBe(false);
  });
});
