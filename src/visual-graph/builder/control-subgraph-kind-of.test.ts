import { describe, expect, test } from "vitest";

import { SCOPE_TYPE, type ScopeType } from "../../analyzer/scope-type.js";
import type { BlockContext } from "../../ir/scope/block-context.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { asFilledString } from "../../util/filled-string.js";
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
    expect(controlSubgraphKindOf({ ...baseScope(), type })).toEqual(expected);
  });

  test("returns 'block' for a block scope without blockContext (bare {})", () => {
    expect(
      controlSubgraphKindOf({ ...baseScope(), type: SCOPE_TYPE.Block }),
    ).toEqual("block");
  });

  test.each<{ ctx: BlockContext; expected: Kind }>([
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: asFilledString("block"),
        parentSpanOffset: 0,
      },
      expected: "try",
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: asFilledString("finalizer"),
        parentSpanOffset: 0,
      },
      expected: "finally",
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: asFilledString("handler"),
        parentSpanOffset: 0,
      },
      expected: "block",
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 0,
      },
      expected: "if",
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("alternate"),
        parentSpanOffset: 0,
      },
      expected: "else",
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("test"),
        parentSpanOffset: 0,
      },
      expected: "block",
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: asFilledString("cases"),
        parentSpanOffset: 0,
      },
      expected: "case",
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: asFilledString("discriminant"),
        parentSpanOffset: 0,
      },
      expected: "block",
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.WhileStatement,
        key: asFilledString("body"),
        parentSpanOffset: 0,
      },
      expected: "while",
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.DoWhileStatement,
        key: asFilledString("body"),
        parentSpanOffset: 0,
      },
      expected: "do-while",
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
      ).toEqual(expected);
    },
  );
});
