import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../parser/ast-type.js";
import { isNodeLike } from "./node-like.js";

describe("isNodeLike", () => {
  test("typed object → true", () => {
    expect(isNodeLike({ type: AST_TYPE.Identifier })).toBe(true);
  });

  test("primitives → false", () => {
    expect(isNodeLike(null)).toBe(false);
    expect(isNodeLike(undefined)).toBe(false);
    expect(isNodeLike("Identifier")).toBe(false);
    expect(isNodeLike(0)).toBe(false);
  });

  test("missing or non-string `type` → false", () => {
    expect(isNodeLike({})).toBe(false);
    expect(isNodeLike({ type: 1 })).toBe(false);
    expect(isNodeLike({ type: null })).toBe(false);
  });
});
