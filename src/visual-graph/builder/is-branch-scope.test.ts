import { describe, expect, test } from "vitest";

import type { BlockContext } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { isBranchScope } from "./is-branch-scope.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

describe("isBranchScope", () => {
  test.each<{ name: string; ctx: BlockContext | null; expected: boolean }>([
    {
      name: "if consequent block scope -> true",
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "consequent",
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: "if alternate block scope -> true",
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "alternate",
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: "switch case scope -> true",
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: "cases",
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: "try block scope -> true (try and catch are sibling branches)",
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: "block",
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: "try handler scope -> true (try and catch are sibling branches)",
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: "handler",
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: "try finalizer scope -> false (finally is post-merge, not branch)",
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: "finalizer",
        parentSpanOffset: 0,
      },
      expected: false,
    },
    {
      name: "no blockContext -> false",
      ctx: null,
      expected: false,
    },
  ])("$name", ({ ctx, expected }) => {
    const scope = { ...baseScope(), id: "s", blockContext: ctx };
    const map = new Map([[scope.id, scope]]);
    expect(isBranchScope("s", map)).toBe(expected);
  });

  test("scope id missing from the map -> false", () => {
    expect(isBranchScope("missing", new Map())).toBe(false);
  });
});
