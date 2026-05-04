import { describe, expect, test } from "vitest";

import { SCOPE_TYPE, type ScopeType } from "../../analyzer/scope-type.js";
import type { BlockContext } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
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
    expect(isControlSubgraph({ ...baseScope(), type })).toBe(expected);
  });

  test.each<{ name: string; ctx: BlockContext; expected: boolean }>([
    {
      name: "block + IfStatement.consequent",
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "consequent",
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: "block + IfStatement.alternate",
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "alternate",
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: "block + TryStatement.block",
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: "block",
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: "block + TryStatement.finalizer",
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.TryStatement,
        key: "finalizer",
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: "block + SwitchStatement.cases",
      ctx: {
        ...baseBlockContext(),
        parentType: AST_TYPE.SwitchStatement,
        key: "cases",
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: "block + WhileStatement.body",
      ctx: {
        ...baseBlockContext(),
        parentType: "WhileStatement",
        key: "body",
        parentSpanOffset: 0,
      },
      expected: false,
    },
  ])("$name -> $expected", ({ ctx, expected }) => {
    expect(
      isControlSubgraph({
        ...baseScope(),
        type: SCOPE_TYPE.Block,
        blockContext: ctx,
      }),
    ).toBe(expected);
  });

  test("plain block without blockContext -> false", () => {
    expect(isControlSubgraph({ ...baseScope(), type: SCOPE_TYPE.Block })).toBe(
      false,
    );
  });
});
