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
      "  classDef elkEmptyPlaceholder fill:transparent,stroke:transparent,color:transparent;",
    );
    expect(out).toContain("  class elk_empty_a elkEmptyPlaceholder;");
    expect(out).toContain("  class elk_empty_b elkEmptyPlaceholder;");
  });

  test("routes through the supplied theme so the placeholder picks up theme literals", () => {
    const out = elkStrategy.trailerLines(["elk_empty_x"], lightTheme);
    expect(out[0]).toEqual(
      `  classDef elkEmptyPlaceholder fill:${lightTheme.elkEmptyPlaceholder.fill},stroke:${lightTheme.elkEmptyPlaceholder.stroke},color:${lightTheme.elkEmptyPlaceholder.color};`,
    );
  });
});

describe("elkStrategy.emptySubgraphPlaceholder", () => {
  test("returns null when the subgraph is not referenced by any edge", () => {
    const result = elkStrategy.emptySubgraphPlaceholder({
      subgraphId: "s_scope_42",
      indent: "    ",
      referencedByEdge: false,
    });
    expect(result).toEqual(null);
  });

  test("returns a placeholder line + id when the subgraph is an edge endpoint", () => {
    const result = elkStrategy.emptySubgraphPlaceholder({
      subgraphId: "s_scope_42",
      indent: "    ",
      referencedByEdge: true,
    });
    expect(result).toEqual({
      line: '    elk_empty_s_scope_42[" "]',
      placeholderId: "elk_empty_s_scope_42",
    });
  });

  test("propagates the indent prefix verbatim into the emitted line", () => {
    const result = elkStrategy.emptySubgraphPlaceholder({
      subgraphId: "x",
      indent: "\t\t",
      referencedByEdge: true,
    });
    expect(result?.line).toEqual('\t\telk_empty_x[" "]');
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
