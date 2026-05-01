import { describe, expect, test } from "vitest";

import { renderTopLevelSubgraphs } from "./render-top-level-subgraphs.js";
import { makeGraph } from "./testing/make-graph.js";
import { makeNode } from "./testing/make-node.js";
import { makeRenderState } from "./testing/make-render-state.js";
import { makeSubgraph } from "./testing/make-subgraph.js";

describe("renderTopLevelSubgraphs", () => {
  test("delegates to emitSubgraph for each top-level subgraph", () => {
    const state = makeRenderState();
    renderTopLevelSubgraphs(
      state,
      makeGraph({
        elements: [
          makeSubgraph({ id: "s1", kind: "if" }),
          makeSubgraph({ id: "s2", kind: "else" }),
        ],
      }),
    );
    expect(state.lines.filter((l) => l.startsWith("  subgraph"))).toHaveLength(
      2,
    );
  });

  test("ignores top-level node elements", () => {
    const state = makeRenderState();
    renderTopLevelSubgraphs(
      state,
      makeGraph({ elements: [makeNode({ id: "n_a" })] }),
    );
    expect(state.lines).toEqual([]);
  });

  test("indents at two spaces", () => {
    const state = makeRenderState();
    renderTopLevelSubgraphs(
      state,
      makeGraph({ elements: [makeSubgraph({ id: "s1", kind: "if" })] }),
    );
    expect(state.lines[0]?.startsWith("  subgraph s1")).toBe(true);
  });
});
