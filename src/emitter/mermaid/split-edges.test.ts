import { describe, expect, test } from "vitest";

import { splitEdges } from "./split-edges.js";
import { baseEdge } from "./testing/make-edge.js";

describe("splitEdges", () => {
  test("routes edges whose `from` is in importSourceIds to imports", () => {
    const importSources = new Set(["mod_a"]);
    const got = splitEdges(
      [
        { ...baseEdge(), from: "mod_a", to: "n_x" },
        { ...baseEdge(), from: "n_x", to: "n_y" },
      ],
      importSources,
    );
    expect(got.imports.map((e) => e.from)).toEqual(["mod_a"]);
    expect(got.body.map((e) => e.from)).toEqual(["n_x"]);
  });

  test("preserves the relative order within each bucket", () => {
    const importSources = new Set(["mod_a"]);
    const got = splitEdges(
      [
        { ...baseEdge(), from: "n_a", to: "n_b" },
        { ...baseEdge(), from: "mod_a", to: "n_a" },
        { ...baseEdge(), from: "n_c", to: "n_d" },
      ],
      importSources,
    );
    expect(got.body.map((e) => `${e.from}->${e.to}`)).toEqual([
      "n_a->n_b",
      "n_c->n_d",
    ]);
    expect(got.imports.map((e) => `${e.from}->${e.to}`)).toEqual([
      "mod_a->n_a",
    ]);
  });

  test("edges that merely target an import-source go to body, not imports", () => {
    // Behavior: only the `from` side classifies the edge.
    const importSources = new Set(["mod_a"]);
    const got = splitEdges(
      [{ ...baseEdge(), from: "n_x", to: "mod_a" }],
      importSources,
    );
    expect(got.imports).toEqual([]);
    expect(got.body).toHaveLength(1);
  });

  test("empty edges -> two empty buckets", () => {
    const got = splitEdges([], new Set());
    expect(got.body).toEqual([]);
    expect(got.imports).toEqual([]);
  });
});
