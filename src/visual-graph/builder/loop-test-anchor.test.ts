import { describe, expect, test } from "vitest";

import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { asFilledString } from "../../util/filled-string.js";
import { DIRECTION } from "../direction.js";
import { NODE_KIND } from "../node-kind.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import type { BuildState } from "./build-state.js";
import { attachLoopTestAnchor } from "./loop-test-anchor.js";
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

function bodySubgraph(
  kind:
    | typeof SUBGRAPH_KIND.For
    | typeof SUBGRAPH_KIND.While
    | typeof SUBGRAPH_KIND.DoWhile
    | typeof SUBGRAPH_KIND.Switch,
  id = "s_body",
): VisualSubgraph {
  return {
    type: VISUAL_ELEMENT_TYPE.Subgraph,
    id,
    kind,
    line: 1,
    endLine: null,
    direction: DIRECTION.RL,
    elements: [],
  };
}

describe("attachLoopTestAnchor", () => {
  test("For scope: pushes a ForTest anchor with name 'for-test', position 'first', and registers the offset", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      id: asScopeId("for_body"),
      type: SCOPE_TYPE.For,
      upper: asScopeId("scope_0"),
      block: {
        type: AST_TYPE.BlockStatement,
        span: span(34, 3, 0),
        endSpan: span(80, 5, 1),
      },
    };
    const sg = bodySubgraph(SUBGRAPH_KIND.For);
    const state = emptyState();

    attachLoopTestAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors).toHaveLength(1);
    const pending = state.pendingLoopTestAnchors[0];
    expect(pending?.subgraph).toEqual(sg);
    expect(pending?.position).toEqual("first");
    expect(pending?.node).toMatchObject({
      type: VISUAL_ELEMENT_TYPE.Node,
      id: "for_test_scope_0_34",
      kind: NODE_KIND.LegacyForTest,
      name: asFilledString("for-test"),
      line: 3,
      endLine: null,
      isJsxElement: false,
      unused: false,
    });
    expect(state.forTestAnchorByOffset.get(34)).toEqual("for_test_scope_0_34");
  });

  test("For scope: re-entering the same offset does not push a duplicate", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      type: SCOPE_TYPE.For,
      upper: asScopeId("scope_0"),
      block: {
        type: AST_TYPE.BlockStatement,
        span: span(34, 3, 0),
        endSpan: span(80, 5, 1),
      },
    };
    const sg = bodySubgraph(SUBGRAPH_KIND.For);
    const state = emptyState();

    attachLoopTestAnchor(scope, sg, state);
    attachLoopTestAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors).toHaveLength(1);
  });

  test("For scope: scope.upper === null falls back to an empty parent in the id", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      type: SCOPE_TYPE.For,
      upper: null,
      block: {
        type: AST_TYPE.BlockStatement,
        span: span(10, 1, 0),
        endSpan: span(50, 3, 1),
      },
    };
    const sg = bodySubgraph(SUBGRAPH_KIND.For);
    const state = emptyState();

    attachLoopTestAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors[0]?.node.id).toEqual("for_test__10");
  });

  test("Block + WhileStatement.body: pushes a WhileTest anchor at 'first' and uses parentSpanOffset as the key", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      type: SCOPE_TYPE.Block,
      upper: asScopeId("scope_0"),
      block: {
        type: AST_TYPE.BlockStatement,
        span: span(33, 3, 0),
        endSpan: span(80, 5, 1),
      },
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.WhileStatement,
        key: asFilledString("body"),
        parentSpanOffset: 27,
      },
    };
    const sg = bodySubgraph(SUBGRAPH_KIND.While);
    const state = emptyState();

    attachLoopTestAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors).toHaveLength(1);
    const pending = state.pendingLoopTestAnchors[0];
    expect(pending?.subgraph).toEqual(sg);
    expect(pending?.position).toEqual("first");
    expect(pending?.node).toMatchObject({
      type: VISUAL_ELEMENT_TYPE.Node,
      id: "while_test_scope_0_27",
      kind: NODE_KIND.LegacyWhileTest,
      name: asFilledString("while-test"),
      line: 3,
    });
    expect(state.whileTestAnchorByOffset.get(27)).toEqual(
      "while_test_scope_0_27",
    );
  });

  test("Block + WhileStatement.body: same parentSpanOffset is not duplicated", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      type: SCOPE_TYPE.Block,
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.WhileStatement,
        key: asFilledString("body"),
        parentSpanOffset: 27,
      },
    };
    const sg = bodySubgraph(SUBGRAPH_KIND.While);
    const state = emptyState();

    attachLoopTestAnchor(scope, sg, state);
    attachLoopTestAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors).toHaveLength(1);
  });

  test("Block + DoWhileStatement.body: pushes a DoWhileTest anchor at 'last' with line from block.endSpan", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      type: SCOPE_TYPE.Block,
      upper: asScopeId("scope_0"),
      block: {
        type: AST_TYPE.BlockStatement,
        span: span(36, 3, 0),
        endSpan: span(80, 6, 1),
      },
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.DoWhileStatement,
        key: asFilledString("body"),
        parentSpanOffset: 33,
      },
    };
    const sg = bodySubgraph(SUBGRAPH_KIND.DoWhile);
    const state = emptyState();

    attachLoopTestAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors).toHaveLength(1);
    const pending = state.pendingLoopTestAnchors[0];
    expect(pending?.position).toEqual("last");
    expect(pending?.node).toMatchObject({
      id: "do_while_test_scope_0_33",
      kind: NODE_KIND.LegacyDoWhileTest,
      name: asFilledString("do-while-test"),
      line: 6,
    });
    expect(state.doWhileTestAnchorByOffset.get(33)).toEqual(
      "do_while_test_scope_0_33",
    );
  });

  test("Block + DoWhileStatement.body: same parentSpanOffset is not duplicated", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      type: SCOPE_TYPE.Block,
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.DoWhileStatement,
        key: asFilledString("body"),
        parentSpanOffset: 33,
      },
    };
    const sg = bodySubgraph(SUBGRAPH_KIND.DoWhile);
    const state = emptyState();

    attachLoopTestAnchor(scope, sg, state);
    attachLoopTestAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors).toHaveLength(1);
  });

  test("Non-loop block (parentType=IfStatement) is a no-op", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      type: SCOPE_TYPE.Block,
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.IfStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 10,
      },
    };
    const sg = bodySubgraph(SUBGRAPH_KIND.For);
    const state = emptyState();

    attachLoopTestAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors).toHaveLength(0);
    expect(state.whileTestAnchorByOffset.size).toEqual(0);
    expect(state.doWhileTestAnchorByOffset.size).toEqual(0);
    expect(state.forTestAnchorByOffset.size).toEqual(0);
  });

  test("Block with WhileStatement parent but key !== 'body' is a no-op", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      type: SCOPE_TYPE.Block,
      blockContext: {
        ...baseBlockContext(),
        parentType: AST_TYPE.WhileStatement,
        key: asFilledString("consequent"),
        parentSpanOffset: 27,
      },
    };
    const sg = bodySubgraph(SUBGRAPH_KIND.While);
    const state = emptyState();

    attachLoopTestAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors).toHaveLength(0);
    expect(state.whileTestAnchorByOffset.size).toEqual(0);
  });

  test("Block with blockContext === null is a no-op", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      type: SCOPE_TYPE.Block,
      blockContext: null,
    };
    const sg = bodySubgraph(SUBGRAPH_KIND.For);
    const state = emptyState();

    attachLoopTestAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors).toHaveLength(0);
  });

  test("Function scope is a no-op", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      type: SCOPE_TYPE.Function,
    };
    const sg = bodySubgraph(SUBGRAPH_KIND.For);
    const state = emptyState();

    attachLoopTestAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors).toHaveLength(0);
  });
});
