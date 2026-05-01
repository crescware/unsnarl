import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../ast-type.js";
import type { BlockContext, ScopeType } from "../../ir/model.js";
import { SCOPE_TYPE } from "../../scope-type.js";
import { isControlSubgraph } from "./is-control-subgraph.js";
import { makeBlockContext } from "./testing/make-block-context.js";
import { makeScope } from "./testing/make-scope.js";

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
    expect(isControlSubgraph(makeScope({ type }))).toBe(expected);
  });

  test.each<{ name: string; ctx: BlockContext; expected: boolean }>([
    {
      name: "block + IfStatement.consequent",
      ctx: makeBlockContext(AST_TYPE.IfStatement, "consequent", 0),
      expected: true,
    },
    {
      name: "block + IfStatement.alternate",
      ctx: makeBlockContext(AST_TYPE.IfStatement, "alternate", 0),
      expected: true,
    },
    {
      name: "block + TryStatement.block",
      ctx: makeBlockContext(AST_TYPE.TryStatement, "block", 0),
      expected: true,
    },
    {
      name: "block + TryStatement.finalizer",
      ctx: makeBlockContext(AST_TYPE.TryStatement, "finalizer", 0),
      expected: true,
    },
    {
      name: "block + SwitchStatement.cases",
      ctx: makeBlockContext(AST_TYPE.SwitchStatement, "cases", 0),
      expected: true,
    },
    {
      name: "block + WhileStatement.body",
      ctx: makeBlockContext("WhileStatement", "body", 0),
      expected: false,
    },
  ])("$name -> $expected", ({ ctx, expected }) => {
    expect(
      isControlSubgraph(
        makeScope({ type: SCOPE_TYPE.Block, blockContext: ctx }),
      ),
    ).toBe(expected);
  });

  test("plain block without blockContext -> false", () => {
    expect(isControlSubgraph(makeScope({ type: SCOPE_TYPE.Block }))).toBe(
      false,
    );
  });
});
