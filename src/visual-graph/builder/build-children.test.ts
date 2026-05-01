import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import { LANGUAGE } from "../../cli/language.js";
import type {
  SerializedIR,
  SerializedScope,
  SerializedVariable,
} from "../../ir/model.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
import type { VisualElement, VisualSubgraph } from "../model.js";
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
    hiddenVariables: new Set(),
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

  test("a single if branch is not wrapped in an if-else-container", () => {
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
    expect((container.elements[0] as VisualSubgraph).kind).toBe("if");
  });

  test("consecutive if siblings (consequent + alternate) wrap in an if-else-container with hasElse=true", () => {
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
    expect(sg.hasElse).toBe(true);
    expect(sg.elements.map((e) => (e as VisualSubgraph).kind)).toEqual([
      "if",
      "else",
    ]);
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

  test("two adjacent if-statements with different parentSpanOffsets are not merged", () => {
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

    expect(container.elements).toHaveLength(2);
    for (const e of container.elements) {
      expect((e as VisualSubgraph).kind).toBe("if");
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
