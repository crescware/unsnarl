import { describe, expect, test } from "vitest";

import { collectWrappedOwnerIds } from "./collect-wrapped-owner-ids.js";
import { makeNode } from "./testing/make-node.js";
import { makeSubgraph } from "./testing/make-subgraph.js";

describe("collectWrappedOwnerIds", () => {
  test("captures ownerNodeId of every function subgraph", () => {
    const out = new Set<string>();
    collectWrappedOwnerIds(
      [
        makeSubgraph({ kind: "function", ownerNodeId: "n_a" }),
        makeSubgraph({ kind: "function", ownerNodeId: "n_b" }),
      ],
      out,
    );
    expect([...out].sort()).toEqual(["n_a", "n_b"]);
  });

  test("ignores function subgraphs without ownerNodeId", () => {
    const out = new Set<string>();
    collectWrappedOwnerIds([makeSubgraph({ kind: "function" })], out);
    expect(out.size).toBe(0);
  });

  test("ignores non-function subgraphs even with ownerNodeId set", () => {
    const out = new Set<string>();
    collectWrappedOwnerIds(
      [makeSubgraph({ kind: "if", ownerNodeId: "n_x" })],
      out,
    );
    expect(out.size).toBe(0);
  });

  test("recurses into nested subgraphs", () => {
    const out = new Set<string>();
    collectWrappedOwnerIds(
      [
        makeSubgraph({
          kind: "function",
          ownerNodeId: "n_outer",
          elements: [
            makeSubgraph({ kind: "function", ownerNodeId: "n_inner" }),
          ],
        }),
      ],
      out,
    );
    expect([...out].sort()).toEqual(["n_inner", "n_outer"]);
  });

  test("plain top-level nodes are skipped without traversal error", () => {
    const out = new Set<string>();
    collectWrappedOwnerIds(
      [
        makeNode({ id: "n_a" }),
        makeSubgraph({ kind: "function", ownerNodeId: "n_b" }),
      ],
      out,
    );
    expect([...out]).toEqual(["n_b"]);
  });
});
