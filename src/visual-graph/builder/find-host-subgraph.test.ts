import { describe, expect, test } from "vitest";

import type { SerializedScope } from "../../ir/model.js";
import { DIRECTION } from "../direction.js";
import type { VisualSubgraph } from "../model.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { BuildState } from "./build-state.js";
import { findHostSubgraph } from "./find-host-subgraph.js";
import { makeRef } from "./testing/make-ref.js";
import { makeScope } from "./testing/make-scope.js";

function makeSubgraph(id: string): VisualSubgraph {
  return {
    type: VISUAL_ELEMENT_TYPE.Subgraph,
    id,
    kind: SUBGRAPH_KIND.Function,
    line: 1,
    direction: DIRECTION.RL,
    elements: [],
  };
}

function emptyState(overrides: Partial<BuildState> = {}): BuildState {
  return {
    subgraphByScope: new Map(),
    functionSubgraphByFn: new Map(),
    returnSubgraphsByFn: new Map(),
    returnUseAdded: new Set(),
    emittedEdges: new Set(),
    edges: [],
    ...overrides,
  };
}

const root = makeScope({ id: "root" });
const inner = makeScope({ id: "inner", upper: "root" });
const leaf = makeScope({ id: "leaf", upper: "inner" });
const scopeMap = new Map<string, SerializedScope>(
  [root, inner, leaf].map((s) => [s.id, s]),
);

describe("findHostSubgraph", () => {
  test("returns the subgraph mapped to the ref's own scope when present", () => {
    const sg = makeSubgraph("s_leaf");
    const state = emptyState({ subgraphByScope: new Map([["leaf", sg]]) });
    const ref = makeRef({ from: "leaf" });
    expect(findHostSubgraph(ref, "fnVar", scopeMap, state)).toBe(sg);
  });

  test("walks up via .upper to find the closest enclosing subgraph", () => {
    const sg = makeSubgraph("s_root");
    const state = emptyState({ subgraphByScope: new Map([["root", sg]]) });
    const ref = makeRef({ from: "leaf" });
    expect(findHostSubgraph(ref, "fnVar", scopeMap, state)).toBe(sg);
  });

  test("falls back to the function subgraph for the enclosing fn variable id", () => {
    const fnSg = makeSubgraph("s_fn");
    const state = emptyState({
      functionSubgraphByFn: new Map([["fnVar", fnSg]]),
    });
    const ref = makeRef({ from: "leaf" });
    expect(findHostSubgraph(ref, "fnVar", scopeMap, state)).toBe(fnSg);
  });

  test("returns null when neither chain nor fn fallback yields a subgraph", () => {
    const ref = makeRef({ from: "leaf" });
    expect(findHostSubgraph(ref, "nope", scopeMap, emptyState())).toBeNull();
  });

  test("returns null when ref.from is not in the scope map and the fn fallback is also missing", () => {
    const ref = makeRef({ from: "missing" });
    expect(
      findHostSubgraph(ref, "missingFn", scopeMap, emptyState()),
    ).toBeNull();
  });
});
