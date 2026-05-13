import { describe, expect, test } from "vitest";

import { NODE_KIND } from "../../visual-graph/node-kind.js";
import { SUBGRAPH_KIND } from "../../visual-graph/subgraph-kind.js";
import { emitSubgraph } from "./emit-subgraph.js";
import { baseNode } from "./testing/make-node.js";
import { baseRenderState } from "./testing/make-render-state.js";
import { baseSubgraph } from "./testing/make-subgraph.js";

describe("emitSubgraph", () => {
  test("function with a known ownerNodeId is wrapped in a wrap_<id> subgraph", () => {
    const owner = {
      ...baseNode(),
      id: "n_owner",
      kind: NODE_KIND.LegacyFunctionName,
      name: "f",
    };
    const state = {
      ...baseRenderState(),
      nodeMap: new Map([[owner.id, owner]]),
    };
    const sg = {
      ...baseSubgraph(),
      id: "s_fn",
      kind: SUBGRAPH_KIND.Function,
      ownerNodeId: "n_owner",
      ownerName: "f",
    };
    emitSubgraph(state, sg, "  ", 1);
    expect(state.lines[0]).toEqual('  subgraph wrap_s_fn[" "]');
    expect(state.lines[1]).toEqual("    direction TB");
    // Wrapper closes after the inner subgraph closes.
    expect(state.lines.at(-1)).toEqual("  end");
  });

  test("function without an ownerNode in the map falls back to plain emission", () => {
    const state = baseRenderState();
    const sg = {
      ...baseSubgraph(),
      id: "s_fn",
      kind: SUBGRAPH_KIND.Function,
      ownerNodeId: "n_missing",
    };
    emitSubgraph(state, sg, "  ", 1);
    expect(state.lines.some((v) => v.startsWith('  subgraph s_fn["'))).toEqual(
      true,
    );
  });

  test("non-function subgraphs are emitted plainly without a wrapper", () => {
    const state = baseRenderState();
    emitSubgraph(
      state,
      { ...baseSubgraph(), id: "s_if", kind: SUBGRAPH_KIND.If },
      "  ",
      1,
    );
    expect(state.lines.some((v) => v.startsWith('  subgraph s_if["'))).toEqual(
      true,
    );
  });

  test("function wrapper sits one palette slot above its body subgraph", () => {
    const owner = {
      ...baseNode(),
      id: "n_owner",
      kind: NODE_KIND.LegacyFunctionName,
      name: "f",
    };
    const state = {
      ...baseRenderState(),
      nodeMap: new Map([[owner.id, owner]]),
    };
    const sg = {
      ...baseSubgraph(),
      id: "s_fn",
      kind: SUBGRAPH_KIND.Function,
      ownerNodeId: "n_owner",
    };
    emitSubgraph(state, sg, "  ", 2);
    // Slot 1 corresponds to depth 2 (wrap), slot 2 to depth 3 (body).
    // The wrapper and body must land on different slots so they render
    // as distinct brightness levels.
    expect(state.nestClassMap.get(1)).toEqual(["wrap_s_fn"]);
    expect(state.nestClassMap.get(2)).toEqual(["s_fn"]);
  });

  test("the owner node line appears INSIDE the wrapper, before the function body subgraph", () => {
    const owner = {
      ...baseNode(),
      id: "n_owner",
      kind: NODE_KIND.LegacyFunctionName,
      name: "f",
    };
    const state = {
      ...baseRenderState(),
      nodeMap: new Map([[owner.id, owner]]),
    };
    const sg = {
      ...baseSubgraph(),
      id: "s_fn",
      kind: SUBGRAPH_KIND.Function,
      ownerNodeId: "n_owner",
    };
    emitSubgraph(state, sg, "  ", 1);
    const ownerIdx = state.lines.findIndex((v) => v.includes("n_owner"));
    const innerIdx = state.lines.findIndex((v) => v.includes("subgraph s_fn"));
    expect(ownerIdx > 0).toEqual(true); // not the wrapper open line
    expect(ownerIdx < innerIdx).toEqual(true);
  });

  test("non-function subgraphs occupy a palette slot at the supplied depth", () => {
    const state = baseRenderState();
    emitSubgraph(
      state,
      { ...baseSubgraph(), id: "s_if", kind: SUBGRAPH_KIND.If },
      "  ",
      3,
    );
    expect(state.nestClassMap.get(2)).toEqual(["s_if"]);
  });
});
