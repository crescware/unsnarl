import { describe, expect, test } from "vitest";

import type { BlockContext, ScopeType } from "../../ir/model.js";
import { isControlSubgraph } from "./is-control-subgraph.js";
import { makeBlockContext } from "./testing/make-block-context.js";
import { makeScope } from "./testing/make-scope.js";

describe("isControlSubgraph", () => {
  test.each<{ type: ScopeType; expected: boolean }>([
    { type: "for", expected: true },
    { type: "catch", expected: true },
    { type: "switch", expected: true },
    { type: "function", expected: false },
    { type: "module", expected: false },
    { type: "global", expected: false },
    { type: "class", expected: false },
  ])("scope type $type -> $expected", ({ type, expected }) => {
    expect(isControlSubgraph(makeScope({ type }))).toBe(expected);
  });

  test.each<{ name: string; ctx: BlockContext; expected: boolean }>([
    {
      name: "block + IfStatement.consequent",
      ctx: makeBlockContext("IfStatement", "consequent", 0),
      expected: true,
    },
    {
      name: "block + IfStatement.alternate",
      ctx: makeBlockContext("IfStatement", "alternate", 0),
      expected: true,
    },
    {
      name: "block + TryStatement.block",
      ctx: makeBlockContext("TryStatement", "block", 0),
      expected: true,
    },
    {
      name: "block + TryStatement.finalizer",
      ctx: makeBlockContext("TryStatement", "finalizer", 0),
      expected: true,
    },
    {
      name: "block + SwitchStatement.cases",
      ctx: makeBlockContext("SwitchStatement", "cases", 0),
      expected: true,
    },
    {
      name: "block + WhileStatement.body",
      ctx: makeBlockContext("WhileStatement", "body", 0),
      expected: false,
    },
  ])("$name -> $expected", ({ ctx, expected }) => {
    expect(
      isControlSubgraph(makeScope({ type: "block", blockContext: ctx })),
    ).toBe(expected);
  });

  test("plain block without blockContext -> false", () => {
    expect(isControlSubgraph(makeScope({ type: "block" }))).toBe(false);
  });
});
