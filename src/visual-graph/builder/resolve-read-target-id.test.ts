import { beforeEach, describe, expect, test } from "vitest";

import { asReferenceId } from "../../ir/serialized/reference-id.js";
import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import { LANGUAGE } from "../../language.js";
import { SERIALIZED_IR_VERSION } from "../../serializer/serialized-ir-version.js";
import { DIRECTION } from "../direction.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";
import { MODULE_ROOT_ID } from "./module-root-id.js";
import { resolveReadTargetId } from "./resolve-read-target-id.js";
import { baseRef } from "./testing/make-ref.js";
import { returnContainer } from "./testing/return-container.js";
import { throwContainer } from "./testing/throw-container.js";
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

function makeContext(
  options: {
    variables?: readonly SerializedVariable[];
    scopes?: readonly SerializedScope[];
  } = {},
): BuilderContext {
  const variables = options.variables ?? [];
  const scopes = options.scopes ?? [];
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
    variableMap: new Map(variables.map((variable) => [variable.id, variable])),
    scopeMap: new Map(scopes.map((scope) => [scope.id, scope])),
    subgraphOwnerVar: new Map(),
    writeOpsByVariable: new Map<string, WriteOp[]>(),
    writeOpsByScope: new Map<string, WriteOp[]>(),
    writeOpByRef: new Map<string, WriteOp>(),
    sortedCasesByContainer: new Map(),
  };
}

function makeStateWithHost(
  host: VisualSubgraph,
  fnVarId = "fnVar",
): BuildState {
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

function makeEmptyState(): BuildState {
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

function makeReadReference(
  returnContainerValue: ReturnType<typeof returnContainer> | null,
  throwContainerValue: ReturnType<typeof throwContainer> | null,
) {
  return {
    ...baseRef(),
    id: asReferenceId("r1"),
    from: asScopeId("scope"),
    returnContainer: returnContainerValue,
    throwContainer: throwContainerValue,
  };
}

describe("resolveReadTargetId", () => {
  describe("when exprStmtId is non-null", () => {
    test("returns exprStmtId verbatim", () => {
      const host = makeHostSubgraph();
      const context = makeContext();
      const state = makeStateWithHost(host);
      const reference = makeReadReference(returnContainer(0, 10), null);

      const result = resolveReadTargetId(
        "expr_42",
        "fnVar",
        reference,
        context,
        state,
      );

      expect(result).toEqual("expr_42");
    });

    test("returns exprStmtId verbatim even when enclosingFnVarId is null", () => {
      const context = makeContext();
      const state = makeEmptyState();
      const reference = makeReadReference(null, null);

      const result = resolveReadTargetId(
        "expr_42",
        null,
        reference,
        context,
        state,
      );

      expect(result).toEqual("expr_42");
    });

    describe("does not produce return-side effects even when returnContainer is set", () => {
      let host: VisualSubgraph;
      let state: BuildState;

      beforeEach(() => {
        host = makeHostSubgraph();
        const context = makeContext();
        state = makeStateWithHost(host);
        const reference = makeReadReference(returnContainer(0, 10), null);
        resolveReadTargetId("expr_42", "fnVar", reference, context, state);
      });

      test("leaves returnSubgraphsByFn empty", () => {
        expect(state.returnSubgraphsByFn.size).toEqual(0);
      });

      test("appends nothing to the host", () => {
        expect(host.elements).toEqual([]);
      });
    });

    describe("does not produce throw-side effects even when throwContainer is set", () => {
      let host: VisualSubgraph;
      let state: BuildState;

      beforeEach(() => {
        host = makeHostSubgraph();
        const context = makeContext();
        state = makeStateWithHost(host);
        const reference = makeReadReference(null, throwContainer(0, 10));
        resolveReadTargetId("expr_42", "fnVar", reference, context, state);
      });

      test("leaves throwSubgraphsByFn empty", () => {
        expect(state.throwSubgraphsByFn.size).toEqual(0);
      });

      test("appends nothing to the host", () => {
        expect(host.elements).toEqual([]);
      });
    });
  });

  describe("when exprStmtId is null and enclosingFnVarId is null", () => {
    test("falls back to module root", () => {
      const context = makeContext();
      const state = makeEmptyState();
      const reference = makeReadReference(null, null);

      const result = resolveReadTargetId(null, null, reference, context, state);

      expect(result).toEqual(MODULE_ROOT_ID);
    });
  });

  describe("when exprStmtId is null and enclosingFnVarId is non-null", () => {
    describe("with a returnContainer and a registered host subgraph", () => {
      let host: VisualSubgraph;
      let state: BuildState;
      let result: ReturnType<typeof resolveReadTargetId>;

      beforeEach(() => {
        host = makeHostSubgraph();
        const context = makeContext();
        state = makeStateWithHost(host);
        const reference = makeReadReference(returnContainer(0, 50, 3, 5), null);
        result = resolveReadTargetId(null, "fnVar", reference, context, state);
      });

      test("returns the return-use id of the reference", () => {
        expect(result).toEqual("ret_use_r1");
      });

      test("registers a return subgraph under the enclosing function", () => {
        expect(state.returnSubgraphsByFn.get("fnVar")?.size).toEqual(1);
      });

      test("appends the return subgraph to the host", () => {
        expect(host.elements).toHaveLength(1);
      });
    });

    describe("with a throwContainer and a registered host subgraph", () => {
      let host: VisualSubgraph;
      let state: BuildState;
      let result: ReturnType<typeof resolveReadTargetId>;

      beforeEach(() => {
        host = makeHostSubgraph();
        const context = makeContext();
        state = makeStateWithHost(host);
        const reference = makeReadReference(null, throwContainer(0, 50, 3, 5));
        result = resolveReadTargetId(null, "fnVar", reference, context, state);
      });

      test("returns the throw-use id of the reference", () => {
        expect(result).toEqual("throw_use_r1");
      });

      test("registers a throw subgraph under the enclosing function", () => {
        expect(state.throwSubgraphsByFn.get("fnVar")?.size).toEqual(1);
      });

      test("appends the throw subgraph to the host", () => {
        expect(host.elements).toHaveLength(1);
      });
    });

    describe("when both returnContainer and throwContainer are set", () => {
      // The analyzer is expected to enforce mutual exclusivity, but the
      // resolver pins the precedence here so a future regression does
      // not silently flip the routing.
      let host: VisualSubgraph;
      let state: BuildState;
      let result: ReturnType<typeof resolveReadTargetId>;

      beforeEach(() => {
        host = makeHostSubgraph();
        const context = makeContext();
        state = makeStateWithHost(host);
        const reference = makeReadReference(
          returnContainer(0, 50, 3, 5),
          throwContainer(0, 50, 3, 5),
        );
        result = resolveReadTargetId(null, "fnVar", reference, context, state);
      });

      test("returnContainer wins and the result is the return-use id", () => {
        expect(result).toEqual("ret_use_r1");
      });

      test("no throw subgraph is registered", () => {
        expect(state.throwSubgraphsByFn.size).toEqual(0);
      });
    });

    test("falls back to module root when both containers are null", () => {
      // Today this routes to module root rather than throwing,
      // so callers can still emit an edge.
      const host = makeHostSubgraph();
      const context = makeContext();
      const state = makeStateWithHost(host);
      const reference = makeReadReference(null, null);

      const result = resolveReadTargetId(
        null,
        "fnVar",
        reference,
        context,
        state,
      );

      expect(result).toEqual(MODULE_ROOT_ID);
    });

    test("falls back to module root when the function body collapsed and exposes no host subgraph (return)", () => {
      // Internally this is the path where ensureReturnUseNode returns null.
      const context = makeContext();
      const state = makeEmptyState();
      const reference = makeReadReference(returnContainer(0, 10), null);

      const result = resolveReadTargetId(
        null,
        "fnVar",
        reference,
        context,
        state,
      );

      expect(result).toEqual(MODULE_ROOT_ID);
    });

    test("falls back to module root when the function body collapsed and exposes no host subgraph (throw)", () => {
      // Internally this is the path where ensureThrowUseNode returns null.
      const context = makeContext();
      const state = makeEmptyState();
      const reference = makeReadReference(null, throwContainer(0, 10));

      const result = resolveReadTargetId(
        null,
        "fnVar",
        reference,
        context,
        state,
      );

      expect(result).toEqual(MODULE_ROOT_ID);
    });
  });

  describe("structurally impossible inputs (defensive)", () => {
    // ReturnStatement / ThrowStatement both require a function body, so
    // a top-level reference whose ancestor chain holds either container
    // should not occur in practice. These tests pin the safe
    // degradation regardless.

    test("falls back to module root for a top-level reference whose returnContainer is non-null", () => {
      const context = makeContext();
      const state = makeEmptyState();
      const reference = makeReadReference(returnContainer(0, 10), null);

      const result = resolveReadTargetId(null, null, reference, context, state);

      expect(result).toEqual(MODULE_ROOT_ID);
    });

    test("falls back to module root for a top-level reference whose throwContainer is non-null", () => {
      const context = makeContext();
      const state = makeEmptyState();
      const reference = makeReadReference(null, throwContainer(0, 10));

      const result = resolveReadTargetId(null, null, reference, context, state);

      expect(result).toEqual(MODULE_ROOT_ID);
    });
  });

  describe("idempotency on repeated calls with the same reference", () => {
    let host: VisualSubgraph;
    let first: ReturnType<typeof resolveReadTargetId>;
    let second: ReturnType<typeof resolveReadTargetId>;

    beforeEach(() => {
      host = makeHostSubgraph();
      const context = makeContext();
      const state = makeStateWithHost(host);
      const reference = makeReadReference(returnContainer(0, 50, 3, 5), null);
      first = resolveReadTargetId(null, "fnVar", reference, context, state);
      second = resolveReadTargetId(null, "fnVar", reference, context, state);
    });

    test("returns equal results on both calls", () => {
      expect(first).toEqual(second);
    });

    test("does not add a second subgraph to the host", () => {
      expect(host.elements).toHaveLength(1);
    });

    test("does not add a second node inside the return subgraph", () => {
      const subgraph = host.elements[0] as VisualSubgraph;
      expect(subgraph.elements).toHaveLength(1);
    });
  });
});
