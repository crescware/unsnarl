import { describe, expect, test } from "vitest";

import { emitNode } from "./emit-node.js";
import { makeNode } from "./testing/make-node.js";
import { makeRenderState } from "./testing/make-render-state.js";

describe("emitNode", () => {
  test("appends '<indent><id><syntax>' as a single line", () => {
    const state = makeRenderState();
    emitNode(state, makeNode({ id: "n_x", name: "x", line: 5 }), "  ");
    expect(state.lines).toHaveLength(1);
    expect(state.lines[0]?.startsWith("  n_x[")).toBe(true);
  });

  test("respects the supplied indent verbatim", () => {
    const state = makeRenderState();
    emitNode(state, makeNode({ id: "n_x" }), "      ");
    expect(state.lines[0]?.startsWith("      n_x")).toBe(true);
  });

  test("does not modify any other RenderState field", () => {
    const state = makeRenderState();
    emitNode(state, makeNode(), "  ");
    expect(state.placeholderIds).toEqual([]);
    expect(state.wrapperIds).toEqual([]);
  });
});
