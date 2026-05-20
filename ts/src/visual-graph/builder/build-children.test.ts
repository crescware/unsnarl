import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import { LANGUAGE } from "../../language.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
import { asFilledString } from "../../util/filled-string.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import type { VisualElement } from "../visual-element.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import { buildChildren } from "./build-children.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";
import { baseBlockContext } from "./testing/make-block-context.js";
import { baseScope } from "./testing/make-scope.js";
import { span } from "./testing/span.js";

function emptyState(): BuildState {
  return {
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
  };
}

function makeCtx(scopes: readonly SerializedScope[], raw = ""): BuilderContext {
  const variables: /* mutable */ SerializedVariable[] = [];
  const ir = {
    version: SERIALIZED_IR_VERSION,
    source: { path: "x.ts", language: LANGUAGE.Ts },
    raw,
    scopes,
    variables,
    references: [],
    unusedVariableIds: [],
    diagnostics: [],
  } as const satisfies SerializedIR;
  return {
    ir,
    variableMap: new Map(),
    scopeMap: new Map(scopes.map((v) => [v.id, v])),
    subgraphOwnerVar: new Map(),
    writeOpsByVariable: new Map(),
    writeOpsByScope: new Map(),
    writeOpByRef: new Map(),
    sortedCasesByContainer: new Map(),
  };
}

describe("buildChildren", () => {
  test("non-branch children are built directly into the parent container", () => {
    const inner = {
      ...baseScope(),
      id: asScopeId("for1"),
      type: SCOPE_TYPE.For,
      upper: asScopeId("outer"),
    };
    const outer = {
      ...baseScope(),
      id: asScopeId("outer"),
      childScopes: [asScopeId("for1")],
    };
    const ctx = makeCtx([outer, inner]);
    const container: { elements: VisualElement[] } = { elements: [] };

    buildChildren(outer, container, ctx, emptyState());

    expect(container.elements).toHaveLength(1);
    expect((container.elements[0] as VisualSubgraph).kind).toEqual(
      SUBGRAPH_KIND.For,
    );
  });

  test("a single if branch is not wrapped in an if-else-container; the if-test anchor lives inside the consequent subgraph", () => {
    const cons = {
      ...baseScope(),
      id: asScopeId("c"),
      upper: asScopeId("outer"),
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 5,
      },
    };
    const outer = {
      ...baseScope(),
      id: asScopeId("outer"),
      childScopes: [asScopeId("c")],
    };
    const ctx = makeCtx([outer, cons]);
    const container: { elements: VisualElement[] } = { elements: [] };

    buildChildren(outer, container, ctx, emptyState());

    expect(container.elements).toHaveLength(1);
    const ifSg = container.elements[0];
    expect((ifSg as VisualSubgraph).kind).toEqual(SUBGRAPH_KIND.If);
    expect(
      container.elements.some(
        (v) =>
          v.type === "subgraph" && v.kind === SUBGRAPH_KIND.IfElseContainer,
      ),
    ).toEqual(false);
    const anchor = (ifSg as VisualSubgraph).elements[0];
    expect(anchor?.type).toEqual("node");
    if (anchor?.type === "node") {
      expect(anchor.kind).toEqual("SyntheticIfStatementTest");
    }
  });

  test("consecutive if siblings (consequent + alternate) wrap in an if-else-container with hasElse=true; the test anchor lives inside the consequent (not the container)", () => {
    const cons = {
      ...baseScope(),
      id: asScopeId("c"),
      upper: asScopeId("outer"),
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 5,
      },
      block: {
        type: AST_TYPE.BlockStatement,
        span: span(5, 1),
        endSpan: span(10, 2),
      },
    };
    const alt = {
      ...baseScope(),
      id: asScopeId("a"),
      upper: asScopeId("outer"),
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("alternate"),
        parentSpanOffset: 5,
      },
      block: {
        type: AST_TYPE.BlockStatement,
        span: span(11, 3),
        endSpan: span(20, 5),
      },
    };
    const outer = {
      ...baseScope(),
      id: asScopeId("outer"),
      childScopes: [asScopeId("c"), asScopeId("a")],
    };
    const ctx = makeCtx([outer, cons, alt], "\n".repeat(20));
    const container: { elements: VisualElement[] } = { elements: [] };

    buildChildren(outer, container, ctx, emptyState());

    expect(container.elements).toHaveLength(1);
    const sg = container.elements[0] as VisualSubgraph;
    expect(sg.kind).toEqual(SUBGRAPH_KIND.IfElseContainer);
    if (sg.kind !== SUBGRAPH_KIND.IfElseContainer) {
      throw new Error("expected if-else-container");
    }
    expect(sg.hasElse).toEqual(true);
    // The container holds only the branch subgraphs; the test anchor
    // lives inside the `if` (consequent) branch, and the `else`
    // (alternate) carries no test of its own.
    expect(sg.elements.map((v) => v.type)).toEqual(["subgraph", "subgraph"]);
    expect(sg.elements.map((v) => v.kind)).toEqual([
      SUBGRAPH_KIND.If,
      SUBGRAPH_KIND.Else,
    ]);
    const ifSg = sg.elements[0];
    if (ifSg?.type !== "subgraph" || ifSg.kind !== SUBGRAPH_KIND.If) {
      throw new Error("expected if subgraph at index 0");
    }
    const anchor = ifSg.elements[0];
    expect(anchor?.type).toEqual("node");
    if (anchor?.type === "node") {
      expect(anchor.kind).toEqual("SyntheticIfStatementTest");
    }
    const elseSg = sg.elements[1];
    if (elseSg?.type !== "subgraph" || elseSg.kind !== SUBGRAPH_KIND.Else) {
      throw new Error("expected else subgraph at index 1");
    }
    expect(
      elseSg.elements.every(
        (v) => !(v.type === "node" && v.kind === "SyntheticIfStatementTest"),
      ),
    ).toEqual(true);
  });

  test("if-else-container endLine is the maximum endLine among grouped branches", () => {
    const cons = {
      ...baseScope(),
      id: asScopeId("c"),
      upper: asScopeId("outer"),
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 5,
      },
      block: {
        type: AST_TYPE.BlockStatement,
        span: span(5, 1),
        endSpan: span(10, 2),
      },
    };
    const alt = {
      ...baseScope(),
      id: asScopeId("a"),
      upper: asScopeId("outer"),
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("alternate"),
        parentSpanOffset: 5,
      },
      block: {
        type: AST_TYPE.BlockStatement,
        span: span(11, 3),
        endSpan: span(20, 7),
      },
    };
    const outer = {
      ...baseScope(),
      id: asScopeId("outer"),
      childScopes: [asScopeId("c"), asScopeId("a")],
    };
    const raw = "\n".repeat(20);
    const ctx = makeCtx([outer, cons, alt], raw);
    const container: { elements: VisualElement[] } = { elements: [] };

    buildChildren(outer, container, ctx, emptyState());

    const sg = container.elements[0] as VisualSubgraph;
    expect(sg.endLine).toEqual(7);
  });

  test("two adjacent if-statements with different parentSpanOffsets are not merged; each gets its own anchor", () => {
    const ifA = {
      ...baseScope(),
      id: asScopeId("ifA"),
      upper: asScopeId("outer"),
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 5,
      },
    };
    const ifB = {
      ...baseScope(),
      id: asScopeId("ifB"),
      upper: asScopeId("outer"),
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 30,
      },
    };
    const outer = {
      ...baseScope(),
      id: asScopeId("outer"),
      childScopes: [asScopeId("ifA"), asScopeId("ifB")],
    };
    const ctx = makeCtx([outer, ifA, ifB]);
    const container: { elements: VisualElement[] } = { elements: [] };

    buildChildren(outer, container, ctx, emptyState());

    // Two lone-ifs at the parent-scope level: each contributes one if
    // subgraph that hosts its own test anchor inside. No merge
    // container in between.
    expect(container.elements).toHaveLength(2);
    expect(container.elements.map((v) => v.type)).toEqual([
      "subgraph",
      "subgraph",
    ]);
    expect(container.elements.map((v) => v.kind)).toEqual([
      SUBGRAPH_KIND.If,
      SUBGRAPH_KIND.If,
    ]);
    for (const sg of container.elements) {
      if (sg.type !== "subgraph") {
        throw new Error("expected subgraph");
      }
      const anchor = sg.elements[0];
      expect(anchor?.type).toEqual("node");
      if (anchor?.type === "node") {
        expect(anchor.kind).toEqual("SyntheticIfStatementTest");
      }
    }
  });

  test("missing child id is skipped silently", () => {
    const outer = {
      ...baseScope(),
      id: asScopeId("outer"),
      childScopes: [asScopeId("missing")],
    };
    const ctx = makeCtx([outer]);
    const container: { elements: VisualElement[] } = { elements: [] };

    buildChildren(outer, container, ctx, emptyState());

    expect(container.elements).toEqual([]);
  });
});
