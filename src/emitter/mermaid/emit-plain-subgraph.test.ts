import { describe, expect, test } from "vitest";

import { DIRECTION } from "../../visual-graph/direction.js";
import { SUBGRAPH_KIND } from "../../visual-graph/subgraph-kind.js";
import { emitPlainSubgraph } from "./emit-plain-subgraph.js";
import type { MermaidStrategy } from "./strategy/strategy.js";
import { baseNode } from "./testing/make-node.js";
import { baseRenderState } from "./testing/make-render-state.js";
import { baseStrategy } from "./testing/make-strategy.js";
import { baseSubgraph } from "./testing/make-subgraph.js";

describe("emitPlainSubgraph", () => {
  test("emits subgraph open, direction line, child nodes, and close", () => {
    const state = baseRenderState();
    const sg = {
      ...baseSubgraph(),
      id: "s_x",
      kind: SUBGRAPH_KIND.If,
      direction: DIRECTION.TB,
      elements: [
        { ...baseNode(), id: "n_a" },
        { ...baseNode(), id: "n_b" },
      ],
    };
    emitPlainSubgraph(state, sg, "  ", 1);
    expect(state.lines[0]).toEqual('  subgraph s_x["if L1"]');
    expect(state.lines[1]).toEqual("    direction TB");
    expect(state.lines.at(-1)).toEqual("  end");
    // The two children are emitted between direction and end.
    expect(state.lines).toHaveLength(5);
  });

  test("emits child nodes before nested subgraphs", () => {
    const state = baseRenderState();
    const sg = {
      ...baseSubgraph(),
      id: "outer",
      kind: SUBGRAPH_KIND.If,
      elements: [
        { ...baseSubgraph(), id: "inner", kind: SUBGRAPH_KIND.Else },
        { ...baseNode(), id: "n_a" },
      ],
    };
    emitPlainSubgraph(state, sg, "  ", 1);
    const nodeIdx = state.lines.findIndex((v) => v.includes("n_a"));
    const innerIdx = state.lines.findIndex((v) => v.includes("subgraph inner"));
    expect(nodeIdx < innerIdx).toEqual(true);
  });

  test("skips child nodes whose id is in wrappedOwnerIds", () => {
    const wrapped = new Set(["n_owner"]);
    const state = { ...baseRenderState(), wrappedOwnerIds: wrapped };
    const sg = {
      ...baseSubgraph(),
      kind: SUBGRAPH_KIND.If,
      elements: [
        { ...baseNode(), id: "n_owner" },
        { ...baseNode(), id: "n_keep" },
      ],
    };
    emitPlainSubgraph(state, sg, "  ", 1);
    expect(state.lines.some((v) => v.includes("n_owner"))).toEqual(false);
    expect(state.lines.some((v) => v.includes("n_keep"))).toEqual(true);
  });

  test("invokes emptySubgraphPlaceholder when there are no emitted children", () => {
    const strategy = {
      ...baseStrategy(),
      emptySubgraphPlaceholder: ({ subgraphId, indent }) => ({
        line: `${indent}__placeholder_${subgraphId}`,
        placeholderId: `ph_${subgraphId}`,
      }),
    } satisfies MermaidStrategy;
    const state = { ...baseRenderState(), strategy };
    const sg = {
      ...baseSubgraph(),
      id: "empty",
      kind: SUBGRAPH_KIND.If,
      elements: [],
    };
    emitPlainSubgraph(state, sg, "  ", 1);
    expect(state.lines.some((v) => v.includes("__placeholder_empty"))).toEqual(
      true,
    );
    expect(state.placeholderIds).toEqual(["ph_empty"]);
  });

  test("does NOT invoke the placeholder when at least one child was emitted", () => {
    let called = false;
    const strategy = {
      ...baseStrategy(),
      emptySubgraphPlaceholder: () => {
        called = true;
        return null;
      },
    } satisfies MermaidStrategy;
    const state = { ...baseRenderState(), strategy };
    const sg = {
      ...baseSubgraph(),
      kind: SUBGRAPH_KIND.If,
      elements: [{ ...baseNode(), id: "n_a" }],
    };
    emitPlainSubgraph(state, sg, "  ", 1);
    expect(called).toEqual(false);
  });

  test("placeholder returning null inserts no line and registers no id", () => {
    const strategy = {
      ...baseStrategy(),
      emptySubgraphPlaceholder: () => null,
    };
    const state = { ...baseRenderState(), strategy };
    const sg = {
      ...baseSubgraph(),
      id: "empty",
      kind: SUBGRAPH_KIND.If,
      elements: [],
    };
    const before = state.lines.length;
    emitPlainSubgraph(state, sg, "  ", 1);
    // open + direction + end = 3 lines, no placeholder
    expect(state.lines.length - before).toEqual(3);
    expect(state.placeholderIds).toEqual([]);
  });

  test("forwards referencedByEdge=true when subgraph id appears in edgeEndpointIds", () => {
    let observed: boolean | null = null;
    const strategy = {
      ...baseStrategy(),
      emptySubgraphPlaceholder: (ctx) => {
        observed = ctx.referencedByEdge;
        return null;
      },
    } satisfies MermaidStrategy;
    const state = {
      ...baseRenderState(),
      strategy,
      edgeEndpointIds: new Set(["empty"]),
    };
    emitPlainSubgraph(
      state,
      { ...baseSubgraph(), id: "empty", kind: SUBGRAPH_KIND.If },
      "  ",
      1,
    );
    expect(observed).toEqual(true);
  });

  test("records the subgraph id under its 1-based depth in nestClassMap", () => {
    const state = baseRenderState();
    emitPlainSubgraph(
      state,
      { ...baseSubgraph(), id: "s_at_depth1", kind: SUBGRAPH_KIND.If },
      "  ",
      1,
    );
    expect(state.nestClassMap.get(0)).toEqual(["s_at_depth1"]);
  });

  test("recurses into nested subgraphs with depth + 1", () => {
    const state = baseRenderState();
    emitPlainSubgraph(
      state,
      {
        ...baseSubgraph(),
        id: "outer",
        kind: SUBGRAPH_KIND.If,
        elements: [
          { ...baseSubgraph(), id: "inner", kind: SUBGRAPH_KIND.Else },
        ],
      },
      "  ",
      1,
    );
    expect(state.nestClassMap.get(0)).toEqual(["outer"]);
    expect(state.nestClassMap.get(1)).toEqual(["inner"]);
  });

  test("wraps to slot 0 when depth exceeds the palette length", () => {
    // baseRenderState carries darkTheme; pin against its palette length so
    // the test stays valid if the palette resizes.
    const state = baseRenderState();
    const paletteLength = state.theme.nestPalette.length;
    const overflowDepth = paletteLength + 1;
    emitPlainSubgraph(
      state,
      { ...baseSubgraph(), id: "s_overflow", kind: SUBGRAPH_KIND.If },
      "  ",
      overflowDepth,
    );
    expect(state.nestClassMap.get(0)).toEqual(["s_overflow"]);
  });
});
