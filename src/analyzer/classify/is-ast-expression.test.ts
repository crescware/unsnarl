import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../constants.js";
import { isAstExpression } from "./is-ast-expression.js";

describe("isAstExpression", () => {
  test("typed object → true", () => {
    expect(isAstExpression({ type: AST_TYPE.Identifier })).toBe(true);
  });

  test("primitives, null, missing/non-string type → false", () => {
    expect(isAstExpression(null)).toBe(false);
    expect(isAstExpression(undefined)).toBe(false);
    expect(isAstExpression("Identifier")).toBe(false);
    expect(isAstExpression({})).toBe(false);
    expect(isAstExpression({ type: 1 })).toBe(false);
  });
});
