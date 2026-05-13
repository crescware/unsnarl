import { describe, expect, test } from "vitest";

import { BOUNDARY_EDGE_DIRECTION } from "../../visual-graph/prune/boundary-edge-direction.js";
import type { VisualBoundaryEdge } from "../../visual-graph/visual-boundary-edge.js";
import type { VisualEdge } from "../../visual-graph/visual-edge.js";
import { collectHighlightEdgeIndices } from "./collect-highlight-edge-indices.js";

const e = (from: string, to: string): VisualEdge => ({ from, to, label: "" });

describe("collectHighlightEdgeIndices", () => {
  test("returns [] when no ids are highlighted", () => {
    const r = collectHighlightEdgeIndices(
      [e("a", "b"), e("b", "c")],
      [],
      [],
      new Set<string>(),
    );
    expect(r).toEqual([]);
  });

  test("collects body edges whose endpoint matches", () => {
    const r = collectHighlightEdgeIndices(
      [e("a", "b"), e("b", "c"), e("a", "c")],
      [],
      [],
      new Set(["b"]),
    );
    expect(r).toEqual([0, 1]);
  });

  test("collects across body and import edges with global indexing", () => {
    const r = collectHighlightEdgeIndices(
      [e("a", "b")],
      [e("mod", "a"), e("b", "sink")],
      [],
      new Set(["a"]),
    );
    expect(r).toEqual([0, 1]);
  });

  test("collects boundary edges by 'inside' id", () => {
    const boundary: readonly VisualBoundaryEdge[] = [
      { inside: "a", direction: BOUNDARY_EDGE_DIRECTION.Out },
      {
        inside: "b",
        direction: BOUNDARY_EDGE_DIRECTION.In,
        label: "read",
      },
    ];
    const r = collectHighlightEdgeIndices([], [], boundary, new Set(["b"]));
    expect(r).toEqual([1]);
  });

  test("indexing across all three buckets is contiguous", () => {
    const r = collectHighlightEdgeIndices(
      [e("a", "b")],
      [e("a", "c")],
      [{ inside: "a", direction: BOUNDARY_EDGE_DIRECTION.Out }],
      new Set(["a"]),
    );
    expect(r).toEqual([0, 1, 2]);
  });
});
