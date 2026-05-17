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
  // dagre sizes empty clusters correctly on its own, so no placeholder is
  // needed regardless of how the subgraph is wired to the rest of the graph.
  test("always returns null (dagre needs no workaround)", () => {
    const result = dagreStrategy.emptySubgraphPlaceholder({
      subgraphId: "s_scope_42",
      indent: "    ",
    });
    expect(result).toEqual(null);
  });
});
