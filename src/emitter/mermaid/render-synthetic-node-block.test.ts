import { describe, expect, test } from "vitest";

import { renderSyntheticNodeBlock } from "./render-synthetic-node-block.js";
import { makeGraph } from "./testing/make-graph.js";
import { makeNode } from "./testing/make-node.js";
import { makeRenderState } from "./testing/make-render-state.js";

describe("renderSyntheticNodeBlock", () => {
  test("emits only synthetic top-level nodes (ModuleSink, ModuleSource, ImportIntermediate)", () => {
    const state = makeRenderState();
    renderSyntheticNodeBlock(
      state,
      makeGraph({
        elements: [
          makeNode({ id: "mod_a", kind: "ModuleSource" }),
          makeNode({ id: "n_a", kind: "Variable" }),
          makeNode({ id: "import_b", kind: "ImportIntermediate" }),
          makeNode({ id: "module_root", kind: "ModuleSink" }),
        ],
      }),
    );
    expect(state.lines.map((l) => l.trim().split(/[[(]/)[0]).sort()).toEqual([
      "import_b",
      "mod_a",
      "module_root",
    ]);
  });

  test("skips non-synthetic nodes entirely", () => {
    const state = makeRenderState();
    renderSyntheticNodeBlock(
      state,
      makeGraph({ elements: [makeNode({ id: "n_x", kind: "Variable" })] }),
    );
    expect(state.lines).toEqual([]);
  });

  test("preserves graph element order", () => {
    const state = makeRenderState();
    renderSyntheticNodeBlock(
      state,
      makeGraph({
        elements: [
          makeNode({ id: "mod_first", kind: "ModuleSource" }),
          makeNode({ id: "import_second", kind: "ImportIntermediate" }),
        ],
      }),
    );
    expect(state.lines.map((l) => l.trim().split(/[[(]/)[0])).toEqual([
      "mod_first",
      "import_second",
    ]);
  });
});
