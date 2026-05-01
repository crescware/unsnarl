import { describe, expect, test } from "vitest";

import type { BlockContext, ScopeType } from "../../ir/model.js";
import type { VisualSubgraph } from "../model.js";
import { controlSubgraphKindOf } from "./control-subgraph-kind-of.js";
import { makeBlockContext } from "./testing/make-block-context.js";
import { makeScope } from "./testing/make-scope.js";

type Kind = VisualSubgraph["kind"] | null;

describe("controlSubgraphKindOf", () => {
  test.each<{ type: ScopeType; expected: Kind }>([
    { type: "catch", expected: "catch" },
    { type: "for", expected: "for" },
    { type: "switch", expected: "switch" },
    { type: "function", expected: null },
    { type: "module", expected: null },
    { type: "global", expected: null },
    { type: "class", expected: null },
  ])("scope type $type maps to $expected", ({ type, expected }) => {
    expect(controlSubgraphKindOf(makeScope({ type }))).toBe(expected);
  });

  test("returns null for a block scope without blockContext", () => {
    expect(controlSubgraphKindOf(makeScope({ type: "block" }))).toBeNull();
  });

  test.each<{ ctx: BlockContext; expected: Kind }>([
    { ctx: makeBlockContext("TryStatement", "block", 0), expected: "try" },
    {
      ctx: makeBlockContext("TryStatement", "finalizer", 0),
      expected: "finally",
    },
    { ctx: makeBlockContext("TryStatement", "handler", 0), expected: null },
    { ctx: makeBlockContext("IfStatement", "consequent", 0), expected: "if" },
    { ctx: makeBlockContext("IfStatement", "alternate", 0), expected: "else" },
    { ctx: makeBlockContext("IfStatement", "test", 0), expected: null },
    { ctx: makeBlockContext("SwitchStatement", "cases", 0), expected: "case" },
    {
      ctx: makeBlockContext("SwitchStatement", "discriminant", 0),
      expected: null,
    },
    { ctx: makeBlockContext("WhileStatement", "body", 0), expected: null },
  ])(
    "block + parentType=$ctx.parentType key=$ctx.key -> $expected",
    ({ ctx, expected }) => {
      expect(
        controlSubgraphKindOf(makeScope({ type: "block", blockContext: ctx })),
      ).toBe(expected);
    },
  );
});
