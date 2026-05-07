import { describe, expect, test } from "vitest";

import { SCOPE_TYPE, type ScopeType } from "../../analyzer/scope-type.js";
import type { BlockContext } from "../../ir/scope/block-context.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { shouldSubgraph } from "./should-subgraph.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";

describe("shouldSubgraph", () => {
  test.each<{
    name: string;
    type: ScopeType;
    blockContext: BlockContext | null;
    expected: boolean;
  }>([
    {
      name: "function scope -> true",
      type: SCOPE_TYPE.Function,
      blockContext: null,
      expected: true,
    },
    {
      name: "control kind (for) -> true",
      type: SCOPE_TYPE.For,
      blockContext: null,
      expected: true,
    },
    {
      name: "branch block (if consequent) -> true",
      type: SCOPE_TYPE.Block,
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "consequent",
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: "plain block without branch context -> false",
      type: SCOPE_TYPE.Block,
      blockContext: null,
      expected: false,
    },
    {
      name: "module -> false",
      type: SCOPE_TYPE.Module,
      blockContext: null,
      expected: false,
    },
    {
      name: "global -> false",
      type: SCOPE_TYPE.Global,
      blockContext: null,
      expected: false,
    },
  ])("$name", ({ type, blockContext, expected }) => {
    const scope = { ...baseScope(), id: "s", type, blockContext };
    expect(shouldSubgraph(scope)).toBe(expected);
  });
});
