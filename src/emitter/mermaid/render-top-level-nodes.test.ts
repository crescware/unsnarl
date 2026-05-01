import { describe, expect, test } from "vitest";

import { NODE_KIND } from "../../constants.js";
import { renderTopLevelNodes } from "./render-top-level-nodes.js";
import { makeGraph } from "./testing/make-graph.js";
import { makeNode } from "./testing/make-node.js";
import { makeRenderState } from "./testing/make-render-state.js";
import { makeSubgraph } from "./testing/make-subgraph.js";

describe("renderTopLevelNodes", () => {
  test("emits non-synthetic, non-wrapped top-level nodes", () => {
    const state = makeRenderState();
    renderTopLevelNodes(
      state,
      makeGraph({
        elements: [makeNode({ id: "n_a" }), makeNode({ id: "n_b" })],
      }),
    );
    expect(state.lines.map((l) => l.trim().split(/[[(]/)[0])).toEqual([
      "n_a",
      "n_b",
    ]);
  });

  test("skips synthetic node kinds", () => {
    const state = makeRenderState();
    renderTopLevelNodes(
      state,
      makeGraph({
        elements: [
          makeNode({ id: "mod_a", kind: NODE_KIND.ModuleSource }),
          makeNode({ id: "n_b" }),
          makeNode({ id: "module_root", kind: NODE_KIND.ModuleSink }),
        ],
      }),
    );
    expect(state.lines).toHaveLength(1);
    expect(state.lines[0]?.trim().startsWith("n_b")).toBe(true);
  });

  test("skips nodes whose id is in wrappedOwnerIds", () => {
    const state = makeRenderState({ wrappedOwnerIds: new Set(["n_owner"]) });
    renderTopLevelNodes(
      state,
      makeGraph({
        elements: [makeNode({ id: "n_owner" }), makeNode({ id: "n_keep" })],
      }),
    );
    expect(state.lines).toHaveLength(1);
    expect(state.lines[0]?.trim().startsWith("n_keep")).toBe(true);
  });

  test("ignores top-level subgraph elements", () => {
    const state = makeRenderState();
    renderTopLevelNodes(
      state,
      makeGraph({ elements: [makeSubgraph({ id: "sg" })] }),
    );
    expect(state.lines).toEqual([]);
  });

  test("uses 2-space indent at the top level", () => {
    const state = makeRenderState();
    renderTopLevelNodes(
      state,
      makeGraph({ elements: [makeNode({ id: "n_a" })] }),
    );
    expect(state.lines[0]?.startsWith("  n_a")).toBe(true);
  });
});
