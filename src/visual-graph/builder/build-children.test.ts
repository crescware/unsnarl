import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import { LANGUAGE } from "../../cli/language.js";
import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
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
    ifTestAnchorByOffset: new Map(),
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
    scopeMap: new Map(scopes.map((s) => [s.id, s])),
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
      id: "for1",
      type: SCOPE_TYPE.For,
      upper: "outer",
    };
    const outer = { ...baseScope(), id: "outer", childScopes: ["for1"] };
    const ctx = makeCtx([outer, inner]);
    const container: { elements: VisualElement[] } = { elements: [] };

    buildChildren(outer, container, ctx, emptyState());

    expect(container.elements).toHaveLength(1);
    expect((container.elements[0] as VisualSubgraph).kind).toBe("for");
  });

  test("a single if branch is not wrapped in an if-else-container; the if-test anchor lives inside the consequent subgraph", () => {
    const cons = {
      ...baseScope(),
      id: "c",
      upper: "outer",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "consequent",
        parentSpanOffset: 5,
      },
    };
    const outer = { ...baseScope(), id: "outer", childScopes: ["c"] };
    const ctx = makeCtx([outer, cons]);
    const container: { elements: VisualElement[] } = { elements: [] };

    buildChildren(outer, container, ctx, emptyState());

    expect(container.elements).toHaveLength(1);
    const ifSg = container.elements[0];
    expect((ifSg as VisualSubgraph).kind).toBe("if");
    expect(
      container.elements.some(
        (e) => e.type === "subgraph" && e.kind === "if-else-container",
      ),
    ).toBe(false);
    const anchor = (ifSg as VisualSubgraph).elements[0];
    expect(anchor?.type).toBe("node");
    if (anchor?.type === "node") {
      expect(anchor.kind).toBe("IfTest");
    }
  });

  test("consecutive if siblings (consequent + alternate) wrap in an if-else-container with hasElse=true; the test anchor lives inside the consequent (not the container)", () => {
    const cons = {
      ...baseScope(),
      id: "c",
      upper: "outer",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "consequent",
        parentSpanOffset: 5,
      },
      block: { type: "Block", span: span(5, 1), endSpan: span(10, 2) },
    };
    const alt = {
      ...baseScope(),
      id: "a",
      upper: "outer",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "alternate",
        parentSpanOffset: 5,
      },
      block: { type: "Block", span: span(11, 3), endSpan: span(20, 5) },
    };
    const outer = { ...baseScope(), id: "outer", childScopes: ["c", "a"] };
    const ctx = makeCtx([outer, cons, alt], "\n".repeat(20));
    const container: { elements: VisualElement[] } = { elements: [] };

    buildChildren(outer, container, ctx, emptyState());

    expect(container.elements).toHaveLength(1);
    const sg = container.elements[0] as VisualSubgraph;
    expect(sg.kind).toBe("if-else-container");
    if (sg.kind !== "if-else-container") {
      throw new Error("expected if-else-container");
    }
    expect(sg.hasElse).toBe(true);
    // The container holds only the branch subgraphs; the test anchor
    // lives inside the `if` (consequent) branch, and the `else`
    // (alternate) carries no test of its own.
    expect(sg.elements.map((e) => e.type)).toEqual(["subgraph", "subgraph"]);
    expect(sg.elements.map((e) => e.kind)).toEqual(["if", "else"]);
    const ifSg = sg.elements[0];
    if (ifSg?.type !== "subgraph" || ifSg.kind !== "if") {
      throw new Error("expected if subgraph at index 0");
    }
    const anchor = ifSg.elements[0];
    expect(anchor?.type).toBe("node");
    if (anchor?.type === "node") {
      expect(anchor.kind).toBe("IfTest");
    }
    const elseSg = sg.elements[1];
    if (elseSg?.type !== "subgraph" || elseSg.kind !== "else") {
      throw new Error("expected else subgraph at index 1");
    }
    expect(
      elseSg.elements.every((e) => !(e.type === "node" && e.kind === "IfTest")),
    ).toBe(true);
  });

  test("if-else-container endLine is the maximum endLine among grouped branches", () => {
    const cons = {
      ...baseScope(),
      id: "c",
      upper: "outer",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "consequent",
        parentSpanOffset: 5,
      },
      block: { type: "Block", span: span(5, 1), endSpan: span(10, 2) },
    };
    const alt = {
      ...baseScope(),
      id: "a",
      upper: "outer",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "alternate",
        parentSpanOffset: 5,
      },
      block: { type: "Block", span: span(11, 3), endSpan: span(20, 7) },
    };
    const outer = { ...baseScope(), id: "outer", childScopes: ["c", "a"] };
    const raw = "\n".repeat(20);
    const ctx = makeCtx([outer, cons, alt], raw);
    const container: { elements: VisualElement[] } = { elements: [] };

    buildChildren(outer, container, ctx, emptyState());

    const sg = container.elements[0] as VisualSubgraph;
    expect(sg.endLine).toBe(7);
  });

  test("two adjacent if-statements with different parentSpanOffsets are not merged; each gets its own anchor", () => {
    const ifA = {
      ...baseScope(),
      id: "ifA",
      upper: "outer",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "consequent",
        parentSpanOffset: 5,
      },
    };
    const ifB = {
      ...baseScope(),
      id: "ifB",
      upper: "outer",
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: "consequent",
        parentSpanOffset: 30,
      },
    };
    const outer = { ...baseScope(), id: "outer", childScopes: ["ifA", "ifB"] };
    const ctx = makeCtx([outer, ifA, ifB]);
    const container: { elements: VisualElement[] } = { elements: [] };

    buildChildren(outer, container, ctx, emptyState());

    // Two lone-ifs at the parent-scope level: each contributes one if
    // subgraph that hosts its own test anchor inside. No merge
    // container in between.
    expect(container.elements).toHaveLength(2);
    expect(container.elements.map((e) => e.type)).toEqual([
      "subgraph",
      "subgraph",
    ]);
    expect(container.elements.map((e) => e.kind)).toEqual(["if", "if"]);
    for (const sg of container.elements) {
      if (sg.type !== "subgraph") {
        throw new Error("expected subgraph");
      }
      const anchor = sg.elements[0];
      expect(anchor?.type).toBe("node");
      if (anchor?.type === "node") {
        expect(anchor.kind).toBe("IfTest");
      }
    }
  });

  test("missing child id is skipped silently", () => {
    const outer = { ...baseScope(), id: "outer", childScopes: ["missing"] };
    const ctx = makeCtx([outer]);
    const container: { elements: VisualElement[] } = { elements: [] };

    buildChildren(outer, container, ctx, emptyState());

    expect(container.elements).toEqual([]);
  });
});
