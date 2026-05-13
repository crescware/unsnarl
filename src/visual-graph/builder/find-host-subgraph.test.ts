import { describe, expect, test } from "vitest";

import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { DIRECTION } from "../direction.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import type { BuildState } from "./build-state.js";
import { findHostSubgraph } from "./find-host-subgraph.js";
import { baseRef } from "./testing/make-ref.js";
import { baseScope } from "./testing/make-scope.js";

function baseSubgraph(id: string): VisualSubgraph {
  return {
    type: VISUAL_ELEMENT_TYPE.Subgraph,
    id,
    kind: SUBGRAPH_KIND.Function,
    line: 1,
    endLine: null,
    direction: DIRECTION.RL,
    ownerNodeId: "n_owner",
    ownerName: "owner",
    elements: [],
  };
}

function emptyState(overrides: Partial<BuildState> = {}): BuildState {
  return {
    subgraphByScope: new Map(),
    functionSubgraphByFn: new Map(),
    returnSubgraphsByFn: new Map(),
    returnUseAdded: new Set(),
    ifTestAnchorByOffset: new Map(),
    switchDiscriminantAnchorByOffset: new Map(),
    whileTestAnchorByOffset: new Map(),
    doWhileTestAnchorByOffset: new Map(),
    forTestAnchorByOffset: new Map(),
    pendingLoopTestAnchors: [],
    expressionStatementByOffset: new Map(),
    emittedEdges: new Set(),
    edges: [],
    ...overrides,
  };
}

const root = { ...baseScope(), id: asScopeId("root") };
const inner = {
  ...baseScope(),
  id: asScopeId("inner"),
  upper: asScopeId("root"),
};
const leaf = {
  ...baseScope(),
  id: asScopeId("leaf"),
  upper: asScopeId("inner"),
};
const scopeMap = new Map<string, SerializedScope>(
  [root, inner, leaf].map((v) => [v.id, v]),
);

describe("findHostSubgraph", () => {
  test("returns the subgraph mapped to the ref's own scope when present", () => {
    const sg = baseSubgraph("s_leaf");
    const state = emptyState({ subgraphByScope: new Map([["leaf", sg]]) });
    const ref = { ...baseRef(), from: asScopeId("leaf") };
    expect(findHostSubgraph(ref, "fnVar", scopeMap, state)).toEqual(sg);
  });

  test("walks up via .upper to find the closest enclosing subgraph", () => {
    const sg = baseSubgraph("s_root");
    const state = emptyState({ subgraphByScope: new Map([["root", sg]]) });
    const ref = { ...baseRef(), from: asScopeId("leaf") };
    expect(findHostSubgraph(ref, "fnVar", scopeMap, state)).toEqual(sg);
  });

  test("falls back to the function subgraph for the enclosing fn variable id", () => {
    const fnSg = baseSubgraph("s_fn");
    const state = emptyState({
      functionSubgraphByFn: new Map([["fnVar", fnSg]]),
    });
    const ref = { ...baseRef(), from: asScopeId("leaf") };
    expect(findHostSubgraph(ref, "fnVar", scopeMap, state)).toEqual(fnSg);
  });

  test("returns null when neither chain nor fn fallback yields a subgraph", () => {
    const ref = { ...baseRef(), from: asScopeId("leaf") };
    expect(findHostSubgraph(ref, "nope", scopeMap, emptyState())).toEqual(null);
  });

  test("returns null when ref.from is not in the scope map and the fn fallback is also missing", () => {
    const ref = { ...baseRef(), from: asScopeId("missing") };
    expect(findHostSubgraph(ref, "missingFn", scopeMap, emptyState())).toEqual(
      null,
    );
  });

  test("returns the scope-chain subgraph even when enclosingFnVarId is null", () => {
    const sg = baseSubgraph("s_root");
    const state = emptyState({ subgraphByScope: new Map([["root", sg]]) });
    const ref = { ...baseRef(), from: asScopeId("leaf") };
    expect(findHostSubgraph(ref, null, scopeMap, state)).toEqual(sg);
  });

  test("returns null when enclosingFnVarId is null and the scope chain yields no subgraph", () => {
    const ref = { ...baseRef(), from: asScopeId("leaf") };
    const fnSg = baseSubgraph("s_fn");
    const state = emptyState({
      // A function subgraph exists but it must be ignored when fnVarId is null.
      functionSubgraphByFn: new Map([["fnVar", fnSg]]),
    });
    expect(findHostSubgraph(ref, null, scopeMap, state)).toEqual(null);
  });
});
