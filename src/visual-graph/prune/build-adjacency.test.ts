import { describe, expect, test } from "vitest";

import type { VisualEdge } from "../model.js";
import { buildAdjacency } from "./build-adjacency.js";

const edge = (from: string, to: string, label = "read"): VisualEdge => ({
  from,
  to,
  label,
});

describe("buildAdjacency", () => {
  test("empty edges produce empty maps", () => {
    const { outEdges, inEdges } = buildAdjacency([]);
    expect(outEdges.size).toBe(0);
    expect(inEdges.size).toBe(0);
  });

  test("each edge contributes to both outEdges[from] and inEdges[to]", () => {
    const { outEdges, inEdges } = buildAdjacency([edge("a", "b")]);
    expect(outEdges.get("a")).toEqual(["b"]);
    expect(inEdges.get("b")).toEqual(["a"]);
  });

  test("multiple edges from the same source append in source order", () => {
    const { outEdges } = buildAdjacency([edge("a", "b"), edge("a", "c")]);
    expect(outEdges.get("a")).toEqual(["b", "c"]);
  });

  test("self-loops record both directions on the same node", () => {
    const { outEdges, inEdges } = buildAdjacency([edge("a", "a")]);
    expect(outEdges.get("a")).toEqual(["a"]);
    expect(inEdges.get("a")).toEqual(["a"]);
  });
});
