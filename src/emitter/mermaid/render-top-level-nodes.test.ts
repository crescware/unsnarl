import { describe, expect, test } from "vitest";

import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { renderTopLevelNodes } from "./render-top-level-nodes.js";
import { baseGraph } from "./testing/make-graph.js";
import { baseNode } from "./testing/make-node.js";
import { baseRenderState } from "./testing/make-render-state.js";
import { baseSubgraph } from "./testing/make-subgraph.js";

describe("renderTopLevelNodes", () => {
  test("emits non-synthetic, non-wrapped top-level nodes", () => {
    const state = baseRenderState();
    renderTopLevelNodes(state, {
      ...baseGraph(),
      elements: [
        { ...baseNode(), id: "n_a" },
        { ...baseNode(), id: "n_b" },
      ],
    });
    expect(state.lines.map((l) => l.trim().split(/[[(]/)[0])).toEqual([
      "n_a",
      "n_b",
    ]);
  });

  test("skips synthetic node kinds", () => {
    const state = baseRenderState();
    renderTopLevelNodes(state, {
      ...baseGraph(),
      elements: [
        { ...baseNode(), id: "mod_a", kind: NODE_KIND.ModuleSource },
        { ...baseNode(), id: "n_b" },
        { ...baseNode(), id: "module_root", kind: NODE_KIND.ModuleSink },
      ],
    });
    expect(state.lines).toHaveLength(1);
    expect(state.lines[0]?.trim().startsWith("n_b")).toBe(true);
  });

  test("skips nodes whose id is in wrappedOwnerIds", () => {
    const state = {
      ...baseRenderState(),
      wrappedOwnerIds: new Set(["n_owner"]),
    };
    renderTopLevelNodes(state, {
      ...baseGraph(),
      elements: [
        { ...baseNode(), id: "n_owner" },
        { ...baseNode(), id: "n_keep" },
      ],
    });
    expect(state.lines).toHaveLength(1);
    expect(state.lines[0]?.trim().startsWith("n_keep")).toBe(true);
  });

  test("ignores top-level subgraph elements", () => {
    const state = baseRenderState();
    renderTopLevelNodes(state, {
      ...baseGraph(),
      elements: [{ ...baseSubgraph(), id: "sg" }],
    });
    expect(state.lines).toEqual([]);
  });

  test("uses 2-space indent at the top level", () => {
    const state = baseRenderState();
    renderTopLevelNodes(state, {
      ...baseGraph(),
      elements: [{ ...baseNode(), id: "n_a" }],
    });
    expect(state.lines[0]?.startsWith("  n_a")).toBe(true);
  });
});
