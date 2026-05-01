import { describe, expect, test } from "vitest";

import { DIRECTION, VISUAL_ELEMENT_TYPE } from "../../constants.js";
import type {
  SerializedIR,
  SerializedScope,
  SerializedVariable,
} from "../../ir/model.js";
import type { VisualSubgraph } from "../model.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";
import { ensureReturnUseNode } from "./ensure-return-use-node.js";
import { jsxContainer } from "./testing/jsx-container.js";
import { makeRef } from "./testing/make-ref.js";
import { makeVariable } from "./testing/make-variable.js";
import { returnContainer } from "./testing/return-container.js";
import { span } from "./testing/span.js";
import type { WriteOp } from "./write-op.js";

function makeHostSubgraph(): VisualSubgraph {
  return {
    type: VISUAL_ELEMENT_TYPE.Subgraph,
    id: "s_fn",
    kind: "function",
    line: 1,
    direction: DIRECTION.RL,
    elements: [],
  };
}

function makeCtx(
  opts: {
    variables?: readonly SerializedVariable[];
    scopes?: readonly SerializedScope[];
  } = {},
): BuilderContext {
  const variables = opts.variables ?? [];
  const scopes = opts.scopes ?? [];
  const ir = {
    version: 1,
    source: { path: "x.ts", language: "ts" },
    raw: "",
    scopes,
    variables,
    references: [],
    unusedVariableIds: [],
    diagnostics: [],
  } as const satisfies SerializedIR;
  return {
    ir,
    variableMap: new Map(variables.map((v) => [v.id, v])),
    scopeMap: new Map(scopes.map((s) => [s.id, s])),
    subgraphOwnerVar: new Map(),
    hiddenVariables: new Set(),
    writeOpsByVariable: new Map<string, WriteOp[]>(),
    writeOpsByScope: new Map<string, WriteOp[]>(),
    writeOpByRef: new Map<string, WriteOp>(),
    sortedCasesByContainer: new Map(),
  };
}

function makeState(host: VisualSubgraph, fnVarId = "fnVar"): BuildState {
  return {
    subgraphByScope: new Map(),
    functionSubgraphByFn: new Map([[fnVarId, host]]),
    returnSubgraphsByFn: new Map(),
    returnUseAdded: new Set(),
    emittedEdges: new Set(),
    edges: [],
  };
}

describe("ensureReturnUseNode", () => {
  test("returns null when no host subgraph exists", () => {
    const ctx = makeCtx();
    const state = {
      subgraphByScope: new Map(),
      functionSubgraphByFn: new Map(),
      returnSubgraphsByFn: new Map(),
      returnUseAdded: new Set(),
      emittedEdges: new Set(),
      edges: [],
    } as const satisfies BuildState;
    expect(
      ensureReturnUseNode("fnVar", makeRef({ from: "scope" }), ctx, state),
    ).toBeNull();
  });

  test("creates a return subgraph and a ReturnUse node, returning the node id", () => {
    const host = makeHostSubgraph();
    const ctx = makeCtx({ variables: [makeVariable({ id: "v", name: "x" })] });
    const state = makeState(host);
    const ref = makeRef({
      id: "r1",
      identifier: { name: "x", span: span(0, 3) },
      resolved: "v",
      returnContainer: returnContainer(0, 50, 3, 5),
    });

    const id = ensureReturnUseNode("fnVar", ref, ctx, state);

    expect(id).toBe("ret_use_r1");
    expect(host.elements).toHaveLength(1);
    const sg = host.elements[0] as VisualSubgraph;
    expect(sg.kind).toBe("return");
    expect(sg.line).toBe(3);
    expect(sg.endLine).toBe(5);
    const node = sg.elements.find((e) => e.type === VISUAL_ELEMENT_TYPE.Node);
    expect(node).toMatchObject({ kind: "ReturnUse", name: "x", line: 3 });
  });

  test("uses the identifier name when the variable is not resolved", () => {
    const host = makeHostSubgraph();
    const ctx = makeCtx();
    const state = makeState(host);
    const ref = makeRef({
      id: "r1",
      identifier: { name: "literal", span: span(0, 1) },
      resolved: null,
    });
    ensureReturnUseNode("fnVar", ref, ctx, state);
    const sg = host.elements[0] as VisualSubgraph;
    expect((sg.elements[0] as { name: string }).name).toBe("literal");
  });

  test("groups references that share the same returnContainer offsets into one subgraph", () => {
    const host = makeHostSubgraph();
    const ctx = makeCtx();
    const state = makeState(host);
    const rc = returnContainer(0, 50, 3, 5);
    const ref1 = makeRef({ id: "r1", returnContainer: rc });
    const ref2 = makeRef({ id: "r2", returnContainer: rc });

    ensureReturnUseNode("fnVar", ref1, ctx, state);
    ensureReturnUseNode("fnVar", ref2, ctx, state);

    expect(host.elements).toHaveLength(1);
    const sg = host.elements[0] as VisualSubgraph;
    expect(sg.elements.map((e) => (e as { id: string }).id)).toEqual([
      "ret_use_r1",
      "ret_use_r2",
    ]);
  });

  test("references with different returnContainer offsets create separate subgraphs", () => {
    const host = makeHostSubgraph();
    const ctx = makeCtx();
    const state = makeState(host);
    const ref1 = makeRef({ id: "r1", returnContainer: returnContainer(0, 10) });
    const ref2 = makeRef({
      id: "r2",
      returnContainer: returnContainer(20, 30),
    });

    ensureReturnUseNode("fnVar", ref1, ctx, state);
    ensureReturnUseNode("fnVar", ref2, ctx, state);

    expect(host.elements).toHaveLength(2);
  });

  test("implicit (returnContainer null) groups all under one 'implicit' bucket", () => {
    const host = makeHostSubgraph();
    const ctx = makeCtx();
    const state = makeState(host);
    ensureReturnUseNode("fnVar", makeRef({ id: "r1" }), ctx, state);
    ensureReturnUseNode("fnVar", makeRef({ id: "r2" }), ctx, state);
    expect(host.elements).toHaveLength(1);
  });

  test("does not duplicate the ReturnUse node when called twice with the same ref id", () => {
    const host = makeHostSubgraph();
    const ctx = makeCtx();
    const state = makeState(host);
    const ref = makeRef({ id: "r1" });
    ensureReturnUseNode("fnVar", ref, ctx, state);
    ensureReturnUseNode("fnVar", ref, ctx, state);
    const sg = host.elements[0] as VisualSubgraph;
    expect(sg.elements).toHaveLength(1);
  });

  test("sets isJsxElement and node.endLine when the reference has a jsxElement", () => {
    const host = makeHostSubgraph();
    const ctx = makeCtx();
    const state = makeState(host);
    const ref = makeRef({
      id: "r1",
      identifier: { name: "Foo", span: span(0, 2) },
      jsxElement: jsxContainer(0, 30, 2, 5),
    });
    ensureReturnUseNode("fnVar", ref, ctx, state);
    const sg = host.elements[0] as VisualSubgraph;
    const node = sg.elements[0] as { isJsxElement: boolean; endLine?: number };
    expect(node.isJsxElement).toBe(true);
    expect(node.endLine).toBe(5);
  });
});
