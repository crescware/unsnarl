import { describe, expect, test } from "vitest";

import { dagreStrategy } from "./dagre-strategy.js";

describe("dagreStrategy", () => {
  test("emits no preamble lines (dagre is the default renderer)", () => {
    expect(dagreStrategy.preambleLines).toEqual([]);
  });

  test("trailerLines returns an empty list regardless of placeholder ids", () => {
    expect(dagreStrategy.trailerLines([])).toEqual([]);
    expect(dagreStrategy.trailerLines(["elk_empty_a", "elk_empty_b"])).toEqual(
      [],
    );
  });
});

describe("dagreStrategy.emptySubgraphPlaceholder", () => {
  test("returns null when the subgraph is not referenced by any edge", () => {
    const result = dagreStrategy.emptySubgraphPlaceholder({
      subgraphId: "s_scope_42",
      indent: "    ",
      referencedByEdge: false,
    });
    expect(result).toBeNull();
  });

  test("returns null even when the subgraph is referenced by an edge (dagre needs no workaround)", () => {
    const result = dagreStrategy.emptySubgraphPlaceholder({
      subgraphId: "s_scope_42",
      indent: "    ",
      referencedByEdge: true,
    });
    expect(result).toBeNull();
  });
});
