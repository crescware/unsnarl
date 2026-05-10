import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../../parser/ast-type.js";
import { isAstNode } from "./is-ast-node.js";

describe("isAstNode", () => {
  test("object with string `type` is an AST node", () => {
    expect(isAstNode({ type: AST_TYPE.Identifier })).toEqual(true);
    expect(isAstNode({ type: AST_TYPE.Literal, value: 1 })).toEqual(true);
  });

  test("object missing `type` is rejected", () => {
    expect(isAstNode({})).toEqual(false);
    expect(isAstNode({ name: "x" })).toEqual(false);
  });

  test("object whose `type` is not a string is rejected", () => {
    expect(isAstNode({ type: 1 })).toEqual(false);
    expect(isAstNode({ type: null })).toEqual(false);
  });

  test("primitives are rejected", () => {
    expect(isAstNode(null)).toEqual(false);
    expect(isAstNode(undefined)).toEqual(false);
    expect(isAstNode(0)).toEqual(false);
    expect(isAstNode("Identifier")).toEqual(false);
    expect(isAstNode(true)).toEqual(false);
  });

  test("arrays are rejected (no string `type` key)", () => {
    expect(isAstNode([])).toEqual(false);
    expect(isAstNode([{ type: AST_TYPE.Identifier }])).toEqual(false);
  });
});
