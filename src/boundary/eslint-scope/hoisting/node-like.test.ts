import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../../parser/ast-type.js";
import { isNodeLike } from "./node-like.js";

describe("isNodeLike", () => {
  test("typed object → true", () => {
    expect(isNodeLike({ type: AST_TYPE.Identifier })).toEqual(true);
  });

  test("primitives → false", () => {
    expect(isNodeLike(null)).toEqual(false);
    expect(isNodeLike(undefined)).toEqual(false);
    expect(isNodeLike("Identifier")).toEqual(false);
    expect(isNodeLike(0)).toEqual(false);
  });

  test("missing or non-string `type` → false", () => {
    expect(isNodeLike({})).toEqual(false);
    expect(isNodeLike({ type: 1 })).toEqual(false);
    expect(isNodeLike({ type: null })).toEqual(false);
  });
});
