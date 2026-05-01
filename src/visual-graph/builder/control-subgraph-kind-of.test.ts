import { describe, expect, test } from "vitest";

import { AST_TYPE } from "../../ast-type.js";
import type { BlockContext, ScopeType } from "../../ir/model.js";
import { SCOPE_TYPE } from "../../scope-type.js";
import type { VisualSubgraph } from "../model.js";
import { controlSubgraphKindOf } from "./control-subgraph-kind-of.js";
import { makeBlockContext } from "./testing/make-block-context.js";
import { makeScope } from "./testing/make-scope.js";

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
    expect(controlSubgraphKindOf(makeScope({ type }))).toBe(expected);
  });

  test("returns null for a block scope without blockContext", () => {
    expect(
      controlSubgraphKindOf(makeScope({ type: SCOPE_TYPE.Block })),
    ).toBeNull();
  });

  test.each<{ ctx: BlockContext; expected: Kind }>([
    {
      ctx: makeBlockContext(AST_TYPE.TryStatement, "block", 0),
      expected: "try",
    },
    {
      ctx: makeBlockContext(AST_TYPE.TryStatement, "finalizer", 0),
      expected: "finally",
    },
    {
      ctx: makeBlockContext(AST_TYPE.TryStatement, "handler", 0),
      expected: null,
    },
    {
      ctx: makeBlockContext(AST_TYPE.IfStatement, "consequent", 0),
      expected: "if",
    },
    {
      ctx: makeBlockContext(AST_TYPE.IfStatement, "alternate", 0),
      expected: "else",
    },
    { ctx: makeBlockContext(AST_TYPE.IfStatement, "test", 0), expected: null },
    {
      ctx: makeBlockContext(AST_TYPE.SwitchStatement, "cases", 0),
      expected: "case",
    },
    {
      ctx: makeBlockContext(AST_TYPE.SwitchStatement, "discriminant", 0),
      expected: null,
    },
    { ctx: makeBlockContext("WhileStatement", "body", 0), expected: null },
  ])(
    "block + parentType=$ctx.parentType key=$ctx.key -> $expected",
    ({ ctx, expected }) => {
      expect(
        controlSubgraphKindOf(
          makeScope({ type: SCOPE_TYPE.Block, blockContext: ctx }),
        ),
      ).toBe(expected);
    },
  );
});
