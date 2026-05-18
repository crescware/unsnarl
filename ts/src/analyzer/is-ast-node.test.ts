import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../parser/ast-type.js";
import { isAstNode } from "./is-ast-node.js";

describe("isAstNode (owner)", () => {
  test("typed object → true", () => {
    expect(isAstNode({ type: AST_TYPE.Identifier })).toEqual(true);
  });

  test("primitives and missing-type objects → false", () => {
    expect(isAstNode(null)).toEqual(false);
    expect(isAstNode(undefined)).toEqual(false);
    expect(isAstNode("Identifier")).toEqual(false);
    expect(isAstNode({})).toEqual(false);
    expect(isAstNode({ type: 1 })).toEqual(false);
  });
});
