import { describe, expect, test } from "vitest";

import { SCOPE_TYPE, type ScopeType } from "../../analyzer/scope-type.js";
import type { BlockContext } from "../../ir/scope/block-context.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { asFilledString } from "../../util/filled-string.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import { controlSubgraphKindOf } from "./control-subgraph-kind-of.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

type Kind = VisualSubgraph["kind"] | null;

describe("controlSubgraphKindOf", () => {
  test.each<{ type: ScopeType; expected: Kind }>([
    { type: SCOPE_TYPE.Catch, expected: SUBGRAPH_KIND.Catch },
    { type: SCOPE_TYPE.For, expected: SUBGRAPH_KIND.For },
    { type: SCOPE_TYPE.Switch, expected: SUBGRAPH_KIND.Switch },
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
    ).toEqual(SUBGRAPH_KIND.Block);
  });

  test.each<{ ctx: BlockContext; expected: Kind }>([
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: asFilledString("block"),
        parentSpanOffset: 0,
      },
      expected: SUBGRAPH_KIND.Try,
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: asFilledString("finalizer"),
        parentSpanOffset: 0,
      },
      expected: SUBGRAPH_KIND.Finally,
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: asFilledString("handler"),
        parentSpanOffset: 0,
      },
      expected: SUBGRAPH_KIND.Block,
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 0,
      },
      expected: SUBGRAPH_KIND.If,
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("alternate"),
        parentSpanOffset: 0,
      },
      expected: SUBGRAPH_KIND.Else,
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("test"),
        parentSpanOffset: 0,
      },
      expected: SUBGRAPH_KIND.Block,
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: asFilledString("cases"),
        parentSpanOffset: 0,
      },
      expected: SUBGRAPH_KIND.Case,
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: asFilledString("discriminant"),
        parentSpanOffset: 0,
      },
      expected: SUBGRAPH_KIND.Block,
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.WhileStatement,
        key: asFilledString("body"),
        parentSpanOffset: 0,
      },
      expected: SUBGRAPH_KIND.While,
    },
    {
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.DoWhileStatement,
        key: asFilledString("body"),
        parentSpanOffset: 0,
      },
      expected: SUBGRAPH_KIND.DoWhile,
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
