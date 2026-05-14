import { describe, expect, test } from "vitest";

import { SCOPE_TYPE, type ScopeType } from "../../analyzer/scope-type.js";
import type { BlockContext } from "../../ir/scope/block-context.js";
import { asScopeId } from "../../ir/serialized/scope-id.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { asFilledString } from "../../util/filled-string.js";
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
      name: asFilledString("function scope -> true"),
      type: SCOPE_TYPE.Function,
      blockContext: null,
      expected: true,
    },
    {
      name: asFilledString("control kind (for) -> true"),
      type: SCOPE_TYPE.For,
      blockContext: null,
      expected: true,
    },
    {
      name: asFilledString("branch block (if consequent) -> true"),
      type: SCOPE_TYPE.Block,
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 0,
      },
      expected: true,
    },
    {
      name: asFilledString(
        "bare block -> true (renders as the generic 'block' subgraph)",
      ),
      type: SCOPE_TYPE.Block,
      blockContext: null,
      expected: true,
    },
    {
      name: asFilledString("module -> false"),
      type: SCOPE_TYPE.Module,
      blockContext: null,
      expected: false,
    },
    {
      name: asFilledString("global -> false"),
      type: SCOPE_TYPE.Global,
      blockContext: null,
      expected: false,
    },
  ])("$name", ({ type, blockContext, expected }) => {
    const scope = { ...baseScope(), id: asScopeId("s"), type, blockContext };
    expect(shouldSubgraph(scope)).toEqual(expected);
  });
});
