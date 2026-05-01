import { describe, expect, test } from "vitest";

import type { BlockContext, ScopeType } from "../../ir/model.js";
import { shouldSubgraph } from "./should-subgraph.js";
import { makeBlockContext } from "./testing/make-block-context.js";
import { makeScope } from "./testing/make-scope.js";

describe("shouldSubgraph", () => {
  test.each<{
    name: string;
    type: ScopeType;
    blockContext: BlockContext | null;
    isOwner: boolean;
    expected: boolean;
  }>([
    {
      name: "function-subgraph owner on a plain block -> true",
      type: "block",
      blockContext: null,
      isOwner: true,
      expected: true,
    },
    {
      name: "control kind (for) without owner -> true",
      type: "for",
      blockContext: null,
      isOwner: false,
      expected: true,
    },
    {
      name: "branch block (if consequent) without owner -> true",
      type: "block",
      blockContext: makeBlockContext("IfStatement", "consequent", 0),
      isOwner: false,
      expected: true,
    },
    {
      name: "plain block without owner or branch context -> false",
      type: "block",
      blockContext: null,
      isOwner: false,
      expected: false,
    },
    {
      name: "module without owner -> false",
      type: "module",
      blockContext: null,
      isOwner: false,
      expected: false,
    },
    {
      name: "global without owner -> false",
      type: "global",
      blockContext: null,
      isOwner: false,
      expected: false,
    },
  ])("$name", ({ type, blockContext, isOwner, expected }) => {
    const scope = makeScope({ id: "s", type, blockContext });
    const owners = isOwner
      ? new Map([["s", "var"]])
      : new Map<string, string>();
    expect(shouldSubgraph(scope, owners)).toBe(expected);
  });
});
