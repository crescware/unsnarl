import { describe, expect, test } from "vitest";

import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import { asVariableId } from "../../ir/serialized/variable-id.js";
import { LANGUAGE } from "../../language.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
import { VARIABLE_DECLARATION_KIND } from "../../serializer/variable-declaration-kind.js";
import { asFilledString } from "../../util/filled-string.js";
import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualElement } from "../visual-element.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import { buildScope } from "./build-scope.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseDef } from "./testing/make-def.js";
import { baseScope } from "./testing/make-scope.js";
import { baseVariable } from "./testing/make-variable.js";
import { baseWriteOp } from "./testing/make-write-op.js";
import { span } from "./testing/span.js";
import type { WriteOp } from "./write-op.js";

function emptyState(): BuildState {
  return {
    subgraphByScope: new Map(),
    functionSubgraphByFn: new Map(),
    returnSubgraphsByFn: new Map(),
    returnUseAdded: new Set(),
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

function makeCtx(opts: {
  scopes: readonly SerializedScope[];
  variables?: readonly SerializedVariable[];
  subgraphOwnerVar?: Map<string, string>;
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
    scopeMap: new Map(opts.scopes.map((v) => [v.id, v])),
    subgraphOwnerVar: opts.subgraphOwnerVar ?? new Map(),
    writeOpsByVariable: new Map(),
    writeOpsByScope: opts.writeOpsByScope ?? new Map(),
    writeOpByRef: new Map(),
    sortedCasesByContainer: new Map(),
  };
}

describe("buildScope", () => {
  test("plain block scope wraps its variable nodes in a 'block' subgraph", () => {
    const scope = {
      ...baseScope(),
      id: asScopeId("s"),
      variables: [asVariableId("v1")],
    };
    const v = {
      ...baseVariable(),
      id: asVariableId("v1"),
      name: asFilledString("x"),
      scope: asScopeId("s"),
    };
    const ctx = makeCtx({ scopes: [scope], variables: [v] });
    const container: { elements: VisualElement[] } = { elements: [] };

    buildScope(scope, container, ctx, emptyState());

    expect(container.elements).toHaveLength(1);
    const sg = container.elements[0] as VisualSubgraph;
    expect(sg.type).toEqual(VISUAL_ELEMENT_TYPE.Subgraph);
    expect(sg.kind).toEqual("block");
    expect(
      sg.elements.find(
        (v) => v.type === VISUAL_ELEMENT_TYPE.Node && v.id === "n_v1",
      ),
    ).toMatchObject({
      kind: NODE_KIND.LetBinding,
      name: asFilledString("x"),
    });
  });

  test("writeOps in the scope appear as WriteOp nodes with declarationKind from the owning variable", () => {
    const scope = {
      ...baseScope(),
      id: asScopeId("s"),
      variables: [asVariableId("v1")],
    };
    const v = {
      ...baseVariable(),
      id: asVariableId("v1"),
      name: asFilledString("x"),
      defs: [baseDef(VARIABLE_DECLARATION_KIND.Let)] as const,
    };
    const op = {
      ...baseWriteOp(),
      refId: "r1",
      varId: "v1",
      varName: "x",
      line: 4,
    };
    const ctx = makeCtx({
      scopes: [scope],
      variables: [v],
      writeOpsByScope: new Map([["s", [op]]]),
    });
    const container: { elements: VisualElement[] } = { elements: [] };

    buildScope(scope, container, ctx, emptyState());

    const sg = container.elements[0] as VisualSubgraph;
    const writeNode = sg.elements.find(
      (v) => v.type === VISUAL_ELEMENT_TYPE.Node && v.id === "wr_r1",
    );
    expect(writeNode).toMatchObject({
      kind: NODE_KIND.WriteReference,
      name: asFilledString("x"),
      line: 4,
      declarationKind: VARIABLE_DECLARATION_KIND.Let,
    });
  });

  test("function-owner scope wraps body in a function subgraph and registers state", () => {
    const fnScope = {
      ...baseScope(),
      id: asScopeId("fn"),
      type: SCOPE_TYPE.Function,
      variables: [asVariableId("param")],
      block: {
        type: AST_TYPE.FunctionDeclaration,
        span: span(0, 1),
        endSpan: span(10, 5),
      },
    };
    const param = {
      ...baseVariable(),
      id: asVariableId("param"),
      name: asFilledString("p"),
      defs: [
        {
          ...baseDef(VARIABLE_DECLARATION_KIND.Let),
          type: DEFINITION_TYPE.Parameter,
        },
      ] as const,
    };
    const owner = {
      ...baseVariable(),
      id: asVariableId("ownerVar"),
      name: asFilledString("myFn"),
    };
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
    expect(sg.type).toEqual("subgraph");
    expect(sg.kind).toEqual("function");
    expect(
      sg.elements.find(
        (v) => v.type === VISUAL_ELEMENT_TYPE.Node && v.id === "n_param",
      ) !== null &&
        sg.elements.find(
          (v) => v.type === VISUAL_ELEMENT_TYPE.Node && v.id === "n_param",
        ) !== undefined,
    ).toEqual(true);
    expect(state.subgraphByScope.get("fn")).toEqual(sg);
    expect(state.functionSubgraphByFn.get("ownerVar")).toEqual(sg);
  });

  test("control-kind scope (for) wraps body in a control subgraph", () => {
    const forScope = {
      ...baseScope(),
      id: asScopeId("for1"),
      type: SCOPE_TYPE.For,
      variables: [asVariableId("v")],
      block: {
        type: AST_TYPE.ForStatement,
        span: span(0, 1),
        endSpan: span(10, 3),
      },
    };
    const v = {
      ...baseVariable(),
      id: asVariableId("v"),
      name: asFilledString("i"),
    };
    const ctx = makeCtx({ scopes: [forScope], variables: [v] });
    const container: { elements: VisualElement[] } = { elements: [] };

    buildScope(forScope, container, ctx, emptyState());

    expect(container.elements).toHaveLength(1);
    const sg = container.elements[0] as VisualSubgraph;
    expect(sg.kind).toEqual("for");
    expect(
      sg.elements.find(
        (v) => v.type === VISUAL_ELEMENT_TYPE.Node && v.id === "n_v",
      ) !== null &&
        sg.elements.find(
          (v) => v.type === VISUAL_ELEMENT_TYPE.Node && v.id === "n_v",
        ) !== undefined,
    ).toEqual(true);
  });

  test("recurses into childScopes", () => {
    const inner = {
      ...baseScope(),
      id: asScopeId("inner"),
      upper: asScopeId("outer"),
      variables: [asVariableId("vIn")],
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 0,
      },
    };
    // Use Module for the outer wrapper so it stays a flat container --
    // a Block scope at the top level would now itself become a "block"
    // subgraph and obscure what this test is asserting (recursion into
    // the inner if).
    const outer = {
      ...baseScope(),
      id: asScopeId("outer"),
      type: SCOPE_TYPE.Module,
      childScopes: [asScopeId("inner")],
    };
    const vIn = {
      ...baseVariable(),
      id: asVariableId("vIn"),
      name: asFilledString("y"),
    };
    const ctx = makeCtx({ scopes: [outer, inner], variables: [vIn] });
    const container: { elements: VisualElement[] } = { elements: [] };

    buildScope(outer, container, ctx, emptyState());

    const childSg = container.elements.find(
      (v) => v.type === VISUAL_ELEMENT_TYPE.Subgraph,
    ) as VisualSubgraph;
    expect(childSg !== null && childSg !== undefined).toEqual(true);
    expect(childSg.kind).toEqual("if");
  });
});
