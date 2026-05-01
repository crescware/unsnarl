import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../ast-type.js";
import type { AstNode } from "../../ir/model.js";
import { isComputed } from "./is-computed.js";

describe("isComputed", () => {
  test("returns true when computed === true", () => {
    expect(
      isComputed({
        type: AST_TYPE.MemberExpression,
        computed: true,
      } as unknown as AstNode),
    ).toBe(true);
  });

  test("returns false when computed === false", () => {
    expect(
      isComputed({
        type: AST_TYPE.MemberExpression,
        computed: false,
      } as unknown as AstNode),
    ).toBe(false);
  });

  test("returns false when computed is missing", () => {
    expect(
      isComputed({ type: AST_TYPE.MemberExpression } as unknown as AstNode),
    ).toBe(false);
  });

  test("returns false when computed is truthy but not strictly true", () => {
    expect(isComputed({ type: "X", computed: 1 } as unknown as AstNode)).toBe(
      false,
    );
  });
});
