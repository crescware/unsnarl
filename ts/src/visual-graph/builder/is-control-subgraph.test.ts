import { describe, expect, test } from "vitest";

import { SCOPE_TYPE, type ScopeType } from "../../analyzer/scope-type.js";
import type { BlockContext } from "../../ir/scope/block-context.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { asFilledString } from "../../util/filled-string.js";
import { isControlSubgraph } from "./is-control-subgraph.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

describe("isControlSubgraph", () => {
  test.each<{ type: ScopeType; expected: boolean }>([
    { type: SCOPE_TYPE.For, expected: true },
    { type: SCOPE_TYPE.Catch, expected: true },
    { type: SCOPE_TYPE.Switch, expected: true },
    { type: SCOPE_TYPE.Function, expected: false },
    { type: SCOPE_TYPE.Module, expected: false },
    { type: SCOPE_TYPE.Global, expected: false },
    { type: SCOPE_TYPE.Class, expected: false },
  ])("scope type $type -> $expected", ({ type, expected }) => {
    expect(isControlSubgraph({ ...baseScope(), type })).toEqual(expected);
  });

  test.each<{ name: string; ctx: BlockContext; expected: boolean }>([
    {
      name: asFilledString("block + IfStatement.consequent"),
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: asFilledString("block + IfStatement.alternate"),
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("alternate"),
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: asFilledString("block + TryStatement.block"),
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: asFilledString("block"),
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: asFilledString("block + TryStatement.finalizer"),
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: asFilledString("finalizer"),
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: asFilledString("block + SwitchStatement.cases"),
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: asFilledString("cases"),
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: asFilledString("block + WhileStatement.body"),
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.WhileStatement,
        key: asFilledString("body"),
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: asFilledString("block + DoWhileStatement.body"),
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.DoWhileStatement,
        key: asFilledString("body"),
        parentSpanOffset: 0,
      },
      expected: true,
    },
  ])("$name -> $expected", ({ ctx, expected }) => {
    expect(
      isControlSubgraph({
        ...baseScope(),
        type: SCOPE_TYPE.Block,
        blockContext: ctx,
      }),
    ).toEqual(expected);
  });

  test("plain block without blockContext -> true (renders as the generic 'block' subgraph)", () => {
    expect(
      isControlSubgraph({ ...baseScope(), type: SCOPE_TYPE.Block }),
    ).toEqual(true);
  });
});
