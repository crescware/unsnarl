import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../../parser/ast-type.js";
import { isIdentifierNode } from "./is-identifier-node.js";

describe("isIdentifierNode", () => {
  test("Identifier node → true", () => {
    expect(isIdentifierNode({ type: AST_TYPE.Identifier, name: "x" })).toEqual(
      true,
    );
  });

  test("non-Identifier node → false", () => {
    expect(isIdentifierNode({ type: AST_TYPE.Literal, value: 1 })).toEqual(
      false,
    );
  });

  test("non-node value → false", () => {
    expect(isIdentifierNode(null)).toEqual(false);
    expect(isIdentifierNode("Identifier")).toEqual(false);
    expect(isIdentifierNode({})).toEqual(false);
  });
});
