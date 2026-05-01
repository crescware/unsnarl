import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../parser/ast-type.js";
import { isIdentifierNode } from "./is-identifier-node.js";

describe("isIdentifierNode", () => {
  test("Identifier node → true", () => {
    expect(isIdentifierNode({ type: AST_TYPE.Identifier, name: "x" })).toBe(
      true,
    );
  });

  test("non-Identifier node → false", () => {
    expect(isIdentifierNode({ type: AST_TYPE.Literal, value: 1 })).toBe(false);
  });

  test("non-node value → false", () => {
    expect(isIdentifierNode(null)).toBe(false);
    expect(isIdentifierNode("Identifier")).toBe(false);
    expect(isIdentifierNode({})).toBe(false);
  });
});
