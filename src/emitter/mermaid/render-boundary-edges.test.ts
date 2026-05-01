import { describe, expect, test } from "vitest";

import { renderBoundaryEdges } from "./render-boundary-edges.js";
import { makeGraph } from "./testing/make-graph.js";

describe("renderBoundaryEdges", () => {
  test("does nothing when boundaryEdges is undefined or empty", () => {
    const lines: string[] = [];
    const stubIds: string[] = [];
    renderBoundaryEdges(makeGraph(), lines, stubIds);
    expect(lines).toEqual([]);
    expect(stubIds).toEqual([]);

    renderBoundaryEdges(makeGraph({ boundaryEdges: [] }), lines, stubIds);
    expect(lines).toEqual([]);
    expect(stubIds).toEqual([]);
  });

  test("emits an unlabeled dashed arrow for direction='out' (label is unknowable)", () => {
    const lines: string[] = [];
    const stubIds: string[] = [];
    renderBoundaryEdges(
      makeGraph({
        boundaryEdges: [{ inside: "n_x", direction: "out" }],
      }),
      lines,
      stubIds,
    );
    expect(stubIds).toEqual(["boundary_stub_1"]);
    expect(lines).toContain("  boundary_stub_1((...))");
    expect(lines).toContain("  n_x -.-> boundary_stub_1");
  });

  test("emits a labeled dashed arrow for direction='in'", () => {
    const lines: string[] = [];
    const stubIds: string[] = [];
    renderBoundaryEdges(
      makeGraph({
        boundaryEdges: [{ inside: "n_x", direction: "in", label: "read" }],
      }),
      lines,
      stubIds,
    );
    expect(lines).toContain("  boundary_stub_1((...))");
    expect(lines).toContain("  boundary_stub_1 -.->|read| n_x");
  });

  test("assigns sequential stub ids and appends them to stubIds", () => {
    const lines: string[] = [];
    const stubIds: string[] = [];
    renderBoundaryEdges(
      makeGraph({
        boundaryEdges: [
          { inside: "a", direction: "out" },
          { inside: "b", direction: "in", label: "write" },
          { inside: "c", direction: "out" },
        ],
      }),
      lines,
      stubIds,
    );
    expect(stubIds).toEqual([
      "boundary_stub_1",
      "boundary_stub_2",
      "boundary_stub_3",
    ]);
  });
});
