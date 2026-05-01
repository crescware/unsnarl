import { describe, expect, test } from "vitest";

import { SUBGRAPH_KIND } from "../../visual-graph/subgraph-kind.js";
import { renderTopLevelSubgraphs } from "./render-top-level-subgraphs.js";
import { baseGraph } from "./testing/make-graph.js";
import { baseNode } from "./testing/make-node.js";
import { baseRenderState } from "./testing/make-render-state.js";
import { baseSubgraph } from "./testing/make-subgraph.js";

describe("renderTopLevelSubgraphs", () => {
  test("delegates to emitSubgraph for each top-level subgraph", () => {
    const state = baseRenderState();
    renderTopLevelSubgraphs(state, {
      ...baseGraph(),
      elements: [
        { ...baseSubgraph(), id: "s1", kind: SUBGRAPH_KIND.If },
        { ...baseSubgraph(), id: "s2", kind: SUBGRAPH_KIND.Else },
      ],
    });
    expect(state.lines.filter((l) => l.startsWith("  subgraph"))).toHaveLength(
      2,
    );
  });

  test("ignores top-level node elements", () => {
    const state = baseRenderState();
    renderTopLevelSubgraphs(state, {
      ...baseGraph(),
      elements: [{ ...baseNode(), id: "n_a" }],
    });
    expect(state.lines).toEqual([]);
  });

  test("indents at two spaces", () => {
    const state = baseRenderState();
    renderTopLevelSubgraphs(state, {
      ...baseGraph(),
      elements: [{ ...baseSubgraph(), id: "s1", kind: SUBGRAPH_KIND.If }],
    });
    expect(state.lines[0]?.startsWith("  subgraph s1")).toBe(true);
  });
});
