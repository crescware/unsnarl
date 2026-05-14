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
import { attachSwitchDiscriminantAnchor } from "./switch-discriminant-anchor.js";
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

function switchSubgraph(): VisualSubgraph {
  return {
    type: VISUAL_ELEMENT_TYPE.Subgraph,
    id: "s_switch",
    kind: SUBGRAPH_KIND.Switch,
    line: 1,
    endLine: null,
    direction: DIRECTION.RL,
    elements: [],
  };
}

describe("attachSwitchDiscriminantAnchor", () => {
  test("Switch scope: pushes a SwitchDiscriminant anchor at 'first' and registers the offset", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      id: asScopeId("switch_body"),
      type: SCOPE_TYPE.Switch,
      upper: asScopeId("scope_0"),
      block: {
        type: AST_TYPE.SwitchStatement,
        span: span(34, 3, 0),
        endSpan: span(120, 8, 1),
      },
    };
    const sg = switchSubgraph();
    const state = emptyState();

    attachSwitchDiscriminantAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors).toHaveLength(1);
    const pending = state.pendingLoopTestAnchors[0];
    expect(pending?.subgraph).toEqual(sg);
    expect(pending?.position).toEqual("first");
    expect(pending?.node).toMatchObject({
      type: VISUAL_ELEMENT_TYPE.Node,
      id: "switch_discriminant_scope_0_34",
      kind: NODE_KIND.SyntheticSwitchStatementDiscriminant,
      name: asFilledString("switch-discriminant"),
      line: 3,
      endLine: null,
      isJsxElement: false,
      unused: false,
    });
    expect(state.switchDiscriminantAnchorByOffset.get(34)).toEqual(
      "switch_discriminant_scope_0_34",
    );
  });

  test("Switch scope: re-entering the same offset does not push a duplicate", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      type: SCOPE_TYPE.Switch,
      upper: asScopeId("scope_0"),
      block: {
        type: AST_TYPE.SwitchStatement,
        span: span(34, 3, 0),
        endSpan: span(120, 8, 1),
      },
    };
    const sg = switchSubgraph();
    const state = emptyState();

    attachSwitchDiscriminantAnchor(scope, sg, state);
    attachSwitchDiscriminantAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors).toHaveLength(1);
  });

  test("Switch scope with scope.upper === null falls back to an empty parent in the id", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      type: SCOPE_TYPE.Switch,
      upper: null,
      block: {
        type: AST_TYPE.SwitchStatement,
        span: span(10, 1, 0),
        endSpan: span(80, 5, 1),
      },
    };
    const sg = switchSubgraph();
    const state = emptyState();

    attachSwitchDiscriminantAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors[0]?.node.id).toEqual(
      "switch_discriminant__10",
    );
  });

  test("Non-Switch scope (Block) is a no-op", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      type: SCOPE_TYPE.Block,
    };
    const sg = switchSubgraph();
    const state = emptyState();

    attachSwitchDiscriminantAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors).toHaveLength(0);
    expect(state.switchDiscriminantAnchorByOffset.size).toEqual(0);
  });

  test("Non-Switch scope (For) is a no-op", () => {
    const scope: SerializedScope = {
      ...baseScope(),
      type: SCOPE_TYPE.For,
    };
    const sg = switchSubgraph();
    const state = emptyState();

    attachSwitchDiscriminantAnchor(scope, sg, state);

    expect(state.pendingLoopTestAnchors).toHaveLength(0);
  });
});
