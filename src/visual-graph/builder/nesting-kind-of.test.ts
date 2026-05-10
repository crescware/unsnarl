import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { NESTING_KIND } from "../../serializer/nesting-kind.js";
import { nestingKindOf } from "./nesting-kind-of.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

describe("nestingKindOf", () => {
  test("function scope -> function", () => {
    expect(
      nestingKindOf({ ...baseScope(), type: SCOPE_TYPE.Function }),
    ).toEqual(NESTING_KIND.Function);
  });

  test("function-expression-name scope -> null (not a counted scope)", () => {
    expect(
      nestingKindOf({
        ...baseScope(),
        type: SCOPE_TYPE.Function,
        functionExpressionScope: true,
      }),
    ).toEqual(null);
  });

  test("for scope -> for", () => {
    expect(nestingKindOf({ ...baseScope(), type: SCOPE_TYPE.For })).toEqual(
      NESTING_KIND.For,
    );
  });

  test("switch scope -> switch", () => {
    expect(nestingKindOf({ ...baseScope(), type: SCOPE_TYPE.Switch })).toEqual(
      NESTING_KIND.Switch,
    );
  });

  test("catch scope -> try-catch-finally", () => {
    expect(nestingKindOf({ ...baseScope(), type: SCOPE_TYPE.Catch })).toEqual(
      NESTING_KIND.TryCatchFinally,
    );
  });

  test("block scope inside IfStatement -> if", () => {
    expect(
      nestingKindOf({
        ...baseScope(),
        type: SCOPE_TYPE.Block,
        blockContext: {
          ...baseBlockContext(),
          parentType: AST_TYPE.IfStatement,
          key: "consequent",
        },
      }),
    ).toEqual(NESTING_KIND.If);
  });

  test("block scope inside ForStatement body -> for", () => {
    expect(
      nestingKindOf({
        ...baseScope(),
        type: SCOPE_TYPE.Block,
        blockContext: {
          ...baseBlockContext(),
          parentType: AST_TYPE.ForStatement,
          key: "body",
        },
      }),
    ).toEqual(NESTING_KIND.For);
  });

  test("block scope inside WhileStatement body -> while", () => {
    expect(
      nestingKindOf({
        ...baseScope(),
        type: SCOPE_TYPE.Block,
        blockContext: {
          ...baseBlockContext(),
          parentType: AST_TYPE.WhileStatement,
          key: "body",
        },
      }),
    ).toEqual(NESTING_KIND.While);
  });

  test("block scope inside TryStatement -> try-catch-finally", () => {
    expect(
      nestingKindOf({
        ...baseScope(),
        type: SCOPE_TYPE.Block,
        blockContext: {
          ...baseBlockContext(),
          parentType: AST_TYPE.TryStatement,
          key: "block",
        },
      }),
    ).toEqual(NESTING_KIND.TryCatchFinally);
  });

  test("bare block scope -> block", () => {
    expect(
      nestingKindOf({
        ...baseScope(),
        type: SCOPE_TYPE.Block,
        blockContext: null,
      }),
    ).toEqual(NESTING_KIND.Block);
  });

  test("module scope -> null", () => {
    expect(nestingKindOf({ ...baseScope(), type: SCOPE_TYPE.Module })).toEqual(
      null,
    );
  });
});
