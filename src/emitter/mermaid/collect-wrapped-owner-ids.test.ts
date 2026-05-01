import { describe, expect, test } from "vitest";

import { SUBGRAPH_KIND } from "../../visual-graph/subgraph-kind.js";
import { collectWrappedOwnerIds } from "./collect-wrapped-owner-ids.js";
import { baseNode } from "./testing/make-node.js";
import { baseSubgraph } from "./testing/make-subgraph.js";

describe("collectWrappedOwnerIds", () => {
  test("captures ownerNodeId of every function subgraph", () => {
    const out = new Set<string>();
    collectWrappedOwnerIds(
      [
        { ...baseSubgraph(), kind: SUBGRAPH_KIND.Function, ownerNodeId: "n_a" },
        { ...baseSubgraph(), kind: SUBGRAPH_KIND.Function, ownerNodeId: "n_b" },
      ],
      out,
    );
    expect([...out].sort()).toEqual(["n_a", "n_b"]);
  });

  test("ignores function subgraphs without ownerNodeId", () => {
    const out = new Set<string>();
    collectWrappedOwnerIds(
      [{ ...baseSubgraph(), kind: SUBGRAPH_KIND.Function }],
      out,
    );
    expect(out.size).toBe(0);
  });

  test("ignores non-function subgraphs even with ownerNodeId set", () => {
    const out = new Set<string>();
    collectWrappedOwnerIds(
      [{ ...baseSubgraph(), kind: SUBGRAPH_KIND.If, ownerNodeId: "n_x" }],
      out,
    );
    expect(out.size).toBe(0);
  });

  test("recurses into nested subgraphs", () => {
    const out = new Set<string>();
    collectWrappedOwnerIds(
      [
        {
          ...baseSubgraph(),
          kind: SUBGRAPH_KIND.Function,
          ownerNodeId: "n_outer",
          elements: [
            {
              ...baseSubgraph(),
              kind: SUBGRAPH_KIND.Function,
              ownerNodeId: "n_inner",
            },
          ],
        },
      ],
      out,
    );
    expect([...out].sort()).toEqual(["n_inner", "n_outer"]);
  });

  test("plain top-level nodes are skipped without traversal error", () => {
    const out = new Set<string>();
    collectWrappedOwnerIds(
      [
        { ...baseNode(), id: "n_a" },
        { ...baseSubgraph(), kind: SUBGRAPH_KIND.Function, ownerNodeId: "n_b" },
      ],
      out,
    );
    expect([...out]).toEqual(["n_b"]);
  });
});
