import { describe, expect, test } from "vitest";

import type { Span } from "../../ir/primitive/span.js";
import type { SerializedReference } from "../../ir/serialized/serialized-reference.js";
import type { VisualElement, VisualNode } from "../model.js";
import { NODE_KIND } from "../node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
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
    expressionStatementByOffset: new Map(),
    emittedEdges: new Set(),
    edges: [],
  };
}

function callRef(opts: {
  refId: string;
  startOffset: number;
  endOffset: number;
  headStartOffset: number;
  headEndOffset: number;
  line: number;
}): SerializedReference {
  return {
    ...baseRef(),
    id: opts.refId,
    identifier: { name: "x", span: span() },
    expressionStatementContainer: {
      startSpan: spanAt(opts.startOffset, opts.line),
      endSpan: spanAt(opts.endOffset, opts.line),
      headStartSpan: spanAt(opts.headStartOffset, opts.line),
      headEndSpan: spanAt(opts.headEndOffset, opts.line),
      isCall: true,
    },
  };
}

describe("ensureExpressionStatementNode", () => {
  test("returns null when the ref carries no ExpressionStatement container", () => {
    const elements: VisualElement[] = [];
    const state = makeState();
    expect(
      ensureExpressionStatementNode(baseRef(), "", elements, state),
    ).toBeNull();
    expect(elements).toHaveLength(0);
  });

  test("appends an ExpressionStatement node with name `<callee>()` and the statement's start line", () => {
    const raw = "console.log(a);";
    const ref = callRef({
      refId: "r1",
      startOffset: 0,
      endOffset: 15,
      headStartOffset: 0,
      headEndOffset: 11,
      line: 7,
    });
    const elements: VisualElement[] = [];
    const state = makeState();
    const id = ensureExpressionStatementNode(ref, raw, elements, state);
    expect(id).toBe("expr_stmt_0");
    expect(elements).toHaveLength(1);
    const node = elements[0] as VisualNode;
    expect(node).toMatchObject({
      type: VISUAL_ELEMENT_TYPE.Node,
      id: "expr_stmt_0",
      kind: NODE_KIND.ExpressionStatement,
      name: "console.log()",
      line: 7,
    });
  });

  test("appends `<expr>` without parens when the expression is not a call", () => {
    const raw = "a;";
    const ref: SerializedReference = {
      ...baseRef(),
      expressionStatementContainer: {
        startSpan: spanAt(0, 2),
        endSpan: spanAt(2, 2),
        headStartSpan: spanAt(0, 2),
        headEndSpan: spanAt(1, 2),
        isCall: false,
      },
    };
    const elements: VisualElement[] = [];
    const state = makeState();
    ensureExpressionStatementNode(ref, raw, elements, state);
    expect((elements[0] as VisualNode).name).toBe("a");
  });

  test("sets endLine when the statement spans multiple lines so the renderer shows L<start>-<end>", () => {
    const raw = "console.log(\n  a,\n);";
    const ref: SerializedReference = {
      ...baseRef(),
      expressionStatementContainer: {
        startSpan: spanAt(0, 1),
        endSpan: spanAt(20, 3),
        headStartSpan: spanAt(0, 1),
        headEndSpan: spanAt(11, 1),
        isCall: true,
      },
    };
    const elements: VisualElement[] = [];
    const state = makeState();
    ensureExpressionStatementNode(ref, raw, elements, state);
    expect((elements[0] as VisualNode).line).toBe(1);
    expect((elements[0] as VisualNode).endLine).toBe(3);
  });

  test("returns the cached id and does not re-append when called twice for refs in the same statement", () => {
    const raw = "console.log(a);";
    const refA = callRef({
      refId: "r1",
      startOffset: 0,
      endOffset: 15,
      headStartOffset: 0,
      headEndOffset: 11,
      line: 7,
    });
    const refB = callRef({
      refId: "r2",
      startOffset: 0,
      endOffset: 15,
      headStartOffset: 0,
      headEndOffset: 11,
      line: 7,
    });
    const elements: VisualElement[] = [];
    const state = makeState();
    const idA = ensureExpressionStatementNode(refA, raw, elements, state);
    const idB = ensureExpressionStatementNode(refB, raw, elements, state);
    expect(idA).toBe(idB);
    expect(elements).toHaveLength(1);
  });
});
