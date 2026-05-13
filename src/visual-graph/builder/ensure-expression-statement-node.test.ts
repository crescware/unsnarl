import { describe, expect, test } from "vitest";

import type { Span } from "../../ir/primitive/span.js";
import { asReferenceId } from "../../ir/serialized/reference-id.js";
import type { SerializedHeadExpression } from "../../ir/serialized/serialized-expression-statement-head.js";
import type { SerializedReference } from "../../ir/serialized/serialized-reference.js";
import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualElement } from "../visual-element.js";
import type { VisualNode } from "../visual-node.js";
import type { BuildState } from "./build-state.js";
import { ensureExpressionStatementNode } from "./ensure-expression-statement-node.js";
import { baseRef } from "./testing/make-ref.js";
import { span } from "./testing/span.js";

function spanAt(offset: number, line: number): Span {
  return { offset, line, column: 0 };
}

function makeState(): BuildState {
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

function refWithHead(opts: {
  refId: string;
  startOffset: number;
  endOffset: number;
  startLine: number;
  endLine?: number;
  head: SerializedHeadExpression;
}): SerializedReference {
  return {
    ...baseRef(),
    id: asReferenceId(opts.refId),
    identifier: { name: "x", span: span() },
    expressionStatementContainer: {
      startSpan: spanAt(opts.startOffset, opts.startLine),
      endSpan: spanAt(opts.endOffset, opts.endLine ?? opts.startLine),
      head: opts.head,
    },
  };
}

const CONSOLE_LOG_HEAD: SerializedHeadExpression = {
  kind: "call",
  callee: {
    kind: "member",
    object: { kind: "identifier", name: "console" },
    property: "log",
  },
};

describe("ensureExpressionStatementNode", () => {
  test("returns null when the ref carries no ExpressionStatement container", () => {
    const elements: VisualElement[] = [];
    const state = makeState();
    expect(
      ensureExpressionStatementNode(baseRef(), "", elements, state),
    ).toEqual(null);
    expect(elements).toHaveLength(0);
  });

  test("renders a call/member/identifier mini-AST to `<receiver>.<property>()`", () => {
    const ref = refWithHead({
      refId: "r1",
      startOffset: 0,
      endOffset: 15,
      startLine: 7,
      head: CONSOLE_LOG_HEAD,
    });
    const elements: VisualElement[] = [];
    const state = makeState();
    const id = ensureExpressionStatementNode(
      ref,
      "console.log(a);",
      elements,
      state,
    );
    expect(id).toEqual("expr_stmt_0");
    expect(elements).toHaveLength(1);
    const node = elements[0] as VisualNode;
    expect(node).toMatchObject({
      type: VISUAL_ELEMENT_TYPE.Node,
      id: "expr_stmt_0",
      kind: NODE_KIND.LegacyExpressionStatement,
      name: "console.log()",
      line: 7,
    });
  });

  test("renders a bare identifier head with no parens", () => {
    const ref = refWithHead({
      refId: "r1",
      startOffset: 0,
      endOffset: 2,
      startLine: 2,
      head: { kind: "identifier", name: "a" },
    });
    const elements: VisualElement[] = [];
    const state = makeState();
    ensureExpressionStatementNode(ref, "a;", elements, state);
    expect((elements[0] as VisualNode).name).toEqual("a");
  });

  test("renders an awaited chain to `await <chain>()`", () => {
    const ref = refWithHead({
      refId: "r1",
      startOffset: 0,
      endOffset: 50,
      startLine: 1,
      endLine: 5,
      head: {
        kind: "await",
        argument: {
          kind: "call",
          callee: {
            kind: "member",
            object: {
              kind: "call",
              callee: {
                kind: "member",
                object: { kind: "identifier", name: "Promise" },
                property: "resolve",
              },
            },
            property: "then",
          },
        },
      },
    });
    const elements: VisualElement[] = [];
    const state = makeState();
    ensureExpressionStatementNode(ref, "", elements, state);
    expect((elements[0] as VisualNode).name).toEqual(
      "await Promise.resolve().then()",
    );
  });

  test("slices the original source for a `raw` head", () => {
    const ref = refWithHead({
      refId: "r1",
      startOffset: 0,
      endOffset: 6,
      startLine: 3,
      head: { kind: "raw", startSpan: spanAt(0, 3), endSpan: spanAt(5, 3) },
    });
    const elements: VisualElement[] = [];
    const state = makeState();
    ensureExpressionStatementNode(ref, "x = 1;", elements, state);
    expect((elements[0] as VisualNode).name).toEqual("x = 1");
  });

  test("sets endLine when the statement spans multiple lines so the renderer shows L<start>-<end>", () => {
    const ref = refWithHead({
      refId: "r1",
      startOffset: 0,
      endOffset: 20,
      startLine: 1,
      endLine: 3,
      head: CONSOLE_LOG_HEAD,
    });
    const elements: VisualElement[] = [];
    const state = makeState();
    ensureExpressionStatementNode(
      ref,
      "console.log(\n  a,\n);",
      elements,
      state,
    );
    expect((elements[0] as VisualNode).line).toEqual(1);
    expect((elements[0] as VisualNode).endLine).toEqual(3);
  });

  test("returns the cached id and does not re-append when called twice for refs in the same statement", () => {
    const refA = refWithHead({
      refId: "r1",
      startOffset: 0,
      endOffset: 15,
      startLine: 7,
      head: CONSOLE_LOG_HEAD,
    });
    const refB = refWithHead({
      refId: "r2",
      startOffset: 0,
      endOffset: 15,
      startLine: 7,
      head: CONSOLE_LOG_HEAD,
    });
    const elements: VisualElement[] = [];
    const state = makeState();
    const raw = "console.log(a);";
    const idA = ensureExpressionStatementNode(refA, raw, elements, state);
    const idB = ensureExpressionStatementNode(refB, raw, elements, state);
    expect(idA).toEqual(idB);
    expect(elements).toHaveLength(1);
  });
});
