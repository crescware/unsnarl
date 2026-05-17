import { describe, expect, test } from "vitest";

import { darkTheme } from "../theme/dark-theme.js";
import { lightTheme } from "../theme/light-theme.js";
import { elkStrategy } from "./elk-strategy.js";

describe("elkStrategy.trailerLines", () => {
  test("returns an empty list when there are no placeholder ids", () => {
    expect(elkStrategy.trailerLines([], darkTheme)).toEqual([]);
  });

  test("emits the elkEmptyPlaceholder classDef from the dark theme and one class line per id", () => {
    const out = elkStrategy.trailerLines(
      ["elk_empty_a", "elk_empty_b"],
      darkTheme,
    );
    expect(out[0]).toEqual(
      "  classDef elkEmptyPlaceholder fill:transparent,stroke:transparent;",
    );
    expect(out).toContain("  class elk_empty_a elkEmptyPlaceholder;");
    expect(out).toContain("  class elk_empty_b elkEmptyPlaceholder;");
  });

  test("routes through the supplied theme so the placeholder picks up theme literals", () => {
    const out = elkStrategy.trailerLines(["elk_empty_x"], lightTheme);
    expect(out[0]).toEqual(
      `  classDef elkEmptyPlaceholder fill:${lightTheme.elkEmptyPlaceholder.fill},stroke:${lightTheme.elkEmptyPlaceholder.stroke};`,
    );
  });

  // The placeholder is a layout marker, not a node. The classDef must
  // not pin a color literal because doing so would either hide the
  // "No nodes" label (color:transparent) or force a hard-coded color
  // that fights the active Mermaid theme. Mermaid's default text
  // color adapts to the page; we want that.
  test("does not emit a color segment in the elkEmptyPlaceholder classDef", () => {
    const out = elkStrategy.trailerLines(["elk_empty_x"], darkTheme);
    expect(out[0]?.includes("color:")).toEqual(false);
  });
});

describe("elkStrategy.emptySubgraphPlaceholder", () => {
  test("returns a placeholder line + id even when the subgraph is not edge-referenced", () => {
    const result = elkStrategy.emptySubgraphPlaceholder({
      subgraphId: "s_scope_42",
      indent: "    ",
      referencedByEdge: false,
    });
    expect(result).toEqual({
      line: '    elk_empty_s_scope_42["No nodes"]',
      placeholderId: "elk_empty_s_scope_42",
    });
  });

  test("returns a placeholder line + id when the subgraph is an edge endpoint", () => {
    const result = elkStrategy.emptySubgraphPlaceholder({
      subgraphId: "s_scope_42",
      indent: "    ",
      referencedByEdge: true,
    });
    expect(result).toEqual({
      line: '    elk_empty_s_scope_42["No nodes"]',
      placeholderId: "elk_empty_s_scope_42",
    });
  });

  test("propagates the indent prefix verbatim into the emitted line", () => {
    const result = elkStrategy.emptySubgraphPlaceholder({
      subgraphId: "x",
      indent: "\t\t",
      referencedByEdge: true,
    });
    expect(result?.line).toEqual('\t\telk_empty_x["No nodes"]');
  });

  test("placeholderId is derived solely from the subgraph id", () => {
    const result = elkStrategy.emptySubgraphPlaceholder({
      subgraphId: "cont_if_scope_0_99",
      indent: "  ",
      referencedByEdge: true,
    });
    expect(result?.placeholderId).toEqual("elk_empty_cont_if_scope_0_99");
  });
});
