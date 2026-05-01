import { describe, expect, test } from "vitest";

import { DIRECTION } from "../../direction.js";
import { SUBGRAPH_KIND } from "../../subgraph-kind.js";
import { emitPlainSubgraph } from "./emit-plain-subgraph.js";
import { makeNode } from "./testing/make-node.js";
import { makeRenderState } from "./testing/make-render-state.js";
import { makeStrategy } from "./testing/make-strategy.js";
import { makeSubgraph } from "./testing/make-subgraph.js";

describe("emitPlainSubgraph", () => {
  test("emits subgraph open, direction line, child nodes, and close", () => {
    const state = makeRenderState();
    const sg = makeSubgraph({
      id: "s_x",
      kind: SUBGRAPH_KIND.If,
      direction: DIRECTION.TB,
      elements: [makeNode({ id: "n_a" }), makeNode({ id: "n_b" })],
    });
    emitPlainSubgraph(state, sg, "  ");
    expect(state.lines[0]).toBe('  subgraph s_x["if L1"]');
    expect(state.lines[1]).toBe("    direction TB");
    expect(state.lines.at(-1)).toBe("  end");
    // The two children are emitted between direction and end.
    expect(state.lines).toHaveLength(5);
  });

  test("emits child nodes before nested subgraphs", () => {
    const state = makeRenderState();
    const sg = makeSubgraph({
      id: "outer",
      kind: SUBGRAPH_KIND.If,
      elements: [
        makeSubgraph({ id: "inner", kind: SUBGRAPH_KIND.Else }),
        makeNode({ id: "n_a" }),
      ],
    });
    emitPlainSubgraph(state, sg, "  ");
    const nodeIdx = state.lines.findIndex((l) => l.includes("n_a"));
    const innerIdx = state.lines.findIndex((l) => l.includes("subgraph inner"));
    expect(nodeIdx).toBeLessThan(innerIdx);
  });

  test("skips child nodes whose id is in wrappedOwnerIds", () => {
    const wrapped = new Set(["n_owner"]);
    const state = makeRenderState({ wrappedOwnerIds: wrapped });
    const sg = makeSubgraph({
      kind: SUBGRAPH_KIND.If,
      elements: [makeNode({ id: "n_owner" }), makeNode({ id: "n_keep" })],
    });
    emitPlainSubgraph(state, sg, "  ");
    expect(state.lines.some((l) => l.includes("n_owner"))).toBe(false);
    expect(state.lines.some((l) => l.includes("n_keep"))).toBe(true);
  });

  test("invokes emptySubgraphPlaceholder when there are no emitted children", () => {
    const strategy = makeStrategy({
      emptySubgraphPlaceholder: ({ subgraphId, indent }) => ({
        line: `${indent}__placeholder_${subgraphId}`,
        placeholderId: `ph_${subgraphId}`,
      }),
    });
    const state = makeRenderState({ strategy });
    const sg = makeSubgraph({
      id: "empty",
      kind: SUBGRAPH_KIND.If,
      elements: [],
    });
    emitPlainSubgraph(state, sg, "  ");
    expect(state.lines.some((l) => l.includes("__placeholder_empty"))).toBe(
      true,
    );
    expect(state.placeholderIds).toEqual(["ph_empty"]);
  });

  test("does NOT invoke the placeholder when at least one child was emitted", () => {
    let called = false;
    const strategy = makeStrategy({
      emptySubgraphPlaceholder: () => {
        called = true;
        return null;
      },
    });
    const state = makeRenderState({ strategy });
    const sg = makeSubgraph({
      kind: SUBGRAPH_KIND.If,
      elements: [makeNode({ id: "n_a" })],
    });
    emitPlainSubgraph(state, sg, "  ");
    expect(called).toBe(false);
  });

  test("placeholder returning null inserts no line and registers no id", () => {
    const strategy = makeStrategy({ emptySubgraphPlaceholder: () => null });
    const state = makeRenderState({ strategy });
    const sg = makeSubgraph({
      id: "empty",
      kind: SUBGRAPH_KIND.If,
      elements: [],
    });
    const before = state.lines.length;
    emitPlainSubgraph(state, sg, "  ");
    // open + direction + end = 3 lines, no placeholder
    expect(state.lines.length - before).toBe(3);
    expect(state.placeholderIds).toEqual([]);
  });

  test("forwards referencedByEdge=true when subgraph id appears in edgeEndpointIds", () => {
    let observed: boolean | null = null;
    const strategy = makeStrategy({
      emptySubgraphPlaceholder: (ctx) => {
        observed = ctx.referencedByEdge;
        return null;
      },
    });
    const state = makeRenderState({
      strategy,
      edgeEndpointIds: new Set(["empty"]),
    });
    emitPlainSubgraph(
      state,
      makeSubgraph({ id: "empty", kind: SUBGRAPH_KIND.If }),
      "  ",
    );
    expect(observed).toBe(true);
  });
});
