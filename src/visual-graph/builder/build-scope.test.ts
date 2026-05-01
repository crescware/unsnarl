import { describe, expect, test } from "vitest";

import {
  DEFINITION_TYPE,
  LANGUAGE,
  NODE_KIND,
  SCOPE_TYPE,
  SERIALIZED_IR_VERSION,
  VARIABLE_DECLARATION_KIND,
  VISUAL_ELEMENT_TYPE,
} from "../../constants.js";
import type {
  SerializedIR,
  SerializedScope,
  SerializedVariable,
} from "../../ir/model.js";
import type { VisualElement, VisualSubgraph } from "../model.js";
import { buildScope } from "./build-scope.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";
import { makeBlockContext } from "./testing/make-block-context.js";
import { makeDef } from "./testing/make-def.js";
import { makeScope } from "./testing/make-scope.js";
import { makeVariable } from "./testing/make-variable.js";
import { makeWriteOp } from "./testing/make-write-op.js";
import { span } from "./testing/span.js";
import type { WriteOp } from "./write-op.js";

function emptyState(): BuildState {
  return {
    subgraphByScope: new Map(),
    functionSubgraphByFn: new Map(),
    returnSubgraphsByFn: new Map(),
    returnUseAdded: new Set(),
    emittedEdges: new Set(),
    edges: [],
  };
}

function makeCtx(opts: {
  scopes: readonly SerializedScope[];
  variables?: readonly SerializedVariable[];
  subgraphOwnerVar?: Map<string, string>;
  hiddenVariables?: Set<string>;
  writeOpsByScope?: Map<string, WriteOp[]>;
}): BuilderContext {
  const variables = opts.variables ?? [];
  const ir = {
    version: SERIALIZED_IR_VERSION,
    source: { path: "x.ts", language: LANGUAGE.Ts },
    raw: "",
    scopes: opts.scopes,
    variables,
    references: [],
    unusedVariableIds: [],
    diagnostics: [],
  } as const satisfies SerializedIR;
  return {
    ir,
    variableMap: new Map(variables.map((v) => [v.id, v])),
    scopeMap: new Map(opts.scopes.map((s) => [s.id, s])),
    subgraphOwnerVar: opts.subgraphOwnerVar ?? new Map(),
    hiddenVariables: opts.hiddenVariables ?? new Set(),
    writeOpsByVariable: new Map(),
    writeOpsByScope: opts.writeOpsByScope ?? new Map(),
    writeOpByRef: new Map(),
    sortedCasesByContainer: new Map(),
  };
}

describe("buildScope", () => {
  test("plain block scope appends variable nodes directly to the container (no subgraph)", () => {
    const scope = makeScope({ id: "s", variables: ["v1"] });
    const v = makeVariable({ id: "v1", name: "x", scope: "s" });
    const ctx = makeCtx({ scopes: [scope], variables: [v] });
    const container: { elements: VisualElement[] } = { elements: [] };

    buildScope(scope, container, ctx, emptyState());

    expect(container.elements).toHaveLength(1);
    expect(container.elements[0]).toMatchObject({
      type: VISUAL_ELEMENT_TYPE.Node,
      id: "n_v1",
      kind: NODE_KIND.Variable,
      name: "x",
    });
  });

  test("hidden variables are skipped", () => {
    const scope = makeScope({ id: "s", variables: ["hidden", "v"] });
    const hidden = makeVariable({ id: "hidden", name: "h" });
    const v = makeVariable({ id: "v", name: "x" });
    const ctx = makeCtx({
      scopes: [scope],
      variables: [hidden, v],
      hiddenVariables: new Set(["hidden"]),
    });
    const container: { elements: VisualElement[] } = { elements: [] };

    buildScope(scope, container, ctx, emptyState());

    expect(container.elements).toHaveLength(1);
    expect((container.elements[0] as { id: string }).id).toBe("n_v");
  });

  test("writeOps in the scope appear as WriteOp nodes with declarationKind from the owning variable", () => {
    const scope = makeScope({ id: "s", variables: ["v1"] });
    const v = makeVariable({
      id: "v1",
      name: "x",
      defs: [makeDef({ declarationKind: VARIABLE_DECLARATION_KIND.Let })],
    });
    const op = makeWriteOp({ refId: "r1", varId: "v1", varName: "x", line: 4 });
    const ctx = makeCtx({
      scopes: [scope],
      variables: [v],
      writeOpsByScope: new Map([["s", [op]]]),
    });
    const container: { elements: VisualElement[] } = { elements: [] };

    buildScope(scope, container, ctx, emptyState());

    const writeNode = container.elements.find(
      (e) => e.type === VISUAL_ELEMENT_TYPE.Node && e.id === "wr_r1",
    );
    expect(writeNode).toMatchObject({
      kind: NODE_KIND.WriteOp,
      name: "x",
      line: 4,
      declarationKind: VARIABLE_DECLARATION_KIND.Let,
    });
  });

  test("function-owner scope wraps body in a function subgraph and registers state", () => {
    const fnScope = makeScope({
      id: "fn",
      type: SCOPE_TYPE.Function,
      variables: ["param"],
      block: { type: "Function", span: span(0, 1), endSpan: span(10, 5) },
    });
    const param = makeVariable({
      id: "param",
      name: "p",
      defs: [makeDef({ type: DEFINITION_TYPE.Parameter })],
    });
    const owner = makeVariable({ id: "ownerVar", name: "myFn" });
    const ctx = makeCtx({
      scopes: [fnScope],
      variables: [param, owner],
      subgraphOwnerVar: new Map([["fn", "ownerVar"]]),
    });
    const state = emptyState();
    const container: { elements: VisualElement[] } = { elements: [] };

    buildScope(fnScope, container, ctx, state);

    expect(container.elements).toHaveLength(1);
    const sg = container.elements[0] as VisualSubgraph;
    expect(sg.type).toBe("subgraph");
    expect(sg.kind).toBe("function");
    expect(
      sg.elements.find(
        (e) => e.type === VISUAL_ELEMENT_TYPE.Node && e.id === "n_param",
      ),
    ).toBeDefined();
    expect(state.subgraphByScope.get("fn")).toBe(sg);
    expect(state.functionSubgraphByFn.get("ownerVar")).toBe(sg);
  });

  test("control-kind scope (for) wraps body in a control subgraph", () => {
    const forScope = makeScope({
      id: "for1",
      type: SCOPE_TYPE.For,
      variables: ["v"],
      block: { type: "ForStatement", span: span(0, 1), endSpan: span(10, 3) },
    });
    const v = makeVariable({ id: "v", name: "i" });
    const ctx = makeCtx({ scopes: [forScope], variables: [v] });
    const container: { elements: VisualElement[] } = { elements: [] };

    buildScope(forScope, container, ctx, emptyState());

    expect(container.elements).toHaveLength(1);
    const sg = container.elements[0] as VisualSubgraph;
    expect(sg.kind).toBe("for");
    expect(
      sg.elements.find(
        (e) => e.type === VISUAL_ELEMENT_TYPE.Node && e.id === "n_v",
      ),
    ).toBeDefined();
  });

  test("recurses into childScopes", () => {
    const inner = makeScope({
      id: "inner",
      upper: "outer",
      variables: ["vIn"],
      blockContext: makeBlockContext("IfStatement", "consequent", 0),
    });
    const outer = makeScope({ id: "outer", childScopes: ["inner"] });
    const vIn = makeVariable({ id: "vIn", name: "y" });
    const ctx = makeCtx({ scopes: [outer, inner], variables: [vIn] });
    const container: { elements: VisualElement[] } = { elements: [] };

    buildScope(outer, container, ctx, emptyState());

    const childSg = container.elements.find(
      (e) => e.type === VISUAL_ELEMENT_TYPE.Subgraph,
    ) as VisualSubgraph;
    expect(childSg).toBeDefined();
    expect(childSg.kind).toBe("if");
  });
});
