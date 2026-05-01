import { describe, expect, test } from "vitest";

import type { BuildState } from "./build-state.js";
import { pushEdge } from "./push-edge.js";

function emptyState(): BuildState {
  return {
    subgraphByScope: new Map(),
    functionSubgraphByFn: new Map(),
    returnSubgraphsByFn: new Map(),
    returnUseAdded: new Set(),
    emittedEdges: new Set(),
    edges: [],
  };
}

describe("pushEdge", () => {
  test("appends a new edge and records the dedup key", () => {
    const state = emptyState();
    pushEdge(state, "a", "read", "b");
    expect(state.edges).toEqual([{ from: "a", to: "b", label: "read" }]);
    expect(state.emittedEdges.has("a -->|read| b")).toBe(true);
  });

  test("ignores a second call that exactly matches a prior (from, label, to)", () => {
    const state = emptyState();
    pushEdge(state, "a", "read", "b");
    pushEdge(state, "a", "read", "b");
    expect(state.edges).toHaveLength(1);
  });

  test.each<{ name: string; second: [string, string, string] }>([
    { name: "different label", second: ["a", "write", "b"] },
    { name: "different from", second: ["x", "read", "b"] },
    { name: "different to", second: ["a", "read", "z"] },
  ])("$name keeps both edges", ({ second }) => {
    const state = emptyState();
    pushEdge(state, "a", "read", "b");
    pushEdge(state, ...second);
    expect(state.edges).toHaveLength(2);
  });

  test("preserves insertion order across distinct edges", () => {
    const state = emptyState();
    pushEdge(state, "a", "read", "b");
    pushEdge(state, "a", "write", "b");
    pushEdge(state, "c", "read", "d");
    expect(state.edges.map((e) => `${e.from}-${e.label}-${e.to}`)).toEqual([
      "a-read-b",
      "a-write-b",
      "c-read-d",
    ]);
  });
});
