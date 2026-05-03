import { describe, expect, test } from "vitest";

import { PREDICATE_CONTAINER_TYPE } from "../../analyzer/predicate-container-type.js";
import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { SerializedScope } from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import type { BuildState } from "./build-state.js";
import { predicateTargetId } from "./predicate-target-id.js";
import { baseRef } from "./testing/make-ref.js";
import { baseScope } from "./testing/make-scope.js";
import { predicateContainer } from "./testing/predicate-container.js";
import { span } from "./testing/span.js";

function makeState(): BuildState {
  return {
    subgraphByScope: new Map(),
    functionSubgraphByFn: new Map(),
    returnSubgraphsByFn: new Map(),
    returnUseAdded: new Set(),
    ifTestAnchorByOffset: new Map(),
    expressionStatementByOffset: new Map(),
    emittedEdges: new Set(),
    edges: [],
  };
}

function withSwitchAt(offset: number): {
  scopeMap: Map<string, SerializedScope>;
  refFrom: string;
} {
  const switchScope = {
    ...baseScope(),
    id: "switch1",
    type: SCOPE_TYPE.Switch,
    upper: "outer",
    block: {
      type: AST_TYPE.SwitchStatement,
      span: span(offset),
      endSpan: span(offset + 50),
    },
  };
  const inner = { ...baseScope(), id: "inner", upper: "switch1" };
  const outer = { ...baseScope(), id: "outer" };
  const scopes = [outer, switchScope, inner] satisfies SerializedScope[];
  return {
    scopeMap: new Map(scopes.map((s) => [s.id, s])),
    refFrom: "inner",
  };
}

// Switch routing intentionally diverges from IfStatement: switch reads
// target the entire Switch subgraph because there is no chain-collapse
// pathology, and per-case anchoring would add noise without separating
// any conflated relationships. Only the if path uses the test anchor.
describe("predicateTargetId", () => {
  test("no predicateContainer -> null", () => {
    const ref = { ...baseRef(), predicateContainer: null };
    expect(predicateTargetId(ref, new Map(), makeState())).toBeNull();
  });

  test("SwitchStatement matches an enclosing switch scope by offset", () => {
    const offset = 100;
    const { scopeMap, refFrom } = withSwitchAt(offset);
    const ref = {
      ...baseRef(),
      from: refFrom,
      predicateContainer: predicateContainer(
        PREDICATE_CONTAINER_TYPE.SwitchStatement,
        offset,
      ),
    };
    expect(predicateTargetId(ref, scopeMap, makeState())).toBe("s_switch1");
  });

  test("SwitchStatement with no enclosing switch at that offset -> null", () => {
    const { scopeMap, refFrom } = withSwitchAt(100);
    const ref = {
      ...baseRef(),
      from: refFrom,
      predicateContainer: predicateContainer(
        PREDICATE_CONTAINER_TYPE.SwitchStatement,
        999,
      ),
    };
    expect(predicateTargetId(ref, scopeMap, makeState())).toBeNull();
  });

  test("IfStatement with no anchor registered at that offset -> null", () => {
    const ref = {
      ...baseRef(),
      from: "outer",
      predicateContainer: predicateContainer(
        PREDICATE_CONTAINER_TYPE.IfStatement,
        50,
      ),
    };
    expect(predicateTargetId(ref, new Map(), makeState())).toBeNull();
  });
});
