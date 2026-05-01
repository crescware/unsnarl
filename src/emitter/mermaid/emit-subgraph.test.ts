import { describe, expect, test } from "vitest";

import { emitSubgraph } from "./emit-subgraph.js";
import { makeNode } from "./testing/make-node.js";
import { makeRenderState } from "./testing/make-render-state.js";
import { makeSubgraph } from "./testing/make-subgraph.js";

describe("emitSubgraph", () => {
  test("function with a known ownerNodeId is wrapped in a wrap_<id> subgraph", () => {
    const owner = makeNode({ id: "n_owner", kind: "FunctionName", name: "f" });
    const state = makeRenderState({
      nodeMap: new Map([[owner.id, owner]]),
    });
    const sg = makeSubgraph({
      id: "s_fn",
      kind: "function",
      ownerNodeId: "n_owner",
      ownerName: "f",
    });
    emitSubgraph(state, sg, "  ");
    expect(state.lines[0]).toBe('  subgraph wrap_s_fn[" "]');
    expect(state.lines[1]).toBe("    direction TB");
    expect(state.wrapperIds).toEqual(["wrap_s_fn"]);
    // Wrapper closes after the inner subgraph closes.
    expect(state.lines.at(-1)).toBe("  end");
  });

  test("function without an ownerNode in the map falls back to plain emission", () => {
    const state = makeRenderState();
    const sg = makeSubgraph({
      id: "s_fn",
      kind: "function",
      ownerNodeId: "n_missing",
    });
    emitSubgraph(state, sg, "  ");
    expect(state.wrapperIds).toEqual([]);
    expect(state.lines.some((l) => l.startsWith('  subgraph s_fn["'))).toBe(
      true,
    );
  });

  test("non-function subgraphs are emitted plainly without a wrapper", () => {
    const state = makeRenderState();
    emitSubgraph(state, makeSubgraph({ id: "s_if", kind: "if" }), "  ");
    expect(state.wrapperIds).toEqual([]);
    expect(state.lines.some((l) => l.startsWith('  subgraph s_if["'))).toBe(
      true,
    );
  });

  test("the owner node line appears INSIDE the wrapper, before the function body subgraph", () => {
    const owner = makeNode({ id: "n_owner", kind: "FunctionName", name: "f" });
    const state = makeRenderState({
      nodeMap: new Map([[owner.id, owner]]),
    });
    const sg = makeSubgraph({
      id: "s_fn",
      kind: "function",
      ownerNodeId: "n_owner",
    });
    emitSubgraph(state, sg, "  ");
    const ownerIdx = state.lines.findIndex((l) => l.includes("n_owner"));
    const innerIdx = state.lines.findIndex((l) => l.includes("subgraph s_fn"));
    expect(ownerIdx).toBeGreaterThan(0); // not the wrapper open line
    expect(ownerIdx).toBeLessThan(innerIdx);
  });
});
