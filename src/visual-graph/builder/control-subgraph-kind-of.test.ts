import { describe, expect, test } from "vitest";

import { SCOPE_TYPE, type ScopeType } from "../../analyzer/scope-type.js";
import type { BlockContext } from "../../ir/scope/block-context.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import { controlSubgraphKindOf } from "./control-subgraph-kind-of.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

type Kind = VisualSubgraph["kind"] | null;

describe("controlSubgraphKindOf", () => {
  test.each<{ type: ScopeType; expected: Kind }>([
    { type: SCOPE_TYPE.Catch, expected: "catch" },
    { type: SCOPE_TYPE.For, expected: "for" },
    { type: SCOPE_TYPE.Switch, expected: "switch" },
    { type: SCOPE_TYPE.Function, expected: null },
    { type: SCOPE_TYPE.Module, expected: null },
    { type: SCOPE_TYPE.Global, expected: null },
    { type: SCOPE_TYPE.Class, expected: null },
  ])("scope type $type maps to $expected", ({ type, expected }) => {
    expect(controlSubgraphKindOf({ ...baseScope(), type })).toBe(expected);
  });

  test("returns null for a block scope without blockContext", () => {
    expect(
      controlSubgraphKindOf({ ...baseScope(), type: SCOPE_TYPE.Block }),
    ).toBeNull();
  });

  test.each<{ ctx: BlockContext; expected: Kind }>([
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: "block",
        parentSpanOffset: 0,
      },
      expected: "try",
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: "finalizer",
        parentSpanOffset: 0,
      },
      expected: "finally",
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: "handler",
        parentSpanOffset: 0,
      },
      expected: null,
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "consequent",
        parentSpanOffset: 0,
      },
      expected: "if",
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "alternate",
        parentSpanOffset: 0,
      },
      expected: "else",
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "test",
        parentSpanOffset: 0,
      },
      expected: null,
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: "cases",
        parentSpanOffset: 0,
      },
      expected: "case",
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: "discriminant",
        parentSpanOffset: 0,
      },
      expected: null,
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: "WhileStatement",
        key: "body",
        parentSpanOffset: 0,
      },
      expected: null,
    },
  ])(
    "block + parentType=$ctx.parentType key=$ctx.key -> $expected",
    ({ ctx, expected }) => {
      expect(
        controlSubgraphKindOf({
          ...baseScope(),
          type: SCOPE_TYPE.Block,
          blockContext: ctx,
        }),
      ).toBe(expected);
    },
  );
});
