import { describe, expect, test } from "vitest";

import { asReferenceId } from "../../ir/serialized/reference-id.js";
import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import { asVariableId } from "../../ir/serialized/variable-id.js";
import { LANGUAGE } from "../../language.js";
import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
import { asFilledString } from "../../util/filled-string.js";
import { DIRECTION } from "../direction.js";
import { NODE_KIND } from "../node-kind.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";
import { ensureThrowUseNode } from "./ensure-throw-use-node.js";
import { throwCompletion } from "./testing/completion.js";
import { jsxContainer } from "./testing/jsx-container.js";
import { baseRef } from "./testing/make-ref.js";
import { baseVariable } from "./testing/make-variable.js";
import { span } from "./testing/span.js";
import type { WriteOp } from "./write-op.js";

function makeHostSubgraph(): VisualSubgraph {
  return {
    type: VISUAL_ELEMENT_TYPE.Subgraph,
    id: "s_fn",
    kind: SUBGRAPH_KIND.Function,
    line: 1,
    endLine: null,
    direction: DIRECTION.RL,
    ownerNodeId: "n_owner",
    ownerName: "owner",
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
    version: SERIALIZED_IR_VERSION,
    source: { path: "x.ts", language: LANGUAGE.Ts },
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
    scopeMap: new Map(scopes.map((v) => [v.id, v])),
    subgraphOwnerVar: new Map(),
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
    throwSubgraphsByFn: new Map(),
    throwUseAdded: new Set(),
    ifTestAnchorByOffset: new Map(),
    switchDiscriminantAnchorByOffset: new Map(),
    whileTestAnchorByOffset: new Map(),
    doWhileTestAnchorByOffset: new Map(),
    forTestAnchorByOffset: new Map(),
    pendingLoopTestAnchors: [],
    expressionStatementByOffset: new Map(),
    emittedEdges: new Set(),
    edges: [],
  };
}

describe("ensureThrowUseNode", () => {
  test("returns null when no host subgraph exists", () => {
    const ctx = makeCtx();
    const state = {
      subgraphByScope: new Map(),
      functionSubgraphByFn: new Map(),
      returnSubgraphsByFn: new Map(),
      returnUseAdded: new Set(),
      throwSubgraphsByFn: new Map(),
      throwUseAdded: new Set(),
      ifTestAnchorByOffset: new Map(),
      switchDiscriminantAnchorByOffset: new Map(),
      whileTestAnchorByOffset: new Map(),
      doWhileTestAnchorByOffset: new Map(),
      forTestAnchorByOffset: new Map(),
      pendingLoopTestAnchors: [],
      expressionStatementByOffset: new Map(),
      emittedEdges: new Set(),
      edges: [],
    } as const satisfies BuildState;
    expect(
      ensureThrowUseNode(
        "fnVar",
        {
          ...baseRef(),
          from: asScopeId("scope"),
          completion: throwCompletion(0, 10),
        },
        ctx,
        state,
      ),
    ).toEqual(null);
  });

  test("returns null when the reference completion is not throw", () => {
    const host = makeHostSubgraph();
    const ctx = makeCtx();
    const state = makeState(host);
    expect(
      ensureThrowUseNode(
        "fnVar",
        { ...baseRef(), id: asReferenceId("r1") },
        ctx,
        state,
      ),
    ).toEqual(null);
    expect(host.elements).toHaveLength(0);
  });

  test("creates a throw subgraph and a ThrowUse node, returning the node id", () => {
    const host = makeHostSubgraph();
    const ctx = makeCtx({
      variables: [
        {
          ...baseVariable(),
          id: asVariableId("v"),
          name: asFilledString("x"),
        },
      ],
    });
    const state = makeState(host);
    const ref = {
      ...baseRef(),
      id: asReferenceId("r1"),
      identifier: { name: asFilledString("x"), span: span(0, 3) },
      resolved: asVariableId("v"),
      completion: throwCompletion(0, 50, 3, 5),
    };

    const id = ensureThrowUseNode("fnVar", ref, ctx, state);

    expect(id).toEqual("throw_use_r1");
    expect(host.elements).toHaveLength(1);
    const sg = host.elements[0] as VisualSubgraph;
    expect(sg.kind).toEqual(SUBGRAPH_KIND.Throw);
    expect(sg.line).toEqual(3);
    expect(sg.endLine).toEqual(5);
    const node = sg.elements.find((v) => v.type === VISUAL_ELEMENT_TYPE.Node);
    expect(node).toMatchObject({
      kind: NODE_KIND.ThrowArgumentReference,
      name: asFilledString("x"),
      line: 3,
    });
  });

  test("uses the identifier name when the variable is not resolved", () => {
    const host = makeHostSubgraph();
    const ctx = makeCtx();
    const state = makeState(host);
    const ref = {
      ...baseRef(),
      id: asReferenceId("r1"),
      identifier: { name: asFilledString("literal"), span: span(0, 1) },
      resolved: null,
      completion: throwCompletion(0, 10),
    };
    ensureThrowUseNode("fnVar", ref, ctx, state);
    const sg = host.elements[0] as VisualSubgraph;
    expect((sg.elements[0] as { name: string }).name).toEqual("literal");
  });

  test("groups references that share the same throw completion offsets into one subgraph", () => {
    const host = makeHostSubgraph();
    const ctx = makeCtx();
    const state = makeState(host);
    const tc = throwCompletion(0, 50, 3, 5);
    const ref1 = { ...baseRef(), id: asReferenceId("r1"), completion: tc };
    const ref2 = { ...baseRef(), id: asReferenceId("r2"), completion: tc };

    ensureThrowUseNode("fnVar", ref1, ctx, state);
    ensureThrowUseNode("fnVar", ref2, ctx, state);

    expect(host.elements).toHaveLength(1);
    const sg = host.elements[0] as VisualSubgraph;
    expect(sg.elements.map((v) => (v as { id: string }).id)).toEqual([
      "throw_use_r1",
      "throw_use_r2",
    ]);
  });

  test("references with different throw completion offsets create separate subgraphs", () => {
    const host = makeHostSubgraph();
    const ctx = makeCtx();
    const state = makeState(host);
    const ref1 = {
      ...baseRef(),
      id: asReferenceId("r1"),
      completion: throwCompletion(0, 10),
    };
    const ref2 = {
      ...baseRef(),
      id: asReferenceId("r2"),
      completion: throwCompletion(20, 30),
    };

    ensureThrowUseNode("fnVar", ref1, ctx, state);
    ensureThrowUseNode("fnVar", ref2, ctx, state);

    expect(host.elements).toHaveLength(2);
  });

  test("does not duplicate the ThrowUse node when called twice with the same ref id", () => {
    const host = makeHostSubgraph();
    const ctx = makeCtx();
    const state = makeState(host);
    const ref = {
      ...baseRef(),
      id: asReferenceId("r1"),
      completion: throwCompletion(0, 10),
    };
    ensureThrowUseNode("fnVar", ref, ctx, state);
    ensureThrowUseNode("fnVar", ref, ctx, state);
    const sg = host.elements[0] as VisualSubgraph;
    expect(sg.elements).toHaveLength(1);
  });

  test("sets isJsxElement and node.endLine when the reference has a jsxElement", () => {
    const host = makeHostSubgraph();
    const ctx = makeCtx();
    const state = makeState(host);
    const ref = {
      ...baseRef(),
      id: asReferenceId("r1"),
      identifier: { name: asFilledString("Foo"), span: span(0, 2) },
      jsxElement: jsxContainer(0, 30, 2, 5),
      completion: throwCompletion(0, 30, 2, 5),
    };
    ensureThrowUseNode("fnVar", ref, ctx, state);
    const sg = host.elements[0] as VisualSubgraph;
    const node = sg.elements[0] as { isJsxElement: boolean; endLine?: number };
    expect(node.isJsxElement).toEqual(true);
    expect(node.endLine).toEqual(5);
  });
});
