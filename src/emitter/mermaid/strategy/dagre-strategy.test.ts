import { describe, expect, test } from "vitest";

import { darkTheme } from "../theme/dark-theme.js";
import { dagreStrategy } from "./dagre-strategy.js";

describe("dagreStrategy", () => {
  test("emits no preamble lines (dagre is the default renderer)", () => {
    expect(dagreStrategy.preambleLines).toEqual([]);
  });

  test("trailerLines returns an empty list regardless of placeholder ids", () => {
    expect(dagreStrategy.trailerLines([], darkTheme)).toEqual([]);
    expect(
      dagreStrategy.trailerLines(["elk_empty_a", "elk_empty_b"], darkTheme),
    ).toEqual([]);
  });
});

describe("dagreStrategy.emptySubgraphPlaceholder", () => {
  test("returns null when the subgraph is not referenced by any edge", () => {
    const result = dagreStrategy.emptySubgraphPlaceholder({
      subgraphId: "s_scope_42",
      indent: "    ",
      referencedByEdge: false,
    });
    expect(result).toEqual(null);
  });

  test("returns null even when the subgraph is referenced by an edge (dagre needs no workaround)", () => {
    const result = dagreStrategy.emptySubgraphPlaceholder({
      subgraphId: "s_scope_42",
      indent: "    ",
      referencedByEdge: true,
    });
    expect(result).toEqual(null);
  });
});
