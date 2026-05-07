import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { CATEGORY } from "../../serializer/category.js";
import { categoryOf } from "./category-of.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

describe("categoryOf", () => {
  test("function scope -> function", () => {
    expect(categoryOf({ ...baseScope(), type: SCOPE_TYPE.Function })).toBe(
      CATEGORY.Function,
    );
  });

  test("function-expression-name scope -> null (not a counted scope)", () => {
    expect(
      categoryOf({
        ...baseScope(),
        type: SCOPE_TYPE.Function,
        functionExpressionScope: true,
      }),
    ).toBeNull();
  });

  test("for scope -> for", () => {
    expect(categoryOf({ ...baseScope(), type: SCOPE_TYPE.For })).toBe(
      CATEGORY.For,
    );
  });

  test("switch scope -> switch", () => {
    expect(categoryOf({ ...baseScope(), type: SCOPE_TYPE.Switch })).toBe(
      CATEGORY.Switch,
    );
  });

  test("catch scope -> try-catch-finally", () => {
    expect(categoryOf({ ...baseScope(), type: SCOPE_TYPE.Catch })).toBe(
      CATEGORY.TryCatchFinally,
    );
  });

  test("block scope inside IfStatement -> if", () => {
    expect(
      categoryOf({
        ...baseScope(),
        type: SCOPE_TYPE.Block,
        blockContext: {
          ...baseBlockContext(),
          parentType: AST_TYPE.IfStatement,
          key: "consequent",
        },
      }),
    ).toBe(CATEGORY.If);
  });

  test("block scope inside ForStatement body -> for", () => {
    expect(
      categoryOf({
        ...baseScope(),
        type: SCOPE_TYPE.Block,
        blockContext: {
          ...baseBlockContext(),
          parentType: AST_TYPE.ForStatement,
          key: "body",
        },
      }),
    ).toBe(CATEGORY.For);
  });

  test("block scope inside WhileStatement body -> while", () => {
    expect(
      categoryOf({
        ...baseScope(),
        type: SCOPE_TYPE.Block,
        blockContext: {
          ...baseBlockContext(),
          parentType: AST_TYPE.WhileStatement,
          key: "body",
        },
      }),
    ).toBe(CATEGORY.While);
  });

  test("block scope inside TryStatement -> try-catch-finally", () => {
    expect(
      categoryOf({
        ...baseScope(),
        type: SCOPE_TYPE.Block,
        blockContext: {
          ...baseBlockContext(),
          parentType: AST_TYPE.TryStatement,
          key: "block",
        },
      }),
    ).toBe(CATEGORY.TryCatchFinally);
  });

  test("bare block scope -> block", () => {
    expect(
      categoryOf({
        ...baseScope(),
        type: SCOPE_TYPE.Block,
        blockContext: null,
      }),
    ).toBe(CATEGORY.Block);
  });

  test("module scope -> null", () => {
    expect(categoryOf({ ...baseScope(), type: SCOPE_TYPE.Module })).toBeNull();
  });
});
