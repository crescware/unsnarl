import { describe, expect, test } from "vitest";

import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { renderSyntheticNodeBlock } from "./render-synthetic-node-block.js";
import { baseGraph } from "./testing/make-graph.js";
import { baseNode } from "./testing/make-node.js";
import { baseRenderState } from "./testing/make-render-state.js";

describe("renderSyntheticNodeBlock", () => {
  test("emits only synthetic top-level nodes (ModuleSink, ModuleSource, ImportIntermediate)", () => {
    const state = baseRenderState();
    renderSyntheticNodeBlock(state, {
      ...baseGraph(),
      elements: [
        { ...baseNode(), id: "mod_a", kind: NODE_KIND.ModuleSource },
        { ...baseNode(), id: "n_a", kind: NODE_KIND.Variable },
        { ...baseNode(), id: "import_b", kind: NODE_KIND.ImportIntermediate },
        { ...baseNode(), id: "module_root", kind: NODE_KIND.ModuleSink },
      ],
    });
    expect(state.lines.map((l) => l.trim().split(/[[(]/)[0]).sort()).toEqual([
      "import_b",
      "mod_a",
      "module_root",
    ]);
  });

  test("skips non-synthetic nodes entirely", () => {
    const state = baseRenderState();
    renderSyntheticNodeBlock(state, {
      ...baseGraph(),
      elements: [{ ...baseNode(), id: "n_x", kind: NODE_KIND.Variable }],
    });
    expect(state.lines).toEqual([]);
  });

  test("preserves graph element order", () => {
    const state = baseRenderState();
    renderSyntheticNodeBlock(state, {
      ...baseGraph(),
      elements: [
        { ...baseNode(), id: "mod_first", kind: NODE_KIND.ModuleSource },
        {
          ...baseNode(),
          id: "import_second",
          kind: NODE_KIND.ImportIntermediate,
        },
      ],
    });
    expect(state.lines.map((l) => l.trim().split(/[[(]/)[0])).toEqual([
      "mod_first",
      "import_second",
    ]);
  });
});
