import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../../parser/ast-type.js";
import { isAstNode } from "./is-ast-node.js";

describe("isAstNode (declare)", () => {
  test("object with string `type` is an AST node", () => {
    expect(isAstNode({ type: AST_TYPE.Identifier })).toEqual(true);
  });

  test("non-objects are rejected", () => {
    expect(isAstNode(null)).toEqual(false);
    expect(isAstNode(undefined)).toEqual(false);
    expect(isAstNode("Identifier")).toEqual(false);
    expect(isAstNode(0)).toEqual(false);
  });

  test("missing or non-string `type` is rejected", () => {
    expect(isAstNode({})).toEqual(false);
    expect(isAstNode({ type: 1 })).toEqual(false);
  });
});
