import { describe, expect, test } from "vitest";

import { AST_TYPE, SCOPE_TYPE } from "../../constants.js";
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
      type: SCOPE_TYPE.Block,
      blockContext: null,
      isOwner: true,
      expected: true,
    },
    {
      name: "control kind (for) without owner -> true",
      type: SCOPE_TYPE.For,
      blockContext: null,
      isOwner: false,
      expected: true,
    },
    {
      name: "branch block (if consequent) without owner -> true",
      type: SCOPE_TYPE.Block,
      blockContext: makeBlockContext(AST_TYPE.IfStatement, "consequent", 0),
      isOwner: false,
      expected: true,
    },
    {
      name: "plain block without owner or branch context -> false",
      type: SCOPE_TYPE.Block,
      blockContext: null,
      isOwner: false,
      expected: false,
    },
    {
      name: "module without owner -> false",
      type: SCOPE_TYPE.Module,
      blockContext: null,
      isOwner: false,
      expected: false,
    },
    {
      name: "global without owner -> false",
      type: SCOPE_TYPE.Global,
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
