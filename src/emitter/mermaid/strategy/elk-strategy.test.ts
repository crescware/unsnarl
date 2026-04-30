import { describe, expect, test } from "vitest";

import { elkStrategy } from "./elk-strategy.js";

describe("elkStrategy.emptySubgraphPlaceholder", () => {
  test("returns null when the subgraph is not referenced by any edge", () => {
    const result = elkStrategy.emptySubgraphPlaceholder({
      subgraphId: "s_scope_42",
      indent: "    ",
      referencedByEdge: false,
    });
    expect(result).toBeNull();
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
    expect(result?.line).toBe('\t\telk_empty_x[" "]');
  });

  test("placeholderId is derived solely from the subgraph id", () => {
    const result = elkStrategy.emptySubgraphPlaceholder({
      subgraphId: "cont_if_scope_0_99",
      indent: "  ",
      referencedByEdge: true,
    });
    expect(result?.placeholderId).toBe("elk_empty_cont_if_scope_0_99");
  });
});
