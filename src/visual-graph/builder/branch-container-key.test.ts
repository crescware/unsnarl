import { describe, expect, test } from "vitest";

import { branchContainerKey } from "./branch-container-key.js";
import { makeBlockContext } from "./testing/make-block-context.js";
import { makeScope } from "./testing/make-scope.js";

describe("branchContainerKey", () => {
  test("returns null when blockContext is missing", () => {
    expect(branchContainerKey(makeScope())).toBeNull();
  });

  test.each([
    {
      name: "switch cases -> switch:<upper>:<offset>",
      upper: "outer" as string | null,
      ctx: makeBlockContext("SwitchStatement", "cases", 12),
      expected: "switch:outer:12" as string | null,
    },
    {
      name: "if consequent -> if:<upper>:<offset>",
      upper: "outer" as string | null,
      ctx: makeBlockContext("IfStatement", "consequent", 3),
      expected: "if:outer:3",
    },
    {
      name: "if alternate -> if:<upper>:<offset>",
      upper: "outer" as string | null,
      ctx: makeBlockContext("IfStatement", "alternate", 3),
      expected: "if:outer:3",
    },
    {
      name: "switch with non-cases key -> null",
      upper: "outer" as string | null,
      ctx: makeBlockContext("SwitchStatement", "discriminant", 7),
      expected: null,
    },
    {
      name: "if with key other than consequent/alternate -> null",
      upper: "outer" as string | null,
      ctx: makeBlockContext("IfStatement", "test", 3),
      expected: null,
    },
    {
      name: "unrelated parent type -> null",
      upper: "outer" as string | null,
      ctx: makeBlockContext("ForStatement", "body", 5),
      expected: null,
    },
    {
      name: "null upper renders as empty string in the key",
      upper: null,
      ctx: makeBlockContext("IfStatement", "consequent", 1),
      expected: "if::1",
    },
  ])("$name", ({ upper, ctx, expected }) => {
    const scope = makeScope({ upper, blockContext: ctx });
    expect(branchContainerKey(scope)).toBe(expected);
  });
});
