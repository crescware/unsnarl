import { describe, expect, test } from "vitest";

import type { BlockContext } from "../../ir/scope/block-context.js";
import { asScopeId } from "../../ir/serialized/scope-id.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { asFilledString } from "../../util/filled-string.js";
import { isBranchScope } from "./is-branch-scope.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

describe("isBranchScope", () => {
  test.each<{ name: string; ctx: BlockContext | null; expected: boolean }>([
    {
      name: asFilledString("if consequent block scope -> true"),
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: asFilledString("if alternate block scope -> true"),
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("alternate"),
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: asFilledString("switch case scope -> true"),
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: asFilledString("cases"),
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: asFilledString(
        "try block scope -> true (try and catch are sibling branches)",
      ),
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: asFilledString("block"),
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: asFilledString(
        "try handler scope -> true (try and catch are sibling branches)",
      ),
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: asFilledString("handler"),
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: asFilledString(
        "try finalizer scope -> false (finally is post-merge, not branch)",
      ),
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: asFilledString("finalizer"),
        parentSpanOffset: 0,
      },
      expected: false,
    },
    {
      name: asFilledString("no blockContext -> false"),
      ctx: null,
      expected: false,
    },
  ])("$name", ({ ctx, expected }) => {
    const scope = { ...baseScope(), id: asScopeId("s"), blockContext: ctx };
    const map = new Map([[scope.id, scope]]);
    expect(isBranchScope("s", map)).toEqual(expected);
  });

  test("scope id missing from the map -> false", () => {
    expect(isBranchScope("missing", new Map())).toEqual(false);
  });
});
