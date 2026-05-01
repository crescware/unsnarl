import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../parser/ast-type.js";
import { isAstNode } from "./is-ast-node.js";

describe("isAstNode (declare)", () => {
  test("object with string `type` is an AST node", () => {
    expect(isAstNode({ type: AST_TYPE.Identifier })).toBe(true);
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
